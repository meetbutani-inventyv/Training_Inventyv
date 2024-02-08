use std::fs;
use crate::common_modules::Student;

pub fn main() {
    // Reading the student data from the json file
    let content = match fs::read_to_string("src/json_data/StudentData.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    
    // Deserializing the student data
    let mut student_data:Vec<Student> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    println!("{:#?}", student_data);


    // Calculating student marks and grade and adding it to student's data
    for i in 0..student_data.len() {
        let overall_marks: u32 = student_data[i].marks.iter().sum();
        let percentage = (overall_marks as f64) / (student_data[i].marks.len() as f64); 
        
        student_data[i].percentage = Some(percentage);
        student_data[i].grade = if percentage >= 90.0 {
                                    Some("A".to_string())
                                } else if percentage >= 80.0 {
                                    Some("B".to_string())
                                } else if percentage >= 70.0 {
                                    Some("C".to_string())
                                } else if percentage >= 60.0 {
                                    Some("D".to_string())
                                } else {
                                    Some("F".to_string())
                                };
    }
    
    
    // Serializing the student data and storing it in a json file
    match serde_json::to_string_pretty(&student_data) {
        Ok(data) => {
            let _ = fs::write("src/json_data/newStudentData.json", data);
        }
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

}