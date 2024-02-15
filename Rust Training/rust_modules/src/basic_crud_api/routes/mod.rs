use axum::Router;
use crate::basic_crud_api::health_check::get_status_routes;
use crate::basic_crud_api::student::get_student_routes;
use crate::basic_crud_api::user::get_user_routes;



pub fn get_routes() -> Router { 
    let app = Router::new().merge(get_status_routes())
                                    .merge(get_student_routes())
                                    .merge(get_user_routes());
    
    app
}