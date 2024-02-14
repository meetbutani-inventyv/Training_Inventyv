use std::{collections::{HashMap, VecDeque}, fs, sync::{Arc, RwLock}, thread, time::Duration};
use common_modules::{Request, User};
use lazy_static::lazy_static;

mod common_modules;
mod request_generator;
mod request_seperator;
mod delete_requests;
mod status_updater;
mod skills_updater;
mod request_segregator;
mod monitoring_service;


lazy_static! {
    #[derive(Debug, Clone, Copy)]
    static ref USERS: Arc<RwLock<Vec<User>>> = Arc::new(RwLock::new(load_user_data()));

    static ref TASKS:Vec<&'static str> = vec!["Call", "Chat"];
    static ref SKILLS:Vec<&'static str> = vec!["CustomerService", "ProblemSolving", "ProductKnowledge", "EffectiveCommunication", "TimeManagement", "Adaptability", "TeamCollaboration", "FeedbackAnalysis", "ProactiveEngagement", "TechnicalProficiency", "CulturalSensitivity", "Documentation"];
    static ref STATUS:Vec<&'static str> = vec!["Online", "Offline"];
    static ref LANGUAGE:Vec<&'static str> = vec!["English", "Spanish"];
    static ref LEVELS:Vec<&'static str> = vec!["L1", "L2", "L3", "L4", "L5"];

    static ref LEVEL_TIME:HashMap<&'static str, Vec<i64>> = {
        let mut m = HashMap::new();
        m.insert("L1", vec![10,20]);
        m.insert("L2", vec![20,30]);
        m.insert("L3", vec![30,40]);
        m.insert("L4", vec![40,50]);
        m.insert("L5", vec![50,60]);
        m
    };
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
    let pending_ref3    : Arc<RwLock<VecDeque<Request>>> = Arc::clone(&pending_queue);
   
    let call_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let call_ref1 = Arc::clone(&call_queue);
    let call_ref2 = Arc::clone(&call_queue);

    let chat_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let chat_ref1 = Arc::clone(&chat_queue);
    let chat_ref2 = Arc::clone(&chat_queue);

    let waiting_queue: Arc<RwLock<HashMap<String, VecDeque<Request>>>> = Arc::new(RwLock::new(HashMap::new()));
    let waiting_ref1:Arc<RwLock<HashMap<String, VecDeque<Request>>>> = Arc::clone(&waiting_queue);
    let waiting_ref2:Arc<RwLock<HashMap<String, VecDeque<Request>>>> = Arc::clone(&waiting_queue);


    for t in 0..TASKS.len() {
        for s in 0..SKILLS.len() {
            for lan in 0..LANGUAGE.len() {
                for lev in 0..LEVELS.len() {
                    waiting_queue.write().unwrap().insert(format!("{}_{}_{}_{}", TASKS[t], SKILLS[s], LANGUAGE[lan], LEVELS[lev]), VecDeque::new());
                }
            }
        }
    }


    let thread1 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        request_generator::gen_new_request(&pending_ref1);
    });


    // let thread2 = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(2));
    //     request_seperator::filter_requests(&pending_ref2, &call_ref1, &chat_ref1);
    // });


    // let thread3 = thread::spawn(move || loop {
    //     thread::sleep(Duration::from_secs(2));
    //     delete_requests::delete_matched_requests(&user_ref1, &call_ref2, &chat_ref2);
    // });


    let thread4 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(15));
        status_updater::update_user_status(&user_ref2);
    });


    let thread5 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(30));
        skills_updater::update_user_skills(&user_ref3);
    });


    let thread6 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(1));
        request_segregator::split_requests(&pending_ref3, &waiting_ref1);
    });


    let thread7 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        monitoring_service::modify_request_level(&waiting_ref2);
    });


    thread1.join().unwrap();
    // thread2.join().unwrap();
    // thread3.join().unwrap();
    thread4.join().unwrap();
    thread5.join().unwrap();
    thread6.join().unwrap();
    thread7.join().unwrap();
}