use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    id: u64,
    name: String,
    location: String,
    f_type: String,
}

