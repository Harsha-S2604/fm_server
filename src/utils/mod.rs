use serde::Serialize;

pub mod errors;
pub mod success;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub status: &'static str,
    pub message: Option<&'static str>,
    pub data: Option<T>,
}
