use std::{collections::VecDeque, sync::{Arc, RwLock}};
use super::common_modules::{Request, TaskType};


pub fn filter_requests(pending_ref: &Arc<RwLock<VecDeque<Request>>>, call_ref: &Arc<RwLock<VecDeque<Request>>>, chat_ref: &Arc<RwLock<VecDeque<Request>>>) {
    pending_ref.write().unwrap().retain(|req| {
        match req.task_type {
            TaskType::Call => {
                call_ref.write().unwrap().push_back(req.clone());
                false
            }
            TaskType::Chat => {
                chat_ref.write().unwrap().push_back(req.clone());
                false
            }
        }
    });

    // println!("Call: {}  =  Chat: {}", call_ref.read().unwrap().len(), chat_ref.read().unwrap().len());
}