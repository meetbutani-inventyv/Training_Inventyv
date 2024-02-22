use tonic::{Response, Request, Status};
use employee::{GetEmployeeRequest, GetAllEmployeeRequest, AddEmployeeRequest, EmployeeResponse};
use self::employee::{employee_server::Employee, DeleteEmployeeRequest, UpdateEmployeeRequest};
use models::{EmployeeJson, Skills, Position};
use services::{create_employee, read_employee, read_all_employee, update_employee, delete_employee};

pub mod services;
pub mod models;

pub mod employee {
    tonic::include_proto!("employee");
}

#[derive(Debug, Default)]
pub struct MyEmployee {}


#[tonic::async_trait]
impl Employee for MyEmployee {
    async fn get_employee(&self, request: Request<GetEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        let employee_id = request.into_inner().id;

        match read_employee(employee_id).await {
            Ok(employee) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "Success fetching the employees".to_string(),
                    data: serde_json::to_string(&employee).unwrap()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "No such employee found".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn get_all_employee(&self, request: Request<GetAllEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        match read_all_employee().await {
            Ok(employee) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "Success fetching all the employees".to_string(),
                    data: serde_json::to_string(&employee).unwrap()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "No such employees found to fetch".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn add_employee(&self, request: Request<AddEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        let emp = request.into_inner();
        let skills: Vec<Skills> = emp.skills.into_iter()
            .filter_map(|skill| match skill {
                0 => Some(Skills::C),
                1 => Some(Skills::CS),
                2 => Some(Skills::Rust),
                3 => Some(Skills::Java),
                4 => Some(Skills::Python),
                _ => Some(Skills::C)
            })
            .collect();

        let employee_data = EmployeeJson {
            id: emp.id,
            name: emp.name,
            age: emp.age,
            skills,
            position: match emp.position {
                0 => Some(Position::SoftwareDeveloper),
                1 => Some(Position::JrSoftwareDeveloper),
                2 => Some(Position::SrSoftwareDeveloper),
                3 => Some(Position::TeamLead),
                4 => Some(Position::ProjectManager),
                _ => Some(Position::JrSoftwareDeveloper)
            },
            experiance: Some(emp.experiance)
        };

        match create_employee(employee_data).await {
            Ok(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "Success adding the new employee".to_string(),
                    data: "".to_string()
                };

                Ok(Response::new(response))
            },
            Err(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "Conflicting employee".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn update_employee(&self, request: Request<UpdateEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        let emp = request.into_inner();
        let skills: Vec<Skills> = emp.skills.into_iter()
            .filter_map(|skill| match skill {
                0 => Some(Skills::C),
                1 => Some(Skills::CS),
                2 => Some(Skills::Rust),
                3 => Some(Skills::Java),
                4 => Some(Skills::Python),
                _ => Some(Skills::C)
            })
            .collect();

        let employee_data = EmployeeJson {
            id: emp.id,
            name: emp.name,
            age: emp.age,
            skills,
            position: match emp.position {
                0 => Some(Position::SoftwareDeveloper),
                1 => Some(Position::JrSoftwareDeveloper),
                2 => Some(Position::SrSoftwareDeveloper),
                3 => Some(Position::TeamLead),
                4 => Some(Position::ProjectManager),
                _ => Some(Position::JrSoftwareDeveloper)
            },
            experiance: Some(emp.experiance)
        };

        match update_employee(employee_data).await {
            Ok(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "Success updating the employee".to_string(),
                    data: "".to_string()
                };

                Ok(Response::new(response))
            },
            Err(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "New employee added instead of updating".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn delete_employee(&self, request: Request<DeleteEmployeeRequest>) -> Result<Response<EmployeeResponse>, Status> {
        let employee_id = request.into_inner().id;

        match delete_employee(employee_id).await {
            Ok(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "Success deleting the employee".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = EmployeeResponse {
                    status: 2000,
                    message: "No such employee found to delete".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }

}