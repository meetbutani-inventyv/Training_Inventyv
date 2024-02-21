use std::{collections::VecDeque, fs, sync::{Arc, RwLock}};
use lazy_static::lazy_static;
use super::models::UserJson;

lazy_static! {
    pub static ref USERS: Arc<RwLock<VecDeque<UserJson>>> = Arc::new(RwLock::new(load_user_data()));
}


/// Function to read the user data from the json file
fn load_user_data() -> VecDeque<UserJson> {
    let content = match fs::read_to_string("src/json_data/Master_data.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error loading user data: {}", err);
            String::new()
        }
    };

    let user_data:VecDeque<UserJson> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error deserializing user data: {}", err);
            VecDeque::new()
        }
    };

    user_data
}


/// Function to get the data of a user
pub async fn read_user(id: u32) -> Result<UserJson, String> {
    let all_users = USERS.write().unwrap();

    if let Some(idx) = all_users.iter().position(|user| user.id == id) {
        Ok(all_users.get(idx).unwrap().clone())
    }
    else {
        Err("No such user exists".to_string())
    }
}


/// Function to get the data of all users
pub async fn read_all_user() -> Result<VecDeque<UserJson>, String> {
    let all_users = USERS.write().unwrap();

    if all_users.len()>0 {
        Ok(all_users.clone())
    }
    else {
        Err("Error fetching all the users".to_string())
    }
}


/// Function to add a new user
pub async fn create_user(users: UserJson) -> Result<String, String> {
    let all_users = USERS.write().unwrap().clone();

    if all_users.iter().any(|user| user.id == users.id) {
        Err("Conflicting user id".to_string())
    }
    else {
        USERS.write().unwrap().push_back(users);
        Ok("New User added successfully".to_string())
    }
}


/// Function to update the user data
pub async fn update_user(users: UserJson) -> Result<String, String> {
    let mut all_users = USERS.write().unwrap();

    if let Some(idx) = all_users.iter().position(|user| user.id == users.id) {
        all_users[idx] = users;
        Ok("User updated successfully".to_string())
    }
    else {
        USERS.write().unwrap().push_back(users);
        Err("New User added instead of updating".to_string())
    }
}


/// Function to delete a user
pub async fn delete_user(id: u32) -> Result<String, String> {
    let all_users = USERS.write().unwrap().clone();

    if let Some(idx) = all_users.iter().position(|user| user.id == id) {
        USERS.write().unwrap().remove(idx);
        Ok("User removed successfully".to_string())
    }
    else {
        Err("No such user found to delete".to_string())
    }
}