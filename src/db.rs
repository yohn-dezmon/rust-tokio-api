
use serde_json::from_reader;
use std::fs::File;
// needs to be wrapped in a thread safe reference 
use std::sync::Arc;
use tokio::sync::Mutex;


use crate::models::Customer;

// Arc allows for concurrent access to the data structure 
// Mutex ensures that only one thread can modify the data structure at once
pub type Db = Arc<Mutex<Vec<Customer>>>;


pub fn init_db() -> Db {
    let file = File::open("./data/customers.json");
    match file => {
        // file is a Result type value
        Ok(json) => {
            let customers = from_reader(json).unwarp();
            Arc::new(Mutex::new(customers))
        },
        Err(_) => {
            // initialize an empty db
            Arc::new(Mutex::new(Vec::new()))
        }
    }
}

