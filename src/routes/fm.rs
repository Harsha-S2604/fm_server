use axum::{ 
    Router,
    routing::{get, post, delete, put}
};

use crate::handlers::fm::{ delete_files, update_files, get_files, create_files };

pub fn get_router() -> Router {
    let router = Router::new()
                    .route("/", delete(delete_files).put(update_files))
                    .route("/{id}", get(get_files))
                    .route("/create_files", post(create_files));
    router
}
