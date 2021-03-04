use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::{HashMap, VecDeque};
use bimap::BiHashMap;
use std::io::{self, Read, Write};
use std::time::Duration;

use crate::component::{LineIO,Lobby};

use hecs::*;

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

  pub fn update(&mut self, world: &mut World) -> io::Result<()> {

    // prepare output and ask for write events
    for (entity, io) in world.query_mut::<&mut LineIO>() {
      if io.output.len() > 0 {
        if let Some(token) = self.token_entity.get_by_right(&entity) {
          if let Some(session) = self.sessions.get_mut(token) {
            session.queue.extend(
              io.output.drain(0..).map(|s| (s.to_owned() + &"\r\n".to_string()).as_bytes().to_vec())
            );
            self.poll.registry().reregister(&mut session.conn, *token, Interest::READABLE.add(Interest::WRITABLE))?;
          }
        }
      }
    }

    let mut events = Events::with_capacity(128);

    self.poll.poll(&mut events, Some(Duration::from_millis(1000)))?;

    for event in events.iter() {
      match event.token() {

        // listening socket
        Token(0) => match self.listener.accept() {

          // no more connections, go for next event
          Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,

          // propagate any other error
          Err(e) => return Err(e),

          // new connection
          Ok((mut conn, addr)) => {
            self.token = Token(self.token.0 + 1);
            self.poll.registry().register(&mut conn, self.token, Interest::READABLE.add(Interest::WRITABLE))?;
            self.sessions.insert(self.token, Session {
              conn:  conn,
              queue: vec!(),
            });

            println!("connected [{:?}]: {}", self.token, addr);

            // start them in the lobby
            let entity = world.spawn((
              LineIO {
                input:  VecDeque::new(),
                output: VecDeque::new(),
              },
              Lobby::Start,
            ));
            
            self.token_entity.insert(self.token, entity);
          }
        },

        // activity on user conn
        token => {
          if event.is_writable() {
            // XXX all kinda shit. mostly, what do errors even mean?
            if let Some(session) = self.sessions.get_mut(&token) {
              for data in session.queue.drain(0..) {
                match session.conn.write(&data) {
                  Ok(n) if n < data.len() => return Err(io::ErrorKind::WriteZero.into()),
                  Ok(_) => self.poll.registry().reregister(&mut session.conn, token, Interest::READABLE)?,
                  Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted => {},
                  Err(e) => return Err(e),
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
                  println!("disconnected: [{:?}] {}", token, session.conn.peer_addr()?);
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
                        if let Ok(mut io) = world.get_mut::<LineIO>(*entity) {
                          io.input.push_back(line.trim().to_string());
                        }
                      }
                    }
                  }

                },

                Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
              }
            }
          }
        }
      };
    }

    Ok(())
  }
}
