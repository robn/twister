use uuid::Uuid;
use std::collections::HashMap;
use std::error::Error;

use crate::session::Session;
use crate::server::Server;
use crate::message::{WorldAction, ServerAction, ServerEvent};

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

      // apply actions to the world and collect up actions to take on connected clients
      let server_actions: Vec<ServerAction> = world_actions.iter().flat_map(|action|
        match action {
          WorldAction::Wall(text) => {
            println!("wall: {}", text);
            self.sessions.keys().map(|id| ServerAction::Write(*id, text.to_string())).collect::<Vec<ServerAction>>()
          },
        }
      ).collect();

      // apply actions to connected clients
      server.process_actions(server_actions)?;
    }
  }

}
