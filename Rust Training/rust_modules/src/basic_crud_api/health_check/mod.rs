use axum::{routing::get, Router};

pub fn get_status_routes() -> Router {
    Router::new().route("/health/check", get(root))
}


async fn root() -> String {
    "Hello World from server".to_string()
}
