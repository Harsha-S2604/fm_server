use serde::{ Deserialize };

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u8>,
}
