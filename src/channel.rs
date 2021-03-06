use hecs::*;
use crate::component::*;

use std::collections::{HashMap,VecDeque};

enum ChannelMessage {
  Join(Entity),
}

pub fn update(world: &mut World) {
  let mut messages: HashMap<Entity,VecDeque<ChannelMessage>> = HashMap::new();

  let to_remove: Vec<Entity> = world.query::<&ChannelEvent>()
    .iter()
    .map(|(entity, event)| {
      match event {
        ChannelEvent::Join(channel, user) => {
          if let Ok(mut c) = world.get_mut::<Channel>(*channel) {
            c.members.insert(*user);
            messages.entry(*channel).or_insert_with(|| VecDeque::new()).push_back(ChannelMessage::Join(*user));
            println!("channel join {:#?} {:#?}", channel, user);
          }
        }
      }
      entity
    })
    .collect();

  to_remove.iter().for_each(|e| { world.despawn(*e).ok(); });

  world.query::<&Channel>()
    .iter()
    .for_each(|(entity, c)| {
      if let Some(queue) = messages.remove(&entity) {
        queue
          .iter()
          .for_each(|message| {
            match message {

              ChannelMessage::Join(user) => {
                let joinmsg = match world.get::<Name>(*user) {
                  Ok(name) => Some(format!("{} has joined", (*name).0)),
                  _        => None,
                };

                if let Some(msg) = joinmsg {
                  world.query::<With<Name, &mut LineIO>>()
                    .iter()
                    .for_each(|(entity, io)| {
                      io.output.push_back(msg.to_string());
                    });
                }
              },
            }
          });
      }
    });
}
