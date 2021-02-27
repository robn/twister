use hecs::*;
use crate::component::*;

pub fn update(world: &mut World) {
  for (entity, (io, state)) in world.query::<(&mut LineIO, &mut Lobby)>().iter() {
    match state {
      Lobby::Start => {
        io.output.push("oh hey".to_string());
        *state = Lobby::End;
      },
      Lobby::End => {},
    }
  }
}
