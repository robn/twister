use uuid::Uuid;

use crate::message::{SessionAction, WorldAction};

#[derive(Default)]
pub struct Session {
  id:      Uuid,
  actions: Vec<SessionAction>,
}

impl Session {
  pub fn new(sid: Uuid) -> Self {
    Session {
      id: sid,
      ..Default::default()
    }
  }

  pub fn queue_action(&mut self, action: SessionAction) {
    println!("session {} queued action: {:?}", self.id, action);
    self.actions.push(action);
  }

  pub fn process_actions(&mut self) -> Vec<WorldAction> {
    let mut world_actions = vec!();

    println!("session {} processing", self.id);

    for action in self.actions.iter() {
      println!("session {} process action: {:?}", self.id, action);

      match action {
        SessionAction::Input(text) => {
          println!("session {} input: {}", self.id, text);

          world_actions.push(WorldAction::Wall(text.to_string()));
        },
      }
    }

    self.actions.truncate(0);

    world_actions
  }
}
