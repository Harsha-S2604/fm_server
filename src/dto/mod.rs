use serde::{ Deserialize };

#[derive(Debug, Deserialize)]
pub struct FilePayload {
    pub name: String, 
    pub data: Vec<u8>,
    pub ftype: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateFilePayload {
    pub user_id: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u8>,
}
