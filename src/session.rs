use uuid::Uuid;

use crate::message::WorldAction;

#[derive(Debug)]
enum Command {
  Empty,
  Hello,
  Unknown(String),
}

#[derive(Default)]
pub struct Session {
  id:       Uuid,
  commands: Vec<Command>,
}

impl Session {
  pub fn new(sid: Uuid) -> Self {
    Session {
      id: sid,
      ..Default::default()
    }
  }

  pub fn input(&mut self, line: String) {
    let mut iter = line.split_whitespace();

    let command = match iter.next() {
      None => Command::Empty,
      Some(word) => {
        let args: Vec<String> = iter.map(|s| s.to_string()).collect();
        match word.to_lowercase().as_ref() {
          "hello" => Command::Hello,
          _       => Command::Unknown(word.to_string()),
        }
      },
    };

    println!("session {} command: {:?}", self.id, command);

    self.commands.push(command);
  }

  pub fn process_actions(&mut self) -> Vec<WorldAction> {
    /*
    let mut world_actions = vec!();

    println!("session {} processing", self.id);

    for action in self.commands.iter() {
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
    */
    vec!()
  }
}
