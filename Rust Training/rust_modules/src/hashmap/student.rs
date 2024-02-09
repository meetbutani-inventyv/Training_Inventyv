use std::fs;
use serde_json::{json, Value};
use std::collections::HashMap;
use crate::common_modules::Student;


pub fn main() {
    // Reading student data from the json file
    let file_content = match fs::read_to_string("src/json_data/StudentData.json") {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };


    // Deserializing the employee data
    let student_data: Vec<Student> = match serde_json::from_str(&file_content) {
        Ok(data) => {
            data
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    // println!("{:?}", student_data);


    let mut student_map = Vec::new();

    for each_student in &student_data {
        let mut student: HashMap<&str, Value> = HashMap::new();

        student.insert("name", Value::String(each_student.name.to_string()));
        student.insert("phone", Value::String(each_student.phone.to_string()));
        student.insert("email", Value::String(each_student.email.to_string()));
        student.insert("city", Value::String(each_student.city.to_string()));
        student.insert("marks", json!(each_student.marks));

    
        let overall_marks:u32 = each_student.marks.iter().sum();
        let percentage = (overall_marks as f64) / (each_student.marks.len() as f64); 
        student.insert("percentage", json!(percentage));


        student.insert("grade", if percentage >= 90.0 {
                Value::String("A".to_string())
            } else if percentage >= 80.0 {
                Value::String("B".to_string())
            } else if percentage >= 70.0 {
                Value::String("C".to_string())
            } else if percentage >= 60.0 {
                Value::String("D".to_string())
            } else {
                Value::String("F".to_string())
            }
        );

        student_map.push(student);
    }


    // Serializing the Student data and storing it in a json file
    match serde_json::to_string_pretty(&student_map) {
        Ok(student_map) => {
            let _ = fs::write("src/json_data/student_Hashmap.json", student_map);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

}
