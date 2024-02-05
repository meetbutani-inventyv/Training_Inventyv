use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Employee {
    name: String,
    age: u8,
    skills: Vec<String>,
    position: Option<String>,
    #[serde(rename(serialize = "experience(y)", deserialize = "experience(y)"))]
    experiance: Option<u8>
}


fn main() {
    let mut employee_type1: Vec<&Employee> = Vec::new();
    let mut employee_type2: Vec<&Employee> = Vec::new();
    let mut employee_type3: Vec<&Employee> = Vec::new();

    let file_content = match fs::read_to_string("Employee.json") {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


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


    for i in 0..employee_data.len() {
        if employee_data[i].position==Some("Software Developer".to_string()) && employee_data[i].skills.contains(&"Rust".to_string()) {
            employee_type1.push(&employee_data[i]);
        }

        if employee_data[i].position==Some("Jr. Software Developer".to_string()) && employee_data[i].skills.contains(&"Java".to_string()) {
            employee_type2.push(&employee_data[i]);
        }

        if employee_data[i].position==Some("Sr. Software Developer".to_string()) || employee_data[i].skills.contains(&"C".to_string()) {
            employee_type3.push(&employee_data[i]);
        }
    }

    println!("Type-1: \n{:#?}", employee_type1);
    println!("Type-2: \n{:#?}", employee_type2);
    println!("Type-3: \n{:#?}", employee_type3);


    match serde_json::to_string_pretty(&employee_type1) {
        Ok(employee_type1) => {
            let _ = fs::write("employee_type1.json", employee_type1);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    match serde_json::to_string_pretty(&employee_type2) {
        Ok(employee_type2) => {
            let _ = fs::write("employee_type2.json", employee_type2);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    match serde_json::to_string_pretty(&employee_type3) {
        Ok(employee_type3) => {
            let _ = fs::write("employee_type3.json", employee_type3);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    
}
