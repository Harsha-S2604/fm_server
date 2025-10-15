mod routes;
mod handlers;
mod infra;
mod utils;
mod domain;

use axum::{ Router };
use sonyflake::Sonyflake;
use infra::db;

#[tokio::main]
async fn main() {
    let conn_result = db::setup_db();
    if let Err(conn_err) = conn_result {
        eprintln!("(ERROR)::{:#?}", conn_err);
        return;
    }

    let api_routes = routes::register_routes();
    let app_routes = Router::new()
                        .nest("/api", api_routes);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server started running on port 3000");
    axum::serve(listener, app_routes).await.unwrap();
}
