mod routes;
mod handlers;
mod infra;
mod utils;
mod domain;

use axum::{ Router, Extension };
use infra::db;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let pool_result = db::setup_db().await;

    let db_pool;
    match pool_result {
        Ok(pool) => {
            db_pool = pool;
        },
        Err(pool_err) => {
            eprintln!("(ERROR)::{:#?}", pool_err);
            return;
        }
    }

    let db_mutex = Arc::new(db_pool.clone());
    let app_state = db::AppState::new(db_mutex);

    let api_routes = routes::register_routes();
    let app_routes = Router::new()
                        .nest("/api", api_routes)
                        .layer(Extension(app_state)); 
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server started running on port 3000");
    axum::serve(listener, app_routes).await.unwrap();
}
