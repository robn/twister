use hecs::*;
use crate::component::*;

use crate::global::Global;
use crate::action::Action;

pub fn update(g: &mut Global) -> Vec<Action> {
  g.world.query_mut::<(&mut LineIO, &mut Connection)>()
    .into_iter()
    .filter_map(|(entity, (io, state))| {
      match state {

        Connection::Connected => {
          io.output.push_back("oh hey this would be the banner or something".to_string());
          *state = Connection::Welcome;
          None
        },

        Connection::Welcome => {
          io.output.push_back("what name?".to_string());
          *state = Connection::Username;
          None
        },

        Connection::Username => {
          match io.input.pop_front() {
            Some(username) => {
              io.output.push_back(format!("yeah hi {}", username));
              *state = Connection::Online;
              Some(Action::SetName(entity, username))
            },
            None => None,
          }
        },

        _ => None,
      }
    })
    .collect()

  /*
  let wall_entity: Option<Entity> = world.query_mut::<With<Channel, &Name>>()
    .into_iter()
    .filter_map(|(entity, name)| {
      match name.0 == "wall" {
        true  => Some(entity),
        false => None,
      }
    })
    .next();

  to_promote.iter().for_each(|(entity, username)| {
    println!("promote {:?}", entity);

    // remove from lobby
    world.remove_one::<Lobby>(*entity).ok();

    // add to main world
    world.insert(*entity, (
      Name(username.to_string()),
    )).ok();

    // post channel join event
    if let Some(e) = wall_entity {
      world.spawn((
        ChannelEvent::Join(e, *entity),
      ));
    }
  });
  */
}
