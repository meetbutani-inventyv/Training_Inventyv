use axum::{routing::post, Router};
use self::user_model::{add_user, delete_user, get_all_users, get_user, update_user};

mod user_model;


pub fn get_user_routes() -> Router { 
    let routees = Router::new().route("/users/add", post(add_user))
                                        .route("/users/:id", post(get_user))
                                        .route("/users", post(get_all_users))
                                        .route("/users/update", post(update_user))
                                        .route("/users/delete/:id", post(delete_user));
    
    routees
}
