use neat_api::interfaces::persistence::db::init_db;
use neat_api::interfaces::server::filters::groceries_api;

#[tokio::main]
async fn main() {
  let database = init_db();

  warp::serve(groceries_api(database))
    .run(([127, 0, 0, 1], 3030))
    .await;
}
