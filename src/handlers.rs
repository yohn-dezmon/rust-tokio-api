use std::convert::Infallible; // this is an enum, for errors that can never happen
use warp::{self, http::StatusCode}; // StatusCode is an enum

// hm I guess imports are different whether or not you're in main.rs ?
// need to review imports from rust book.
use crate::models::Customer;
use crate::db::Db;

// Db is a thread safe vector of Customer objects
// why isn't db a reference (&) here?
pub async fn list_customers(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await; // task will yield until a lock can be acquired on the data store
    let customers: Vec<Customer> = customers.clone(); // takes the inner vector out of the MutexGuard
    // converts a vector object to a JSON object
    Ok(warp::reply::json(&customers)) // wraps a JSON reply in an ok variant of the Result type
}

pub async fn create_customer(
    new_customer: Customer, 
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter() {
        // checks to see if customer is already in the datastore
        if customer.guid == new_customer.guid {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    customers.push(new_customer);

    Ok(StatusCode::CREATED)
}

pub async fn get_customer(guid: String, db: Db) -> Result<Box<dyn warp::Reply, Infallible> {
    let customers = db.lock().await;
    for customer in customers.iter() {
        if customer.guid == guid {
            return Ok(Box::new(warp::reply::json(&customer)));
        }
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_customer(
    guid: String, 
    updated_customer: Customer, 
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter_mut() {
        if customer.guid == guid {
            *customer = updated_customer;
            return Ok(StatusCode::OK);
        }   
    }

    Ok(StatusCode::)
}

pub async fn delete_customer(guid: String, db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;
    let customer_count = customers.len();

    customers.retain(|customer| customer.guid != guid);

    let deleted = customers.len() != customer_count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Ok(StatusCode::NOT_FOUND)
    }
}