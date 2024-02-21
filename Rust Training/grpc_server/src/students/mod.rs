use tonic::{Response, Request, Status};
use student::{GetStudentRequest, GetAllStudentRequest, AddStudentRequest, StudentResponse};
use self::student::{student_server::Student, DeleteStudentRequest, UpdateStudentRequest};
use models::StudentJson;
use services::{create_student, read_student, read_all_student, update_student, delete_student};

pub mod services;
pub mod models;

pub mod student {
    tonic::include_proto!("student");
}

#[derive(Debug, Default)]
pub struct MyStudent {}


#[tonic::async_trait]
impl Student for MyStudent {
    async fn get_student(&self, request: Request<GetStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        let user_id = request.into_inner().id;

        match read_student(user_id).await {
            Ok(student) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "Success fetching the user".to_string(),
                    data: serde_json::to_string(&student).unwrap()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "No such user found".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }

        // let student_data = StudentData {
        //     id: student.id,
        //     name: student.name,
        //     phone: student.phone,
        //     email: student.email,
        //     city: student.city,
        //     address: student.address,
        //     marks: student.marks,
        //     percentage: student.percentage.unwrap(),
        //     grade: student.grade.unwrap()
        // };
    }


    async fn get_all_student(&self, request: Request<GetAllStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        match read_all_student().await {
            Ok(student) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "Success fetching the user".to_string(),
                    data: serde_json::to_string(&student).unwrap()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "No such user found".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn add_student(&self, request: Request<AddStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        let student = request.into_inner();

        let student_data = StudentJson {
            id: student.id,
            name: student.name,
            phone: student.phone,
            email: student.email,
            city: student.city,
            address: student.address,
            marks: student.marks,
            percentage: Some(student.percentage),
            grade: Some(student.grade)
        };

        match create_student(student_data).await {
            Ok(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "Success adding the new user".to_string(),
                    data: "".to_string()
                };

                Ok(Response::new(response))
            },
            Err(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "Conflicting user".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn update_student(&self, request: Request<UpdateStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        let student = request.into_inner();

        let student_data = StudentJson {
            id: student.id,
            name: student.name,
            phone: student.phone,
            email: student.email,
            city: student.city,
            address: student.address,
            marks: student.marks,
            percentage: Some(student.percentage),
            grade: Some(student.grade)
        };

        match update_student(student_data).await {
            Ok(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "Success updating the user".to_string(),
                    data: "".to_string()
                };

                Ok(Response::new(response))
            },
            Err(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "New user added instead of updating".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn delete_student(&self, request: Request<DeleteStudentRequest>) -> Result<Response<StudentResponse>, Status> {
        let user_id = request.into_inner().id;

        match delete_student(user_id).await {
            Ok(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "Success deleting the user".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = StudentResponse {
                    status: 2000,
                    message: "No such user found to delete".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }

}
