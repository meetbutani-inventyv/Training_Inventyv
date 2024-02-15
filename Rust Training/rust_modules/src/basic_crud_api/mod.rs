use std::net::SocketAddr;
use middlewares::get_middlewares;
use crate::basic_crud_api::routes::get_routes;


mod routes;
mod middlewares;
mod health_check;
mod student;
mod user;


#[tokio::main]
pub async fn main() {
    let app = get_routes();
    let app = get_middlewares(app);
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let _server = axum::Server::bind(&addr).serve(app.clone().into_make_service()).await.expect("something went wrong!");
}