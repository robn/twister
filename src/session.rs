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

    println!("session {} input command: {:?}", self.id, command);

    self.commands.push(command);
  }

  pub fn pump(&mut self) -> Vec<WorldAction> {
    let mut world_actions = vec!();

    println!("session {} pump", self.id);

    for command in &self.commands {
      println!("session {} run command: {:?}", self.id, command);

      match command {
        Command::Empty         => {},
        Command::Unknown(word) => {},
        Command::Hello         => {},
      }

      /*
        SessionAction::Input(text) => {
          println!("session {} input: {}", self.id, text);

          world_actions.push(WorldAction::Wall(text.to_string()));
        },
      }
      */
    }

    self.commands.truncate(0);

    world_actions
  }
}
