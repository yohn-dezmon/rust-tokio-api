use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::Customer;

// This function allows the data store to be injected into the route and passed along into a handler
// Filter is a trait 
fn with_db(db: Db) -> impl Filer<Extract = (Db,), Error = Infallible> {
    warp::any().map(move || db.clone())
}

// get all customers in the data store 
// hmm what is the "+ Clone" syntax?
fn customers_list(
    db: Db, 
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::list_customers)
}

pub fn customer_routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // customer_list() is more specific than create_customer() so it needs to come first in the sequence of `or` statements 
    get_customer(db.clone())
    .or(update_customer(db.clone()))
    .or(delete_customer(db.clone()))
    .or(create_customer(db.clone()))
    .or(customers_list(db.clone()))
}

fn json_body() -> impl Filter<Extract = (Customer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
    .and(warp::body::json())
}

fn create_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
    .and(warp::post())
    .and(json_body())
    .and(with_db(db))
    .and_then(handlers::create_customer)
}

fn get_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // the path! macro allows us to create a path with a variable of a given type, in this case String
    warp::path!("customers" / String)
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::get_customer)
}

fn update_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
    .and(warp::put())
    .and(json_body())
    .and(with_db(db))
    .and_then(handlers::update_customer)
}

fn delete_customer(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone { 
    warp::path!("customers" / String)
    .and(warp::delete())
    .and(with_db(db))
    .and_then(handlers::delete_customer)
}