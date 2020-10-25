use crate::domains::models;
use crate::interfaces::persistence::db::Database;
use std::collections::HashMap;
use warp::http;

pub async fn update_grocery_list_item(
  db: Database,
  item: models::item::Item,
) -> Result<impl warp::Reply, warp::Rejection> {
  let new_item = models::item::Item::new(&item.name, item.quantity);
  match new_item {
    Ok(result) => {
      db.grocery_list.write().insert(result.name, result.quantity);
      Ok(warp::reply::with_status(
        "Added items to the grocery list".to_string(),
        http::StatusCode::CREATED,
      ))
    }
    Err(err) => {
      return Ok(warp::reply::with_status(
        format!("{}{}", "Err: ", err),
        warp::http::StatusCode::BAD_REQUEST,
      ));
    }
  }
}

pub async fn delete_grocery_list_item(
  db: Database,
  id: models::id::Id,
) -> Result<impl warp::Reply, warp::Rejection> {
  db.grocery_list.write().remove(&id.name);

  Ok(warp::reply::with_status(
    "Removed item from grocery list",
    http::StatusCode::OK,
  ))
}

pub async fn get_grocery_list(db: Database) -> Result<impl warp::Reply, warp::Rejection> {
  let mut result = HashMap::new();
  let r = db.grocery_list.read();

  for (key, value) in r.iter() {
    result.insert(key, value);
  }

  Ok(warp::reply::json(&result))
}
