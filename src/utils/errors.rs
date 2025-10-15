use std::fmt;
use axum:: {
    http::{ StatusCode }
};

use serde::{
    Serialize,
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub status: &'static str,
    pub message: &'static str,
}
