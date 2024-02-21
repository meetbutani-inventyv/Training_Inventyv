use std::{collections::VecDeque, fs, sync::{Arc, RwLock}};
use lazy_static::lazy_static;
use super::models::StudentJson;

lazy_static! {
    pub static ref STUDENTS: Arc<RwLock<VecDeque<StudentJson>>> = Arc::new(RwLock::new(load_student_data()));
}


/// Function to read the student data from the json file
fn load_student_data() -> VecDeque<StudentJson> {
    let content = match fs::read_to_string("src/json_data/Student_data.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error loading user data: {}", err);
            String::new()
        }
    };

    let user_data:VecDeque<StudentJson> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error deserializing user data: {}", err);
            VecDeque::new()
        }
    };

    user_data
}


/// Function to get the data of a student
pub async fn read_student(id: u32) -> Result<StudentJson, String> {
    let all_students = STUDENTS.write().unwrap();

    if let Some(idx) = all_students.iter().position(|user| user.id == id) {
        Ok(all_students.get(idx).unwrap().clone())
    }
    else {
        Err("No such student exists".to_string())
    }
}


/// Function to get the data of all students
pub async fn read_all_student() -> Result<VecDeque<StudentJson>, String> {
    let all_students = STUDENTS.read().unwrap();

    if all_students.len()>0 {
        Ok(all_students.clone())
    }
    else {    
        Err("Error fetching all the students".to_string())
    }
}


/// Function to add a new student
pub async fn create_student(student: StudentJson) -> Result<String, String> {
    let all_students = STUDENTS.write().unwrap().clone();

    if all_students.iter().any(|user| user.id == student.id) {
        Err("Conflicting student id".to_string())
    }
    else {
        STUDENTS.write().unwrap().push_back(student);
        Ok("New Student added successfully".to_string())
    }
}


/// Function to update the student data
pub async fn update_student(student: StudentJson) -> Result<String, String> {
    let mut all_students = STUDENTS.write().unwrap();

    if let Some(user) = all_students.iter_mut().find(|user| user.id == student.id) {
        *user = student.clone();
        Ok("Student updated successfully".to_string())
    }
    else {
        STUDENTS.write().unwrap().push_back(student);
        Err("New Student added instead of updating".to_string())
    }
}


/// Function to delete a student
pub async fn delete_student(id: u32) -> Result<String, String> {
    let all_students = STUDENTS.write().unwrap().clone();

    if let Some(idx) = all_students.iter().position(|user| user.id == id) {
        STUDENTS.write().unwrap().remove(idx);
        Ok("Student removed successfully".to_string())
    }
    else {
        Err("No such student found to delete".to_string())
    }
}