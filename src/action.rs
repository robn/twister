use hecs::*;
use crate::component::*;

#[derive(Debug)]
pub enum Action {
  SetName(Entity, String), // add or replace a Name component

  Hello(Entity),
}

pub fn apply(world: &mut World, actions: Vec<Action>) {
  println!("{:#?}", actions);

  actions
    .iter()
    .for_each(|action| {
      match action {

        Action::SetName(entity, username) => {
          world.insert(*entity, (
            Name(username.to_string()),
          )).ok();
        },

        Action::Hello(entity) => {
          if let Some(mut io) = world.get_mut::<LineIO>(*entity).ok() {
            io.output.push_back("hello!".to_string());
          }
        },

        _ => unimplemented!()
      }
    });
}
