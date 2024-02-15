use std::{collections::VecDeque, fs, sync::{Arc, RwLock}};
use axum::{extract::Path, response::{IntoResponse, Response}, Json};
use crate::common_modules::{Students as Student, Message};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ALL_STUDENTS: Arc<RwLock<VecDeque<Student>>> = Arc::new(RwLock::new(load_student_data()));
}

fn load_student_data() -> VecDeque<Student> {
    let content = match fs::read_to_string("src/json_data/Student_data_modified.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error loading user data: {}", err);
            String::new()
        }
    };

    let user_data:VecDeque<Student> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error deserializing user data: {}", err);
            VecDeque::new()
        }
    };

    user_data
}


pub async fn add_student(Json(payload): Json<Student>) -> Response {
    let all_users = ALL_STUDENTS.write().unwrap().clone();

    if all_users.iter().any(|user| user.id == payload.id) {
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

    ALL_STUDENTS.write().unwrap().push_back(payload.clone());

    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: format!("User added with Username:{} and id:{}", payload.name, payload.id)
        })
    }
    .into_response();
}


pub async fn get_student(Path(id): Path<u32>) -> Response {
    let mut all_users = ALL_STUDENTS.write().unwrap();

    if let Some(idx) = all_users.iter().position(|user| user.id == id) {
        return Json(Message {
            status: 2000,
            message_key: String::from("Success fetching user"),
            data: all_users.clone().get(idx)
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


pub async fn get_all_students() -> Response {
    return {
        Json(Message {
            status: 2000,
            message_key: String::from("success"),
            data: ALL_STUDENTS.read().unwrap().clone()
        })
    }
    .into_response();
}


pub async fn update_student(Json(payload): Json<Student>) -> Response {
    let mut all_users = ALL_STUDENTS.write().unwrap();

    if let Some(user) = all_users.iter_mut().find(|user| user.id == payload.id) {
        *user = payload.clone();

        return Json(Message {
            status: 2000,
            message_key: String::from("Successfully updated user"),
            data: all_users.clone()
        })
        .into_response();
    }

    all_users.push_back(payload.clone());

    return {
        Json(Message {
            status: 2000,
            message_key: String::from("Added a new user instead of updating"),
            data: all_users.clone()
        })
    }
    .into_response();
}


pub async fn delete_student(Path(id): Path<u32>) -> Response {
    let mut all_users = ALL_STUDENTS.write().unwrap();

    if let Some(idx) = all_users.iter().position(|user| user.id == id) {
        all_users.remove(idx);

        return Json(Message {
            status: 2000,
            message_key: String::from("Success deleting user"),
            data: all_users.clone()
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
