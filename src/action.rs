use hecs::*;
use crate::component::*;

use crate::global::Global;

#[derive(Debug)]
pub enum Action {
  SetName(Entity, String), // add or replace a Name component

  Output(Entity, String), // send some text to the entity

  Tell(Entity, String, String), // send text to named user

  Hello(Entity),
}

pub fn apply(g: &mut Global, actions: Vec<Action>) {
  println!("{:#?}", actions);

  actions
    .iter()
    .for_each(|action| {
      match action {

        Action::SetName(entity, username) => {
          g.world.insert(*entity, (
            Name(username.to_string()),
          )).ok();
          g.catalog.insert(username.to_string(), *entity);
        },

        Action::Output(entity, text) => {
          if let Some(mut io) = g.world.get_mut::<LineIO>(*entity).ok() {
            io.output.push_back(text.to_string());
          }
        },

        Action::Tell(entity, who, text) => {
          match g.catalog.get(who) {
            Some(e) => {
              if let Some(mut io) = g.world.get_mut::<LineIO>(*e).ok() {
                io.output.push_back(text.to_string());
              }
            },
            None => {
              if let Some(mut io) = g.world.get_mut::<LineIO>(*entity).ok() {
                io.output.push_back("not found".to_string());
              }
            },
          }
        }

        Action::Hello(entity) => {
          if let Some(mut io) = g.world.get_mut::<LineIO>(*entity).ok() {
            io.output.push_back("hello!".to_string());
          }
        },

        _ => unimplemented!()
      }
    });
}
