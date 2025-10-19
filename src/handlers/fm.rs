use crate::utils:: {
    success::FileSuccessResponse,
    errors::ErrorResponse,
};

use axum::{
    extract::{ Query, Extension, Multipart },
    response::{ IntoResponse, Json },
};

use futures::future::join_all;
use futures_util::TryStreamExt;
use tokio::{
    io::AsyncWriteExt,
    fs,
    sync::mpsc,
};

use crate::infra::{
    db::{ AppState },
    repositories::{ fm_repository },
};

use crate::dto:: {
    GetQueryParams,
};

use std::{
    sync::Arc,
    env,
    fs as sys_fs,
    path::PathBuf,
};

use crate::domain::file::File;

pub async fn get_files(
    Query(params): Query<GetQueryParams>,
    Extension(state): Extension<AppState>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let page = match params.page {
        Some(p) => p,
        None => 0
    };

    let limit = match params.limit {
        Some(l) => l,
        None => 0
    };

    if page <= 0 || limit <= 0 {
        let err_resp = ErrorResponse {
            status: "failed",
            message: "invliad or missing pagination params: please provide valid page and limit value.",
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

pub async fn upload_files(
    Extension(state): Extension<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let home_path = env::var("HOME").unwrap_or("Nil".to_string());
    if home_path == "Nil" {
        let err_resp = ErrorResponse {
            status: "failed",
            message: "sorry something went wrong",
        };

        return Err(Json(err_resp));
    }

    let mut failed = false;
    let mut dir_path = PathBuf::from(home_path).join("fm_uploads");
    let mut file_upload_handlers = vec![];
    let sf = state.get_sf();
    let db = state.get_db();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let key = field.name().unwrap().to_string();
        
        match key.as_str() {
            "user_id" => {
                let user_id = field.text().await.unwrap();
                if user_id.is_empty() {
                    failed = true;
                    break;
                }

                dir_path = dir_path.join(&user_id);
                let _ = sys_fs::create_dir_all(&dir_path);
            },

            "file" => {
                let file_name = field.file_name().unwrap().to_string();
                let ftype = field.content_type().unwrap_or("octet-stream").to_string();
                let mut file_streams = field.into_stream();
                let file_path = dir_path.join(&file_name);

                let sf_clone = Arc::clone(sf);
                let db_clone = Arc::clone(db);

                let (tx, mut rx) = mpsc::channel::<bytes::Bytes>(8);
                let handler = tokio::spawn(async move {
                    let mut file = fs::File::create(&file_path).await;
                    match file {
                        Ok(mut f) => {
                            while let Some(chunk) = rx.recv().await {
                                let write_result = f.write_all(&chunk).await;

                                match write_result {
                                    Err(e) => {
                                        eprintln!("(ERROR):: Failed to write");
                                        break;
                                    },
                                    _ => {},
                                } 
                            }

                            let file_path_str = file_path.to_string_lossy().into_owned();
                            let file_id = sf_clone.next_id().unwrap();
                            let file = File::new(file_id, file_name, file_path_str, ftype);

                            fm_repository::upload_file(file, &(*db_clone)).await;
                        },
                        Err(e) => {
                            eprintln!("(ERROR):: file creation failed {:?}", e);
                            failed = true;
                        }
                    }
                });

                file_upload_handlers.push(handler);
                while let Some(chunk) = file_streams.try_next().await.unwrap() {
                    tx.send(chunk).await.unwrap();
                }

            },

            _ => {
                eprintln!("(WARNING):: Invalid field {key}");            
            }
        }
    }
    
    if failed {
        let err_resp = ErrorResponse {
            status: "failed",
            message: "please provide the valid user_id",
        };

        return Err(Json(err_resp));
    }

    let file_upload_results = join_all(file_upload_handlers).await;

    let ok_resp = FileSuccessResponse {
        status: "success",
        data: vec!["123"],
    };

    Ok(Json(ok_resp))
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
