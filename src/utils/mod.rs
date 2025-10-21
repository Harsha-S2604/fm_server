use serde::Serialize;

pub mod errors;
pub mod success;

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ApiData {
    Id(u64),
    Message(&'static str),
    Object(serde_json::Value),
}

impl ApiData {
    pub fn into_id(self) -> Option<u64> {
        if let ApiData::Id(id) = self {
            Some(id)
        } else {
            None
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ApiResponse {
    pub status: &'static str,
    pub message: Option<&'static str>,
    pub data: Option<ApiData>,
}

#[derive(Debug, Serialize)]
pub struct TaskResponse<T> {
    pub status: &'static str,
    pub message: Option<&'static str>,
    pub file_id: u64,
    pub file_data: Option<T>
}

impl<T> TaskResponse<T> {
    pub fn new(status: &'static str, message: Option<&'static str>, file_id: u64, file_data: Option<T>) -> Self {
        Self {
            status,
            message,
            file_id,
            file_data,
        }
    }
}

impl ApiResponse {
    pub fn new(status: &'static str, message: Option<&'static str>, data: Option<ApiData>) -> Self {
        Self {
            status,
            message,
            data,
        }
    }

    pub fn success(data: ApiData, message: &'static str) -> Self {
        Self {
            status: "success",
            message: Some(message),
            data: Some(data),
        }
    }

    pub fn error(message: &'static str) -> Self {
        Self {
            status: "failed",
            message: Some(message),
            data: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FileData {
    pub contents: Vec<u8>,
    pub file_name: String,
}
