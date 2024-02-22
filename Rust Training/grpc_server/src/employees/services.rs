use std::{collections::VecDeque, fs, sync::{Arc, RwLock}};
use lazy_static::lazy_static;
use super::models::EmployeeJson;

lazy_static! {
    pub static ref EMPLOYEES: Arc<RwLock<VecDeque<EmployeeJson>>> = Arc::new(RwLock::new(load_employee_data()));
}


/// Function to read the employee data from the json file
fn load_employee_data() -> VecDeque<EmployeeJson> {
    let content = match fs::read_to_string("src/json_data/Employee_data.json") {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error loading employee data: {}", err);
            String::new()
        }
    };

    let user_data:VecDeque<EmployeeJson> = match serde_json::from_str(&content) {
        Ok(json_data) => json_data,
        Err(err) => {
            eprintln!("Error deserializing employee data: {}", err);
            VecDeque::new()
        }
    };

    user_data
}


/// Function to get the data of a employee
pub async fn read_employee(id: u32) -> Result<EmployeeJson, String> {
    let all_users = EMPLOYEES.write().unwrap();

    if let Some(idx) = all_users.iter().position(|emp: &EmployeeJson| emp.id == id) {
        Ok(all_users.get(idx).unwrap().clone())
    }
    else {
        Err("No such employee exists".to_string())
    }
}


/// Function to get the data of all employees
pub async fn read_all_employee() -> Result<VecDeque<EmployeeJson>, String> {
    let all_users = EMPLOYEES.write().unwrap();

    if all_users.len()>0 {
        Ok(all_users.clone())
    }
    else {
        Err("Error fetching all the employees".to_string())
    }
}


/// Function to add a new employee
pub async fn create_employee(employees: EmployeeJson) -> Result<String, String> {
    let all_users = EMPLOYEES.write().unwrap().clone();

    if all_users.iter().any(|emp| emp.id == employees.id) {
        Err("Conflicting employee id".to_string())
    }
    else {
        EMPLOYEES.write().unwrap().push_back(employees);
        Ok("New Employee added successfully".to_string())
    }
}


/// Function to update the employee data
pub async fn update_employee(employees: EmployeeJson) -> Result<String, String> {
    let mut all_users = EMPLOYEES.write().unwrap();

    if let Some(idx) = all_users.iter().position(|emp| emp.id == employees.id) {
        all_users[idx] = employees;
        Ok("Employee updated successfully".to_string())
    }
    else {
        EMPLOYEES.write().unwrap().push_back(employees);
        Err("New Employee added instead of updating".to_string())
    }
}


/// Function to delete a employee
pub async fn delete_employee(id: u32) -> Result<String, String> {
    let all_users = EMPLOYEES.write().unwrap().clone();

    if let Some(idx) = all_users.iter().position(|emp| emp.id == id) {
        EMPLOYEES.write().unwrap().remove(idx);
        Ok("Employee removed successfully".to_string())
    }
    else {
        Err("No such employee found to delete".to_string())
    }
}