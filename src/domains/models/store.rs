use super::item;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Store {
  pub grocery_list: Arc<RwLock<item::Items>>,
}

impl Store {
  pub fn new() -> Self {
    Store {
      grocery_list: Arc::new(RwLock::new(HashMap::new())),
    }
  }
}
