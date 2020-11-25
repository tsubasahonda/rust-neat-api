extern crate diesel;

use crate::domains::models::id::Id;
use crate::domains::models::item::Item;
use crate::interfaces::persistence::db::Database;
use crate::interfaces::server::handler::{
  delete_grocery_list_item, get_grocery_list, update_grocery_list_item,
};

use warp::{Filter, Rejection, Reply};

fn delete_json() -> impl Filter<Extract = (Id,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

fn post_json() -> impl Filter<Extract = (Item,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn groceries_api(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  add_items(db.clone())
    .or(get_items(db.clone()))
    .or(delete_item(db.clone()))
    .or(update_item(db.clone()))
}

fn v1() -> warp::filters::BoxedFilter<()> {
  warp::path("v1").boxed()
}

fn groceries() -> warp::filters::BoxedFilter<()> {
  v1().and(warp::path("groceries")).boxed()
}

fn add_items(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  groceries()
    .and(warp::post())
    .and(post_json())
    .and_then(move |item: Item| update_grocery_list_item(db.clone(), item))
}

fn get_items(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  groceries()
    .and(warp::get())
    .and_then(move || get_grocery_list(db.clone()))
}

fn delete_item(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  groceries()
    .and(warp::delete())
    .and(delete_json())
    .and_then(move |id| delete_grocery_list_item(db.clone(), id))
}

fn update_item(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  groceries()
    .and(warp::put())
    .and(post_json())
    .and_then(move |item: Item| update_grocery_list_item(db.clone(), item))
}
