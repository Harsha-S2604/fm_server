use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(id: u64, email: String, password: String) -> Self {
        Self {
            id,
            email,
            password,
        }
    }
}
