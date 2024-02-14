use super::{Request, LANGUAGE, LEVELS, LEVEL_TIME, SKILLS, TASKS};
use chrono::Utc;
use std::{collections::{HashMap, VecDeque}, fs, sync::{Arc, RwLock}};


pub fn modify_request_level(waiting_ref: &Arc<RwLock<HashMap<String, VecDeque<Request>>>>) {

    for t in 0..TASKS.len() {
        for s in 0..SKILLS.len() {
            for lan in 0..LANGUAGE.len() {
                for i in 0..LEVELS.len() {
                    let mut left_over: VecDeque<Request> = VecDeque::new();
                    let mut to_shift: VecDeque<Request> = VecDeque::new();
                    let key = format!("{}_{}_{}_{}", TASKS[t], SKILLS[s], LANGUAGE[lan], LEVELS[i]);
                    let mut deq = waiting_ref.read().unwrap().get(&key).unwrap().clone();

                    let min_time = LEVEL_TIME.get(LEVELS[i]).unwrap()[0];
                    let max_time = LEVEL_TIME.get(LEVELS[i]).unwrap()[1];

                    let new_level = if i < LEVELS.len()-1 {
                        i + 1
                    } else {
                        LEVELS.len()-1
                    };


                    while let Some(each_req) = deq.pop_front() {
                        let time_diff = Utc::now().signed_duration_since(each_req.time_stamp).num_seconds();
                        // let new_level = i+1;
                        
                        if min_time <= time_diff && time_diff < max_time && i!=new_level{
                            to_shift.push_back(each_req);
                        }
                        else {
                            left_over.push_back(each_req);
                        }
                    }

                    waiting_ref.write().unwrap().entry(key.clone()).and_modify(|queue| {
                        *queue = left_over.clone();
                    });


                    if to_shift.len() > 0 {
                        let new_key = format!("{}_{}_{}_{}", TASKS[t], SKILLS[s], LANGUAGE[lan], LEVELS[new_level]);
                        waiting_ref.write().unwrap().entry(new_key.clone()).and_modify(|queue| {
                            queue.append(&mut to_shift);
                        });
                        
                        println!("(^) Request moved from L{} to L{}", i+1, new_level+1);
                    }
                }
            }
        }
    }


    let waiting_data = waiting_ref.read().unwrap();
    match serde_json::to_string_pretty(&*waiting_data) {
        Ok(w_req) => {
            let _ = fs::write("src/temp_output.json", w_req);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

}