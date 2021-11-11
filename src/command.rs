use hecs::*;
use crate::component::*;
use crate::action::Action;

pub fn update(world: &mut World) -> Vec<Action> {
  // a player has IO and a name
  world.query_mut::<With<Name, &mut LineIO>>()
    .into_iter()
    .flat_map(|(entity, io)| {
      io.input.drain(0..).flat_map(move |line| {
        let mut iter = line.split_whitespace();

        match iter.next() {
          None => None,
          Some(word) => {
            let args: Vec<String> = iter.map(|s| s.to_string()).collect();
            match word.to_lowercase().as_ref() {
              "hello" => Some(Action::Hello(entity)),
              _ => None,
            }
          }
        }
      })
    })
    .collect()
}
    
              /*
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
              */
