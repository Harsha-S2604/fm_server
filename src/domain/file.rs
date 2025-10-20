use serde::{ Serialize, Deserialize };
use crate::domain::user::User;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub f_type: String,
    pub user_id: u64,
}

impl File {
    pub fn new(id: u64, name: String, location: String, f_type: String, user_id: u64) -> Self {
        Self {
            id,
            name,
            location,
            f_type,
            user_id,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() || self.location.is_empty() || self.f_type.is_empty()
    }
}


