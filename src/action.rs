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
        },

        Action::Output(entity, text) => {
          if let Some(mut io) = g.world.get_mut::<LineIO>(*entity).ok() {
            io.output.push_back(text.to_string());
          }
        },

        Action::Tell(entity, who, text) => {
          // XXX this is stupid and I want a real name->entity lookup but this is fine for now
          let n_told = g.world.query_mut::<(&Name, &mut LineIO)>()
            .into_iter()
            .flat_map(|(e, (name, io))| {
              if name.0 == *who {
                io.output.push_back(text.to_string());
                Some(())
              }
              else {
                None
              }
            })
            .collect::<Vec<()>>()
            .len();
          if n_told == 0 {
            if let Some(mut io) = g.world.get_mut::<LineIO>(*entity).ok() {
              io.output.push_back("not found".to_string());
            }
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
