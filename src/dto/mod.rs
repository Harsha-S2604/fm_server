use serde::{ Serialize, Deserialize };

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    pub page: Option<u64>,
    pub limit: Option<u8>,
}


#[derive(Debug, Deserialize)]
pub struct DeletePayload {
    pub file_ids: Option<Vec<u64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FailedPayload {
    pub file_id: Option<u64>,
    pub message: Option<&'static str>,
}

impl FailedPayload {
    pub fn new(file_id: Option<u64>, message: Option<&'static str>) -> Self {
        Self {
            file_id,
            message
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: &'static str,
    pub message: Option<&'static str>,
    pub data: Option<T>,
}
