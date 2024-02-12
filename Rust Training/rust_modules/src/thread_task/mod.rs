use rand::Rng;
use chrono::{DateTime, Utc};
use std::sync::{Arc, RwLock};
use std::{thread, time::Duration};

/// User structure to store the user details
#[derive(Debug, Clone)]
struct User {
    id: u32,
    name: String,
    time_stamp: DateTime<Utc>
}

/// Methods corresponding to User structure
impl User {
    fn new() -> User {
        let user = User {
            id: rand::thread_rng().gen_range(1001..10000),
            name: gen_new_username(),
            time_stamp: Utc::now()
        };
        user
    }
}

/// Function to generate a new random username
fn gen_new_username() -> String {
    let mut username = String::new();
    username.push(char::from((b'A' + (rand::random::<u8>() % 26)) as char));

    for _ in 0..rand::thread_rng().gen_range(4..12) {
        username.push(char::from((b'a' + (rand::random::<u8>() % 26)) as char));
    }  

    username
}


pub fn main() {
    let users: RwLock<Vec<User>> = RwLock::new(Vec::new());
    let arc: Arc<RwLock<Vec<User>>> = Arc::new(users);

    let ref1 = Arc::clone(&arc);
    let ref2 = Arc::clone(&arc);
    let ref3 = Arc::clone(&arc);


    // This thread is used to display the total user count at regular intervals of time
    let t1 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(4));
        
        match ref1.read() {
            Ok(r1) => println!("Total user count: {}", r1.len()),
            Err(err) => println!("Error: {}", err)
        }
    });


    // This thread is used to add a new user at regular intervals of time
    let t2 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(3));

        match ref2.write() {
            Ok(mut r2) => {
                r2.push(User::new());
                println!("A new User added!");
            },
            Err(err) => println!("Error: {}", err)
        }
    });


    // This thread is used to remove the old users whose time_stamp difference is more than 5 seconds
    let t3 = thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));

        match ref3.write() {
            Ok(mut r3) => 
                r3.retain(|user| { Utc::now() - user.time_stamp <= chrono::Duration::seconds(5) }
            ),
            Err(err) => println!("Error: {}", err)
        }

        println!("Success deleting old users!");
    });


    t1.join().unwrap();
    t2.join().unwrap();
    t3.join().unwrap();
}