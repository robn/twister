use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};
use std::collections::HashMap;
use bimap::BiHashMap;
use uuid::Uuid;
use std::io::{self, Read};

use crate::message::ServerEvent;

pub struct Server {
  poll:        Poll,
  listener:    TcpListener,
  token:       Token,
  connections: HashMap<Token,TcpStream>,
  sid_map:     BiHashMap<Token,Uuid>
}

impl Server {
  pub fn new() -> io::Result<Server> {
    let addr = "127.0.0.1:3000".parse().unwrap();

    let mut server = Server {
      poll:        Poll::new()?,
      listener:    TcpListener::bind(addr)?,
      token:       Token(1),
      connections: HashMap::new(),
      sid_map:     BiHashMap::new(),
    };

    server.poll.registry().register(&mut server.listener, Token(0), Interest::READABLE)?;

    Ok(server)
  }

  pub fn pump(&mut self) -> io::Result<Vec<ServerEvent>> {
    let mut events: Vec<ServerEvent> = vec!();

    let mut io_events = Events::with_capacity(128);

    self.poll.poll(&mut io_events, None)?;

    for event in io_events.iter() {
      match event.token() {

        // listening socket
        Token(0) => match self.listener.accept() {

          // no more connections, go for next event
          Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,

          // propogate any other error
          Err(e) => return Err(e),

          // new connection
          Ok((mut conn, addr)) => {
            println!("accepted connection: {}", addr);

            self.token = Token(self.token.0 + 1);
            self.poll.registry().register(&mut conn, self.token, Interest::READABLE.add(Interest::WRITABLE))?;
            self.connections.insert(self.token, conn);

            let sid = Uuid::new_v4();
            self.sid_map.insert(self.token, sid);

            events.push(ServerEvent::Connect(sid));
          }
        },

        // activity on user conn
        token => {
          if event.is_readable() {
            if let Some(conn) = self.connections.get_mut(&token) {
              let mut buf = vec![0; 4096];
              match conn.read(&mut buf) {
                Ok(0) => {
                  // disconnected
                  println!("disconnected: {}", conn.peer_addr()?);
                  self.connections.remove(&token);

                  let (_, sid) = self.sid_map.remove_by_left(&token).unwrap();
                  events.push(ServerEvent::Disconnect(sid));
                },

                Ok(n) => {
                  let sid = self.sid_map.get_by_left(&token).unwrap();
                  events.push(ServerEvent::Read(*sid, buf[..n].to_vec()));
                },

                Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::Interrupted => {},
                Err(e) => return Err(e),
              }
            }
          }
        }
      };
    }

    Ok(events)
  }
}
