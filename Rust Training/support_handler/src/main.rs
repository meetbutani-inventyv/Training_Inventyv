use std::{collections::VecDeque, sync::{Arc, RwLock}, thread, time::Duration};
use common_modules::Request;

mod common_modules;
mod request_generator;
mod request_seperator;


pub fn main() {
    let pending_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let pending_ref1: Arc<RwLock<VecDeque<Request>>> = Arc::clone(&pending_queue);
    let pending_ref2: Arc<RwLock<VecDeque<Request>>> = Arc::clone(&pending_queue);
   
    let call_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let call_ref1 = Arc::clone(&call_queue);

    let chat_queue: Arc<RwLock<VecDeque<Request>>> = Arc::new(RwLock::new(VecDeque::new()));
    let chat_ref1 = Arc::clone(&chat_queue);



    let thread1 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        request_generator::gen_new_request(&pending_ref1);
    });


    let thread2 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(2));
        request_seperator::filter_requests(&pending_ref2, &call_ref1, &chat_ref1);
    });


    thread1.join().unwrap();
    thread2.join().unwrap();
}