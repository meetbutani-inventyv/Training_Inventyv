use std::{collections::VecDeque, sync::{Arc, RwLock}};
use super::Request;

pub fn gen_new_request(pending_ref: &Arc<RwLock<VecDeque<Request>>>) {
    pending_ref.write().unwrap().push_back(Request::new());
    // println!("{:?}", pending_ref.read().unwrap());

    println!("New request added(+)");
}