use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
  pub name: String,
  pub quantity: i32,
}

pub type Items = HashMap<String, i32>;
