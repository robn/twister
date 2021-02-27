use std::collections::HashMap;
use uuid::Uuid;

struct Directory {
  named_session: HashMap<String,Uuid>;
}

impl Directory {
  pub fn new() -> Self {
    Default::default()
  }
}
