use axum::{ Router };
use sonyflake::Sonyflake;

mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let api_routes = routes::register_routes();
    let app_routes = Router::new()
                        .nest("/api", api_routes);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    println!("Server started running on port 3000");
    axum::serve(listener, app_routes).await.unwrap();
}
