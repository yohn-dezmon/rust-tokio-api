
use serde::{Deserialize, Serialize};


// Represents a customer 
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Customer {
    // A unique identifier for a customer record
    pub guid: String, // these are referred to as "fields" not class attributes
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    // Physical address
    pub address: String,
}