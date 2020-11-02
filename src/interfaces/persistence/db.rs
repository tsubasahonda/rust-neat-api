use crate::domains::models::item::Items;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct Database {
  pub grocery_list: Arc<RwLock<Items>>,
}

pub fn init_db() -> Database {
  Database {
    grocery_list: Arc::new(RwLock::new(HashMap::new())),
  }
}
