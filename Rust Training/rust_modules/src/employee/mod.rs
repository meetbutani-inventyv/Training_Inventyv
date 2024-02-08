use std::fs;
use crate::common_modules::{Employee, Position, Skills};


pub fn main() {
    // Defining 3 empty vectors to store data of 3 fdifferent types of employees
    let mut employee_type1: Vec<&Employee> = Vec::new();
    let mut employee_type2: Vec<&Employee> = Vec::new();
    let mut employee_type3: Vec<&Employee> = Vec::new();


    // Reading employee data from the json file
    let file_content = match fs::read_to_string("src/json_data/EmployeeData.json") {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    // Deserializing the employee data
    let employee_data: Vec<Employee> = match serde_json::from_str(&file_content) {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    // println!("{:?}", employee_data);


    // Filtering the employee data into 3 different categories
    for i in 0..employee_data.len() {
        if employee_data[i].position==Some(Position::SoftwareDeveloper) && employee_data[i].skills.contains(&Skills::Rust) {
            employee_type1.push(&employee_data[i]);
        }

        if employee_data[i].position==Some(Position::JrSoftwareDeveloper) && employee_data[i].skills.contains(&Skills::Java) {
            employee_type2.push(&employee_data[i]);
        }

        if employee_data[i].position==Some(Position::SrSoftwareDeveloper) || employee_data[i].skills.contains(&Skills::C) {
            employee_type3.push(&employee_data[i]);
        }
    }


    // Serializing the employee Type-1 data and writing it to a json file
    match serde_json::to_string_pretty(&employee_type1) {
        Ok(employee_type1) => {
            let _ = fs::write("src/json_data/employee_type1.json", employee_type1);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    // Serializing the employee Type-2 data and writing it to a json file
    match serde_json::to_string_pretty(&employee_type2) {
        Ok(employee_type2) => {
            let _ = fs::write("src/json_data/employee_type2.json", employee_type2);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    // Serializing the employee Type-3 data and writing it to a json file
    match serde_json::to_string_pretty(&employee_type3) {
        Ok(employee_type3) => {
            let _ = fs::write("src/json_data/employee_type3.json", employee_type3);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    
}