use axum::{ 
    Router,
    routing::{get, post, delete, put}
};

use crate::handlers::fm;

pub fn get_router() -> Router {
    let router = Router::new()
                    .route("/{id}", get(fm::get_files).delete(fm::delete_files).put(fm::update_files))
                    .route("/create_files", post(fm::create_files));
    router
}
