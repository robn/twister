use hecs::*;
use crate::component::*;

pub fn update(world: &mut World) {
  for (_, (io, state)) in world.query::<(&mut LineIO, &mut Lobby)>().iter() {
    match state {

      Lobby::Start => {
        io.output.push_back("oh hey".to_string());
        io.output.push_back("what name?".to_string());
        *state = Lobby::Username;
      },

      Lobby::Username => {
        if let Some(username) = io.input.pop_front() {
          io.output.push_back(format!("yeah hi {}", username));
          *state = Lobby::End;
        }
      }

      Lobby::End => {},
    }
  }
}
