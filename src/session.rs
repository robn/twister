use uuid::Uuid;
use std::io;
use friendly_zoo::Zoo;

use crate::world::WorldAction;
use crate::server::Server;

#[derive(Debug)]
enum Command {
  Empty,
  Hello,
  Tell(String, String),
  Error(String),
  Unknown(String),
}

#[derive(Default)]
pub struct Session {
  id:       Uuid,
  name:     String,
  commands: Vec<Command>,
  output:   Vec<String>,
}

impl Session {
  pub fn new(sid: Uuid) -> Self {
    let mut s = Session {
      id: sid,
      name: Zoo::default().generate(),
      ..Default::default()
    };
    s.output(format!("Your name: {}", s.name));
    s
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn input(&mut self, line: String) {
    let mut iter = line.split_whitespace();

    let command = match iter.next() {
      None => Command::Empty,
      Some(word) => {
        let args: Vec<String> = iter.map(|s| s.to_string()).collect();
        match word.to_lowercase().as_ref() {
          "hello" => Command::Hello,
          "tell"  => match args.len() {
              n if n < 2 => Command::Error(format!("try: tell [who] [what...]")),
              _             => Command::Tell(args[0].to_string(), args[1..].join(" ")),
          },
          _       => Command::Unknown(word.to_string()),
        }
      },
    };

    println!("session {} input command: {:?}", self.id, command);

    self.commands.push(command);
  }

  pub fn output(&mut self, line: String) {
    self.output.push(line);
  }

  pub fn pump(&mut self) -> Vec<WorldAction> {
    let mut world_actions = vec!();

    println!("session {} pump", self.id);

    for command in &self.commands {
      println!("session {} run command: {:?}", self.id, command);

      match command {
        Command::Empty          => {},
        Command::Error(text)    => self.output.push(text.to_string()),
        Command::Unknown(word)  => self.output.push(format!("Unknown command: {}", word)),
        Command::Hello          => self.output.push("Hi!".to_string()),
        Command::Tell(to, text) => world_actions.push(WorldAction::Tell(self.id, to.to_string(), text.to_string())),
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

  pub fn update_server(&mut self, server: &mut Server) -> io::Result<()> {
    for s in self.output.drain(0..) {
      server.queue_write(self.id, &s)?;
    }
    Ok(())
  }
}
