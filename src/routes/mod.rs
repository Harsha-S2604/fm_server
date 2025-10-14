use axum::{ Router };

mod fm;

pub fn register_routes() -> Router {
    let fm_routes = fm::get_router();
    let api_routes = Router::new()
                        .nest("/fm", fm_routes);
    api_routes
}
