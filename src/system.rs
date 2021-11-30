use crate::global::Global;
use crate::action::Action;

pub trait System {
  fn update(&mut self, g: &mut Global) -> Vec<Action>; // XXX remove mut once we're converted
  fn apply(&mut self, g: &mut Global, actions: &Vec<Action>);
}
