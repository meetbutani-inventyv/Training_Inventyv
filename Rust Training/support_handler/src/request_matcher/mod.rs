use super::{LANGUAGE, LEVELS, SKILLS, TASKS, USERS};
use super::common_modules::{Request, Status};
use std::{collections::{HashMap, VecDeque}, sync::{Arc, RwLock}};

pub fn delete_matched_requests(waiting_ref: &Arc<RwLock<HashMap<String, VecDeque<Request>>>>) {
    for t in 0..TASKS.len() {
        for s in 0..SKILLS.len() {
            for lan in 0..LANGUAGE.len() {
                for lev in 0..LEVELS.len() {
                    let key = format!("{}_{}_{}_{}", TASKS[t], SKILLS[s], LANGUAGE[lan], LEVELS[LEVELS.len()-lev-1]);
                    let mut deq = waiting_ref.read().unwrap().get(&key.clone()).unwrap().clone();
                    let mut left_over: VecDeque<Request> = VecDeque::new();
                    
                    while let Some(each_req) = deq.pop_front() {
                        let user_data  = USERS.read().unwrap();
                        let mut found = false;

                        for u in 0..user_data.len() {
                            let user = user_data.get(u).unwrap().clone();

                            if user.language==each_req.language && user.skills.contains(&each_req.skills) && user.status==Status::Online {
                                println!("(-) Request served by {} and closed!", user.name);
                                found = true;
                                break;
                            }
                        }

                        if !found {
                            left_over.push_back(each_req);
                            break;
                        }
                    }

                    waiting_ref.write().unwrap().entry(key.clone()).and_modify(|queue| {
                        *queue = left_over.clone();
                    });
                }
            }
        }
    }
}