extern crate diesel;

use crate::domains::models;
use crate::schema;

fn main() {
  use schema::posts::dsl::*;

  let connection = establish_connection();
  let results = posts
    .filter(published.eq(true))
    .limit(5)
    .load::<models::post::Post>(&connection)
    .expect("Error loading posts");

  println!("Displaying {} posts", results.len());
  for post in results {
    println!("{}", post.title);
    println!("----------\n");
    println!("{}", post.body);
  }
}
