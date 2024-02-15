use axum::Router;
use crate::basic_crud_api::health_check::get_status_routes;


pub fn get_routes() -> Router { 
    let app = Router::new().merge(get_status_routes());;
    
    app
}