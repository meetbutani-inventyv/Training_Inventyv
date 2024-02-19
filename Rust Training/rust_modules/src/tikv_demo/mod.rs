use std::fs;
use crate::common_modules::Student;
use crate::utils::{add_record, delete_record, get_record};


pub async fn main() {
    // Reading the student data from the json file
    let content = match fs::read_to_string("src/json_data/StudentData.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {}", err);
            "Error".to_string()
        }
    };
    

    // Storing student data in TiKV
    add_record("student".to_string(), content.to_string()).await;
    

    // Fetching all data from TiKV
    let value = get_record("student".to_string()).await;
    println!("--{:?}", value.as_str());
    let value:Vec<Student> = serde_json::from_str(value.as_str()).unwrap();
    println!("{:?}", value);


    // Deleting student data from TiKV
    delete_record("student".to_string()).await;
}