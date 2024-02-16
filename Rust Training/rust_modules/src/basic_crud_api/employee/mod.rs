use axum::{routing::post, Router};
use self::employee_model::{add_employee, delete_employee, get_all_employees, get_employee, update_employee};

mod employee_model;


pub fn get_employee_routes() -> Router { 
    let routees = Router::new().route("/employees/add", post(add_employee))
                                        .route("/employees/:id", post(get_employee))
                                        .route("/employees", post(get_all_employees))
                                        .route("/employees/update", post(update_employee))
                                        .route("/employees/delete/:id", post(delete_employee));
    
    routees
}
