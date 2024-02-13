use std::{collections::VecDeque, sync::{Arc, RwLock}, thread, time::Duration};
use common_modules::Request;

mod common_modules;
mod request_generator;



pub fn main() {
    let pending_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let pending_ref1: Arc<RwLock<VecDeque<Request>>> = Arc::clone(&pending_queue);
   
    let thread1 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        request_generator::gen_new_request(&pending_ref1);
    });

    thread1.join().unwrap();
}