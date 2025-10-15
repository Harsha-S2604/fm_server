use crate::utils:: {
    success::FileSuccessResponse,
    errors::ErrorResponse,
};

use axum::{
    http::{ StatusCode },
    response::{IntoResponse, Json},
};

use serde::{
    Serialize,
};

pub async fn get_files() -> Result<impl IntoResponse, impl IntoResponse> {
    let id = 43;
    if id == 43 {
        let success_resp = FileSuccessResponse {
            status: "success",
            data: "123",
        };

        return Ok(Json(success_resp));
    }
    
    let err_resp = ErrorResponse {
       status: "failed",
       message: "Not found",
    };

    Err(Json(err_resp))
}

pub async fn create_files() -> Result<impl IntoResponse, impl IntoResponse> {
    let id = 43;
    if id == 43 {
        let success_resp = FileSuccessResponse {
            status: "sucess",
            data: "123",
        };

        return Ok(Json(success_resp));
    }
    
    let err_resp = ErrorResponse {
       status: "failed",
       message: "Not found",
    };

    Err(Json(err_resp))
}

pub async fn delete_files() -> Result<impl IntoResponse, impl IntoResponse> {
    let id = 43;
    if id == 43 {
        let success_resp = FileSuccessResponse {
            status: "success",
            data: "123",
        };

        return Ok(Json(success_resp));
    }
    
    let err_resp = ErrorResponse {
       status: "failed",
       message: "Not found",
    };

    Err(Json(err_resp))
}

pub async fn update_files() ->  Result<impl IntoResponse, impl IntoResponse> {
    let id = 43;
    if id == 43 {
        let success_resp = FileSuccessResponse {
            status: "success",
            data: "123",
        };

        return Ok(Json(success_resp));
    }
    
    let err_resp = ErrorResponse {
       status: "failed",
       message: "Not found",
    };

    Err(Json(err_resp))
}
