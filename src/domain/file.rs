use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub f_type: String,
}

