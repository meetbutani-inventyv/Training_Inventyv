use std::{fs, collections::VecDeque, sync::{Arc, RwLock}, thread, time::Duration};
use common_modules::{Request, User};
use lazy_static::lazy_static;

mod common_modules;
mod request_generator;
mod request_seperator;
mod delete_requests;
mod status_updater;
mod skills_updater;


lazy_static! {
    #[derive(Debug, Clone, Copy)]
    static ref USERS: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(load_user_data()));
}

fn load_user_data() -> Vec<User> {
    let content = match fs::read_to_string("src/MasterData.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error loading user data: {}", err);
            String::new()
        }
    };

    let user_data:Vec<User> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error deserializing user data: {}", err);
            Vec::new()
        }
    };

    user_data
}


pub fn main() {
    let user_ref1  = Arc::new(&USERS);
    let user_ref2  = Arc::new(&USERS);
    let user_ref3  = Arc::new(&USERS);

    let pending_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let pending_ref1: Arc<RwLock<VecDeque<Request>>> = Arc::clone(&pending_queue);
    let pending_ref2: Arc<RwLock<VecDeque<Request>>> = Arc::clone(&pending_queue);
   
    let call_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let call_ref1 = Arc::clone(&call_queue);
    let call_ref2 = Arc::clone(&call_queue);

    let chat_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let chat_ref1 = Arc::clone(&chat_queue);
    let chat_ref2 = Arc::clone(&chat_queue);



    let thread1 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        request_generator::gen_new_request(&pending_ref1);
    });


    let thread2 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        request_seperator::filter_requests(&pending_ref2, &call_ref1, &chat_ref1);
    });


    let thread3 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        delete_requests::delete_matched_requests(&user_ref1, &call_ref2, &chat_ref2);
    });


    let thread4 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(15));
        status_updater::update_user_status(&user_ref2);
    });


    let thread5 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(30));
        skills_updater::update_user_skills(&user_ref3);
    });


    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
    thread5.join().unwrap();
}