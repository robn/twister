use hecs::*;
use crate::component::*;

#[derive(Debug)]
enum Command {
  Hello,
  Tell(String, String),
  Error(String),
  Unknown(String),
}

pub fn update(world: &mut World) {
  // a player has IO and a name
  let commands: Vec<(Entity, Command)> = world.query::<With<Name, &mut LineIO>>()
    .iter()
    .flat_map(|(entity, io)| {
      io.input.drain(0..).flat_map(move |line| {
        let mut iter = line.split_whitespace();

        match iter.next() {
          None => None,
          Some(word) => {
            let args: Vec<String> = iter.map(|s| s.to_string()).collect();
            match word.to_lowercase().as_ref() {
              "hello" => Some((entity, Command::Hello)),
              "tell"  => match args.len() {
                n if n < 2 => Some((entity, Command::Error(format!("try: tell [who] [what...]")))),
                _          => Some((entity, Command::Tell(args[0].to_string(), args[1..].join(" ")))),
              },
              _ => Some((entity, Command::Unknown(word.to_string()))),
            }
          },
        }
      })
    })
    .collect();
  
  println!("{:?}", commands);
}
