use std::{collections::VecDeque, fs, sync::{Arc, RwLock}};
use axum::{extract::Path, response::{IntoResponse, Response}, Json};

use crate::common_modules::{Employees as Employee, Message};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALL_EMPLOYEES: Arc<RwLock<VecDeque<Employee>>> = Arc::new(RwLock::new(load_employee_data()));
}


fn load_employee_data() -> VecDeque<Employee> {
    let content = match fs::read_to_string("src/json_data/Employee_data.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error loading user data: {}", err);
            String::new()
        }
    };

    let user_data:VecDeque<Employee> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error deserializing user data: {}", err);
            VecDeque::new()
        }
    };

    user_data
}


pub async fn add_employee(Json(payload): Json<Employee>) -> Response {
    let all_employees = ALL_EMPLOYEES.write().unwrap().clone();

    if all_employees.iter().any(|user| user.id == payload.id) {
        return {
            Json(Message {
                status: 2000,
                message_key: String::from("Conflicting user id"),
                data: "Failed to add user"
            })
        }
        .into_response();
    }

    // let random_id = rand::thread_rng().gen_range(0..=1000);

    ALL_EMPLOYEES.write().unwrap().push_back(payload.clone());

    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: format!("User added with Username:{} and id:{}", payload.name, payload.id)
        })
    }
    .into_response();
}


pub async fn get_employee(Path(id): Path<u32>) -> Response {
    let all_employees = ALL_EMPLOYEES.write().unwrap();

    if let Some(idx) = all_employees.iter().position(|user| user.id == id) {
        return Json(Message {
            status: 2000,
            message_key: String::from("Success fetching user"),
            data: all_employees.clone().get(idx)
        })
        .into_response();
    }

    return {
        Json(Message {
            status: 2000,
            message_key: String::from("No user found"),
            data: "Error fetching the user"
        })
    }
    .into_response();
}


pub async fn get_all_employees() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: ALL_EMPLOYEES.read().unwrap().clone()
        })
    }
    .into_response();
}


pub async fn update_employee(Json(payload): Json<Employee>) -> Response {
    let mut all_employees = ALL_EMPLOYEES.write().unwrap();

    if let Some(user) = all_employees.iter_mut().find(|user| user.id == payload.id) {
        *user = payload.clone();

        return Json(Message {
            status: 2000,
            message_key: String::from("Successfully updated user"),
            data: all_employees.clone()
        })
        .into_response();
    }

    all_employees.push_back(payload.clone());

    return {
        Json(Message {
            status: 2000,
            message_key: String::from("Added a new user instead of updating"),
            data: all_employees.clone()
        })
    }
    .into_response();
}


pub async fn delete_employee(Path(id): Path<u32>) -> Response {
    let mut all_employees = ALL_EMPLOYEES.write().unwrap();

    if let Some(idx) = all_employees.iter().position(|user| user.id == id) {
        all_employees.remove(idx);

        return Json(Message {
            status: 2000,
            message_key: String::from("Success deleting user"),
            data: all_employees.clone()
        })
        .into_response();
    }

    return {
        Json(Message {
            status: 2000,
            message_key: String::from("No user found to be deleted"),
            data: "Error deleting an user"
        })
    }
    .into_response();
}
