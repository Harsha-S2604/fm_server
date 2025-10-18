use axum::{ 
    Router,
    routing::{ get, post }
};

use crate::handlers::fm::{ 
    delete_files,
    update_files,
    get_files,
    upload_files
};

pub fn get_router() -> Router {
    let router = Router::new()
                    .route("/", get(get_files).delete(delete_files).put(update_files))
                    .route("/upload", post(upload_files));
    router
}
