use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FileSuccessResponse<T> {
    pub status: &'static str,
    pub data: T,
}
