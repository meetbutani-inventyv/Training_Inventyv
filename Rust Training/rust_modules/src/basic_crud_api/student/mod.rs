use axum::{routing::post, Router};
use self::student_model::{add_student, delete_student, get_all_students, get_student, update_student};

mod student_model;


pub fn get_student_routes() -> Router { 
    let routees = Router::new().route("/students/add", post(add_student))
                                        .route("/students/:id", post(get_student))
                                        .route("/students", post(get_all_students))
                                        .route("/students/update", post(update_student))
                                        .route("/students/delete/:id", post(delete_student));
    
    routees
}
