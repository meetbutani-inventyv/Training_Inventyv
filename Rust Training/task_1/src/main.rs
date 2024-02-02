use std::fs;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]

struct Student {
    name: String,
    phone: String,
    email: String,
    city: String,
    address: String,
    marks: Vec<u32>,
    percentage: Option<f64>,
    grade: Option<String>
}

fn main() {
    let content = match fs::read_to_string("./StudentData.json") {
        Ok(data) => data,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    
    let mut student_data:Vec<Student> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    // println!("{:#?}", student_data);

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

    // for student in &mut student_data {
    //     let overall_marks: u32 = student.marks.iter().sum();
    //     let percentage = (overall_marks as f64) / (student.marks.len() as f64); 
        
    //     student.percentage = Some(percentage);
    //     student.grade = if percentage >= 90.0 {
    //                         Some("A".to_string())
    //                     } else if percentage >= 80.0 {
    //                         Some("B".to_string())
    //                     } else if percentage >= 70.0 {
    //                         Some("C".to_string())
    //                     } else if percentage >= 60.0 {
    //                         Some("D".to_string())
    //                     } else {
    //                         Some("F".to_string())
    //                     };

        // println!("{} has scored {} marks and percentage of {}", student.name, overall_marks, percentage);
    // }

    // println!("{:#?}", student_data);

    let mut output_file = match fs::File::create("./newStudentData.json") {
        Ok(file) => file,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };
    
    let serialized_data = match serde_json::to_string_pretty(&student_data) {
        Ok(data) => data,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let _ = output_file.write_all(serialized_data.as_bytes());

}
