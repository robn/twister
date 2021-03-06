use hecs::*;
use crate::component::*;

pub fn update(world: &mut World) {
  let to_promote: Vec<_> = world.query_mut::<(&mut LineIO, &mut Lobby)>()
    .into_iter()
    .filter_map(|(entity, (io, state))| {
      match state {

        Lobby::Start => {
          io.output.push_back("oh hey".to_string());
          io.output.push_back("what name?".to_string());
          *state = Lobby::Username;
          None
        },

        Lobby::Username => {
          if let Some(username) = io.input.pop_front() {
            io.output.push_back(format!("yeah hi {}", username));
            *state = Lobby::End(username);
          }
          None
        },

        Lobby::End(username) => {
          Some((entity, username.to_string()))
        },
      }
    })
    .collect();

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
}
