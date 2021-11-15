use hecs::*;
use std::collections::HashMap;

pub struct Global {
  pub world:   World,
  pub catalog: HashMap<String,Entity>,
}
