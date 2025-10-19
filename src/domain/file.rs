use serde::{ Serialize, Deserialize };

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct File {
    pub id: u64,
    pub name: String,
    pub location: String,
    pub f_type: String,
}

impl File {
    pub fn new(id: u64, name: String, location: String, f_type: String) -> Self {
        Self {
            id,
            name,
            location,
            f_type,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name.is_empty() || self.location.is_empty() || self.f_type.is_empty()
    }
}


