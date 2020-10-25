use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
  pub name: String,
  pub quantity: i32,
}

impl Item {
  pub fn new(name: &str, quantity: i32) -> Result<Self, String> {
    let size = name.chars().count();
    if size < 1 || size > 20 {
      return Err("商品名は20文字以内です".to_string());
    }

    if name.chars().any(|c| !c.is_ascii_alphabetic()) {
      return Err("名前に使用できる文字はA-Z, a-zです".to_string());
    }

    return Ok(Item {
      name: name.to_string(),
      quantity: quantity,
    });
  }
}

pub type Items = HashMap<String, i32>;
