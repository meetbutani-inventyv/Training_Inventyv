use std::{collections::VecDeque, sync::{Arc, RwLock}};
use super::common_modules::{User, Request, Status};


pub fn delete_matched_requests(user_ref: &Arc<RwLock<Vec<User>>>, call_ref: &Arc<RwLock<VecDeque<Request>>>, chat_ref: &Arc<RwLock<VecDeque<Request>>>) {

    let mut to_remove = None;
    for (idx, call_req) in call_ref.write().unwrap().iter_mut().enumerate() {
        let user_online = user_ref.read().unwrap().iter().any(|user|
                user.status==Status::Online && user.skills.contains(&call_req.skills) && user.language==call_req.language
        );
            
        if user_online {
            to_remove = Some(idx);
            break;
        }
    }
    if let Some(idx) = to_remove {
        println!("Call deleted: {:?}", call_ref.read().unwrap()[idx]);
        call_ref.write().unwrap().remove(idx);
    }


    let mut to_remove = None;
    for (idx, chat_req) in chat_ref.write().unwrap().iter_mut().enumerate() {
        let user_online = user_ref.read().unwrap().iter().any(|user|
            user.status==Status::Online && user.skills.contains(&chat_req.skills) && user.language==chat_req.language
        );
        
        if user_online {
            to_remove = Some(idx);
            break;
        }
    }
    if let Some(idx) = to_remove {
        println!("Chat deleted: {:?}", chat_ref.read().unwrap()[idx]);
        chat_ref.write().unwrap().remove(idx);
    }

}