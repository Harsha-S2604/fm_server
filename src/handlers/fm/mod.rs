mod service;

use crate::utils:: {
    ApiResponse,
    ApiData,
    FileData,
    TaskResponse,
    success::FileSuccessResponse,
    errors::ErrorResponse,
};

// use crate::handlers::fm::service;

use axum::{
    http::{header, StatusCode},
    extract::{ Query, Extension, Multipart },
    response::{ IntoResponse, Json },
};
use zip::{
    write::SimpleFileOptions,
    ZipWriter,
    CompressionMethod::Stored,
};
use serde_json::json;
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
    DeletePayload,
    FailedPayload,
    DownloadData,
};

use std::{
    sync::Arc,
    io::{ Read, Write },
    env,
    fs as sys_fs,
    path::{ Path, PathBuf },
};

use crate::domain::file::File;


fn get_file_path(file_dir: &PathBuf, file_name: &String) -> (PathBuf, String) {
    let mut copy_num = 1;
    let mut file_path = file_dir.join(file_name);

    let path = PathBuf::from(file_name);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
    let mut new_file_name = String::from(file_name);

    while file_path.exists() {
        new_file_name = if ext.is_empty() {
            format!("{}({})", stem, copy_num)
        } else {
            format!("{}({}).{}", stem, copy_num, ext)
        };

        file_path = file_dir.join(&new_file_name);
        copy_num += 1;
    }

    (file_path, new_file_name)

}

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
                message: None,
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
    let mut user_id: u64 = 0;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let key = field.name().unwrap().to_string();

        match key.as_str() {
            "user_id" => {
                let user_id_str = field.text().await.unwrap();
                if user_id_str.is_empty() {
                    failed = true;
                    break;
                }

                user_id = match user_id_str.parse() {
                    Ok(uid) => uid,
                    Err(e) => {
                        eprintln!("(INVALID_DATA):: Failed to parse user_id");
                        failed = true;
                        break;
                    }
                };

                dir_path = dir_path.join(&user_id_str);
                let _ = sys_fs::create_dir_all(&dir_path);
            },

            "file" => {
                let mut file_name = field.file_name().unwrap().to_string();
                let ftype = field.content_type().unwrap_or("octet-stream").to_string();
                let (file_path, new_file_name) = get_file_path(&dir_path, &file_name);
                let fname = new_file_name.to_string();
                
                let mut file_streams = field.into_stream();
                let sf_clone = Arc::clone(sf);
                let db_clone = Arc::clone(db);

                let (tx, mut rx) = mpsc::channel::<bytes::Bytes>(8);
                let handler = tokio::spawn(async move {
                    let file = fs::File::create(&file_path).await;
                    match file {
                        Ok(mut f) => {
                            while let Some(chunk) = rx.recv().await {
                                let write_result = f.write_all(&chunk).await;
                                if let Err(e) = write_result {
                                    eprintln!("(ERROR):: Failed to write {:?}", e);
                                    return Err(file_name);
                                }
                            }

                            let file_path_str = file_path.to_string_lossy().into_owned();
                            let file_id = sf_clone.next_id().unwrap();
                            let file = File::new(file_id, fname, file_path_str, ftype, user_id);

                            let db_res = fm_repository::upload_file(file, &(*db_clone)).await;
                            if let Err(db_err) = db_res {
                                // handle error here
                                eprintln!("{:?}", db_err);
                                return Err(file_name);
                            }

                            Ok(file_name)

                        },
                        Err(e) => {
                            eprintln!("(ERROR):: file creation failed {:?}", e);
                            Err(file_name)
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
    let mut failed_uploads = vec![];

    for file_upload_result in file_upload_results {
        if let Ok(inner_result) = file_upload_result {
            if let Err(file_name) = inner_result {
                failed_uploads.push(file_name);
            }
        } else if let Err(join_err) = file_upload_result {
            eprintln!("(ERROR):: {:?}", join_err);
        }
    }

    let ok_resp = FileSuccessResponse {
        status: "success",
        data: failed_uploads,
        message: None
    };

    Ok(Json(ok_resp))
}

pub async fn delete_files(
    Extension(state): Extension<AppState>,
    Json(payload): Json<DeletePayload>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut fut_handlers = vec![];
    let db = state.get_db();
    let file_ids = match payload.file_ids {
        Some(f_ids) => f_ids,
        None => vec![],
    };

    for idx in 0..file_ids.len() {
        let file_id = file_ids[idx];
        let db_clone = Arc::clone(db);

        let fut_handler = tokio::spawn(async move {
            let delete_result = fm_repository::delete_file(file_id, &(*db_clone)).await;
            if let Err(del_err) = delete_result {
                let err_resp = ApiResponse::new("failed", Some(del_err), Some(ApiData::Id(file_id)));
                return Err(err_resp);
            }
            

            let ok_resp = ApiResponse::new("success", None, Some(ApiData::Id(file_id)));
            Ok(ok_resp)
        });

        fut_handlers.push(fut_handler);
    }

    let file_delete_results = join_all(fut_handlers).await;
    let mut failed_files: Vec<FailedPayload> = vec![];

    for file_del_res in file_delete_results {
        if let Ok(inner_result) = file_del_res {
            if let Err(resp) = inner_result {
                let data: Option<u64> = resp.data.and_then(ApiData::into_id);
                let failed_res = FailedPayload::new(data, resp.message);
                failed_files.push(failed_res);
            }
        } else if let Err(join_err) = file_del_res {
            eprintln!("(ERROR):: {:?}", join_err);
            let err_resp = ErrorResponse {
                status: "failed",
                message: "sorry, something went wrong",
            };

            return Err(Json(err_resp));
        }
    }

    if failed_files.len() > 0 {
        let err_resp = FileSuccessResponse {
            status: "failed",
            data: failed_files,
            message: Some("Some files are failed to upload"),
        };

        return Ok(Json(err_resp));
    }

    let ok_resp = FileSuccessResponse {
        status: "success",
        data: failed_files,
        message: None,
    };

    Ok(Json(ok_resp))
}

pub async fn download_files(
    Extension(state): Extension<AppState>,
    Json(payload): Json<DownloadData>,
) -> Result <impl IntoResponse, impl IntoResponse> {
    let db = state.get_db();

    let user_id = match payload.user_id {
        Some(uid) => uid,
        None => {
            let api_resp = ApiResponse::error("invalid user_id");
            return Err(Json(api_resp));
        }
    };

    let file_ids = match payload.file_ids {
        Some(f_ids) => f_ids,
        None => {
            let api_resp = ApiResponse::error("invalid file_ids");
            return Err(Json(api_resp));
        }
    };

    let mut buffer = Vec::new();
    let mut download_handlers = vec![];
    let mut zip = ZipWriter::new(std::io::Cursor::new(&mut buffer));
    let zip_opt = SimpleFileOptions::default().compression_method(Stored).unix_permissions(0o644);

    for file_id in file_ids {
        let db_clone = Arc::clone(db);
        let download_handler = tokio::spawn(async move {
            let file_result = fm_repository::get_file(file_id, &(*db_clone)).await;
            let file_data = match file_result {
                Ok(file) => file,
                Err(file_err) => {
                    let message = match file_err {
                        "NOT_FOUND" => "file not found",
                        "DB_ERROR" => "internal error",
                        _ => "unknown error",
                    };

                    return TaskResponse::new("failed", Some(message), file_id, None);
                }
            };

            let mut contents = Vec::new();
            match sys_fs::File::open(&(file_data.location)) {
                Ok(mut fp) => {
                    match fp.read_to_end(&mut contents) {
                        Err(e) => {
                            eprintln!("failed to read the file {:?}", e);
                            let message = Some("failed to read the file");
                            return TaskResponse::new("failed", message, file_id, None);
                        },
                        _ => {}
                    }
                },
                Err(e) => {
                    eprintln!("(ERROR):: File not found");
                    let message = Some("file not found");
                    let task_result = TaskResponse::new("failed", message, file_id, None);
                    return task_result;
                }
            }
            
            let data = Some(FileData {
                contents,
                file_name: file_data.name
            });
            let task_result = TaskResponse::new("success", None, file_id, data);
            task_result
        });

        download_handlers.push(download_handler);
    }

    let download_results = join_all(download_handlers).await;

    let mut results = vec![];
    for download_result in download_results {
        match download_result {
            Ok(task_result) => {
                if let Some(ref file_data) = task_result.file_data {
                    let _ = zip.start_file(&file_data.file_name, zip_opt);
                    let _ = zip.write_all(&file_data.contents);
                }
                
                let status = task_result.status;
                let data = Some(ApiData::Id(task_result.file_id));
                let message = task_result.message;

                let api_resp = ApiResponse::new(status, message, data);
                results.push(api_resp);
            },
            Err(join_err) => {
                eprintln!("task failed to join: {:?}", join_err);
                let api_resp = ApiResponse::error("internal error");
                return Err(Json(api_resp));
            }
        }
    }

    zip.finish().unwrap();

    let zip_name = "fm_files.zip";
    let data = ApiData::Object(json!(results));
    let api_resp = ApiResponse::success(data, "");

    let response = axum::response::Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "application/zip")
                    .header(header::CONTENT_DISPOSITION, 
                        format!("attachment; filename=\"{}\"", zip_name),)
                    .body(axum::body::Body::from(buffer))
                        .unwrap();
    
    Ok(response)
}

pub async fn update_files(Extension(state): Extension<AppState>) ->  Result<impl IntoResponse, impl IntoResponse> {
    let id = 43;
    if id == 43 {
        let success_resp = FileSuccessResponse {
            status: "success",
            data: "123",
            message: None,
        };

        return Ok(Json(success_resp));
    }
    
    let err_resp = ErrorResponse {
       status: "failed",
       message: "Not found",
    };

    Err(Json(err_resp))
}
