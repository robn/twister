use uuid::Uuid;

use crate::message::SessionAction;

#[derive(Default)]
pub struct Session {
  id:      Uuid,
  actions: Vec<SessionAction>,
}

impl Session {
  pub fn new() -> Self {
    Session {
      id: Uuid::new_v4(),
      ..Default::default()
    }
  }

  pub fn id(&self) -> Uuid {
    self.id
  }

  pub fn queue_action(&mut self, action: SessionAction) {
    println!("session {} queued action: {:?}", self.id, action);
    self.actions.push(action);
  }

  pub fn process_actions(&mut self) {
    println!("session {} processing", self.id);

    for action in self.actions.iter() {
      println!("session {} process action: {:?}", self.id, action);

      match action {
        SessionAction::Input(text) => {
          println!("session {} input: {}", self.id, text);
        },
      }
    }

    self.actions.truncate(0);
  }
}
