use std::sync::{Arc, RwLock};
use super::common_modules::{User, Status};
use rand::Rng;

pub fn update_user_status(user_ref: &Arc<RwLock<Vec<User>>>) {
    let mut users = user_ref.write().unwrap();
    let idx = rand::thread_rng().gen_range(0..users.len());

    users[idx].status = match rand::thread_rng().gen_range(0..=2) {
        0 => Status::Online,
        1 => Status::Offline,
        _ => Status::Online
    };

    println!("(\\) Status of User{} updated", idx+1);
}