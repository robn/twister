use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::{HashMap, VecDeque};
use bimap::BiHashMap;
use std::io::{self, Read, Write};
use std::time::Duration;

use crate::global::Global;
use crate::system::System;
use crate::action::Action;

use hecs::*;
use crate::component::{LineIO,Connection};

struct Session {
  conn:  TcpStream,
  queue: Vec<Vec<u8>>,
}

pub struct Server {
  poll:         Poll,
  listener:     TcpListener,
  token:        Token,
  sessions:     HashMap<Token,Session>,
  token_entity: BiHashMap<Token,Entity>
}

impl Server {
  pub fn new() -> io::Result<Server> {
    let addr = "127.0.0.1:3000".parse().unwrap();

    let mut server = Server {
      poll:         Poll::new()?,
      listener:     TcpListener::bind(addr)?,
      token:        Token(1),
      sessions:     HashMap::new(),
      token_entity: BiHashMap::new(),
    };

    server.poll.registry().register(&mut server.listener, Token(0), Interest::READABLE)?;

    Ok(server)
  }
}

impl System for Server {
  fn update(&mut self, g: &mut Global) -> Vec<Action> {
    let actions = vec!();

    // prepare output and ask for write events
    for (entity, io) in g.world.query_mut::<&mut LineIO>() {
      if io.output.len() > 0 {
        if let Some(token) = self.token_entity.get_by_right(&entity) {
          if let Some(session) = self.sessions.get_mut(token) {
            session.queue.extend(
              io.output.drain(0..).map(|s| (s.to_owned() + &"\r\n".to_string()).as_bytes().to_vec())
            );
            if let Err(e) = self.poll.registry().reregister(&mut session.conn, *token, Interest::READABLE.add(Interest::WRITABLE)) {
              println!("reregister token {:?} (r+w) failed: {}", token, e);
              // XXX straight up disconnect probably
            }
          }
        }
      }
    }

    let mut events = Events::with_capacity(128);

    if let Err(e) = self.poll.poll(&mut events, Some(Duration::from_millis(1000))) {
      println!("poll failed: {}", e); // XXX do something
      return actions;
    }

    for event in events.iter() {
      match event.token() {

        // listening socket
        Token(0) => match self.listener.accept() {

          // no more connections, go for next event
          Err(e) if e.kind() == io::ErrorKind::WouldBlock => (),

          // any other error
          Err(e) => {
            println!("accept failed: {}", e); // XXX do something
          },

          // new connection
          Ok((mut conn, addr)) => {
            self.token = Token(self.token.0 + 1);
            match self.poll.registry().register(&mut conn, self.token, Interest::READABLE.add(Interest::WRITABLE)) {
              Err(e) => {
                println!("register token {:?} failed: {}", self.token, e);
                // XXX straight up disconnect probably
              },
              _ => {
                self.sessions.insert(self.token, Session {
                  conn:  conn,
                  queue: vec!(),
                });

                println!("connected [{:?}]: {}", self.token, addr);

                // start them in the lobby
                let entity = g.world.spawn((
                  LineIO {
                    input:  VecDeque::new(),
                    output: VecDeque::new(),
                  },
                  Connection::Connected,
                ));

                self.token_entity.insert(self.token, entity);
              },
            }
          }
        },

        // activity on user conn
        token => {
          if event.is_writable() {
            // XXX all kinda shit. mostly, what do errors even mean?
            if let Some(session) = self.sessions.get_mut(&token) {
              for data in session.queue.drain(0..) {
                match session.conn.write(&data) {
                  Ok(n) if n < data.len() => {
                    println!("short write on token {:?}", token);
                    // XXX not really error, just queue the remainder or something
                  },
                  Ok(_) => {
                    if let Err(e) = self.poll.registry().reregister(&mut session.conn, token, Interest::READABLE) {
                      println!("reregister token {:?} (r) failed: {}", token, e);
                      // XXX straight up disconnect probably
                    }
                  },
                  Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted => (),
                  Err(e) => {
                    println!("write on token {:?} failed: {}", token, e);
                    // XXX straight up disconnect probably
                  },
                }
              }
            }
          }

          if event.is_readable() {
            if let Some(session) = self.sessions.get_mut(&token) {
              let mut buf = vec![0; 4096];
              match session.conn.read(&mut buf) {
                Ok(0) => {
                  // disconnected
                  let peer_addr: String = match session.conn.peer_addr() {
                    Ok(a)  => a.to_string(),
                    Err(e) => format!("[unknown ({})]", e).to_string(),
                  };
                  println!("disconnected: [{:?}] {}", token, peer_addr);
                  self.sessions.remove(&token);

                  if let Some((_, entity)) = self.token_entity.remove_by_left(&token) {
                    // XXX world despawn
                  }
                },

                Ok(n) => {
                  // convert buffer to utf8, if that works, pull out all the lines
                  // XXX non-utf8?
                  // XXX incomplete lines?
                  if let Ok(str) = std::str::from_utf8(&buf[..n]) {
                    for line in str.lines() {
                      if let Some(entity) = self.token_entity.get_by_left(&token) {
                        if let Ok(mut io) = g.world.get_mut::<LineIO>(*entity) {
                          io.input.push_back(line.trim().to_string());
                        }
                      }
                    }
                  }

                },

                Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted => {},
                Err(e) => {
                  println!("read on token {:?} failed: {}", token, e);
                  // XXX straight up disconnect probably
                },
              }
            }
          }
        }
      };
    }

    actions
  }

  fn apply(&mut self, g: &mut Global, actions: &Vec<Action>) {
  }
}
