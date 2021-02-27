use hecs::*;
use crate::component::*;

pub fn update(world: &mut World) {
  for (entity, (io, state)) in world.query::<(&mut LineIO, &mut Lobby)>().iter() {
    match state {
      Lobby::Start => { println!("start"); *state = Lobby::End },
      Lobby::End   => println!("end"),
    }
  }
}
