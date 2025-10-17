use serde::Serialize;

pub mod errors;
pub mod success;

use crate::utils::{
    success::{ FileSuccessResponse },
    errors::{ ErrorResponse },
};

#[derive(Serialize)]
pub enum ApiResponse<T> {
    Success(FileSuccessResponse<T>),
    Error(ErrorResponse),
}
