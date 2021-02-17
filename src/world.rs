use uuid::Uuid;
use std::collections::HashMap;
use std::error::Error;

use crate::session::Session;
use crate::server::{Server, ServerEvent};

#[derive(Debug)]
pub enum WorldAction {
  Wall(String),
  Tell(Uuid, String, String),
}

#[derive(Default)]
pub struct World {
  sessions: HashMap<Uuid,Session>,
}

impl World {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn run(&mut self, mut server: Server) -> Result<(), Box<dyn Error>> {
    loop {

      // pump the server for events from connected clients
      for event in server.pump()? {
        match event {
          ServerEvent::Connect(sid) => {
            let s = Session::new(sid);
            self.sessions.insert(sid, s);
            println!("managing session: {}", sid);
          },
          ServerEvent::Disconnect(sid) => {
            self.sessions.remove(&sid);
            println!("dropped session: {}", sid);
          },
          ServerEvent::Read(sid, line) => {
            if let Some(s) = self.sessions.get_mut(&sid) {
              s.input(line);
            }
          }
        }
      }

      // run all the sessions, and collect up the changes they want to make to the world
      let world_actions: Vec<WorldAction> = self.sessions.iter_mut().flat_map(|(_,s)| s.pump()).collect();

      // apply actions to the world
      for action in world_actions {
        match action {
          WorldAction::Wall(text) => {
            println!("wall: {}", text);
            self.sessions.iter_mut().for_each(|(_,s)| s.output(text.to_string()));
          },
          WorldAction::Tell(ref sid, ref to, text) => {
            println!("tell: {} {} {}", sid, to, text);
            match self.sessions.get_mut(&Uuid::parse_str(to).unwrap()) {
              Some(s) => s.output(format!("<{}> {}", sid, text)),
              None    => self.sessions.get_mut(sid).unwrap().output(format!("tell: {} isn't online right now.", to)),
            }
          },
        }
      }

      // format and queue session output into the server
      for (_,s) in self.sessions.iter_mut() {
        s.update_server(&mut server)?;
      }
    }
  }

}
