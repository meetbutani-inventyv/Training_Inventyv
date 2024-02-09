
use std::fs;
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::common_modules::{Employee, Position,Skills};


pub fn main() {
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


    // Creating three vectors to store the different types of employees
    let mut employee_map1 = Vec::new();
    let mut employee_map2 = Vec::new();
    let mut employee_map3 = Vec::new();


    for employee in &employee_data {
        if employee.position==Some(Position::SoftwareDeveloper) && employee.skills.contains(&Skills::Rust) {
            let mut emp: HashMap<&str, Value> = HashMap::new();

            emp.insert("name", json!(employee.name));
            emp.insert("age", json!(employee.age));
            emp.insert("skills", json!(employee.skills));

            if !employee.position.is_none() {
                emp.insert("position", json!(employee.position));
            }
            if !employee.experiance.is_none() {
                emp.insert( "experiance(y)", json!(employee.experiance));
            }

            employee_map1.push(emp);
        }


        if employee.position==Some(Position::JrSoftwareDeveloper) && employee.skills.contains(&Skills::Java) {
            let mut emp: HashMap<&str, Value> = HashMap::new();

            emp.insert("name", json!(employee.name));
            emp.insert("age", json!(employee.age));
            emp.insert("skills", json!(employee.skills));

            if !employee.position.is_none() {
                emp.insert("position", json!(employee.position));
            }
            if !employee.experiance.is_none() {
                emp.insert( "experiance(y)", json!(employee.experiance));
            }

            employee_map2.push(emp);
        }


        if employee.position==Some(Position::SrSoftwareDeveloper) || employee.skills.contains(&Skills::C) {
            let mut emp: HashMap<&str, Value> = HashMap::new();

            emp.insert("name", json!(employee.name));
            emp.insert("age", json!(employee.age));
            emp.insert("skills", json!(employee.skills));

            if !employee.position.is_none() {
                emp.insert("position", json!(employee.position));
            }
            if !employee.experiance.is_none() {
                emp.insert( "experiance(y)", json!(employee.experiance));
            }

            employee_map3.push(emp);
        }
    }


    // Serializing the Employee type-1 data and storing it in a json file
    match serde_json::to_string_pretty(&employee_map1) {
        Ok(employee_map1) => {
            let _ = fs::write("src/json_data/employee_map1.json", employee_map1);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    // Serializing the Employee type-2 data and storing it in a json file
    match serde_json::to_string_pretty(&employee_map2) {
        Ok(employee_map2) => {
            let _ = fs::write("src/json_data/employee_map2.json", employee_map2);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    // Serializing the Employee type-3 data and storing it in a json file
    match serde_json::to_string_pretty(&employee_map3) {
        Ok(employee_map3) => {
            let _ = fs::write("src/json_data/employee_map3.json", employee_map3);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };   
    
}