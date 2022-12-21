use warp;

// makes a models module
mod db;
mod handlers;
mod models;
mod routes;

// this is a function attribute
// sets entrypoint for tokyo runtime
// allows us to make main() async
#[tokio::main]
async fn main() {
    // initialize the datastore
    let db = db::init_db();
    // get customer routes wrapper
    let customer_routes = routes::customer_routes(db);

    warp::serve(customer_routes)
    .run(([127,0,0,1], 3000))
    .await;
}
