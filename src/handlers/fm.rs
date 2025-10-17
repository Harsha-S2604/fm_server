use crate::utils:: {
    success::FileSuccessResponse,
    errors::ErrorResponse,
};

use axum::{
    extract::{ Query, Extension },
    response::{IntoResponse, Json},
};

use serde::Deserialize;

use crate::infra::{
    db::{ AppState },
    repositories::{ fm_repository },
};

#[derive(Debug, Deserialize)]
pub struct GetQueryParams {
    pub page: u64,
    pub limit: u8,
}

pub async fn get_files(
    Query(params): Query<GetQueryParams>,
    Extension(state): Extension<AppState>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let page = params.page;
    let limit = params.limit;

    if params.page <= 0 || params.limit <= 0 {
        let err_resp = ErrorResponse {
            status: "failed",
            message: "invalid page or limit",
        };

        return Err(Json(err_resp));
    }
    
    let offset = (page - 1) * limit as u64;
    let db = state.get_db();
    let files_result = fm_repository::get_files(limit, offset, db).await;
    
    match files_result {
        Ok(files) => {
            let ok_resp = FileSuccessResponse {
                status: "success",
                data: files,
            };

            Ok(Json(ok_resp))
        },
        Err(e) => {
            eprintln!("(ERROR):: get_files {:#?}", e);
            let err_resp = ErrorResponse {
                status: "failed",
                message: "something went wrong",
            };

            Err(Json(err_resp))
        }
    }
    
}

pub async fn create_files(Extension(state): Extension<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
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

pub async fn delete_files(Extension(state): Extension<AppState>) -> Result<impl IntoResponse, impl IntoResponse> {
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

pub async fn update_files(Extension(state): Extension<AppState>) ->  Result<impl IntoResponse, impl IntoResponse> {
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
