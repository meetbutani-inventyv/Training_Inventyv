use tonic::{Response, Request, Status};
use user::{GetUserRequest, GetAllUserRequest, AddUserRequest, UserResponse};
use self::{models::Language, user::{user_server::User, DeleteUserRequest, UpdateUserRequest}};
use models::{UserJson, UserSkills, Status as UserStatus};
use services::{create_user, read_user, read_all_user, update_user, delete_user};

pub mod services;
pub mod models;

pub mod user {
    tonic::include_proto!("user");
}

#[derive(Debug, Default)]
pub struct MyUser {}


#[tonic::async_trait]
impl User for MyUser {
    async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<UserResponse>, Status> {
        let user_id = request.into_inner().id;

        match read_user(user_id).await {
            Ok(user) => {
                let response = UserResponse {
                    status: 2000,
                    message: "Success fetching the user".to_string(),
                    data: serde_json::to_string(&user).unwrap()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "No such user found".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn get_all_user(&self, request: Request<GetAllUserRequest>) -> Result<Response<UserResponse>, Status> {
        match read_all_user().await {
            Ok(user) => {
                let response = UserResponse {
                    status: 2000,
                    message: "Success fetching all the users".to_string(),
                    data: serde_json::to_string(&user).unwrap()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "No such users found to fetch".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn add_user(&self, request: Request<AddUserRequest>) -> Result<Response<UserResponse>, Status> {
        let user = request.into_inner();
        let skills: Vec<UserSkills> = user.skills.into_iter()
            .filter_map(|skill| match skill {
                0 => Some(UserSkills::CustomerService),
                1 => Some(UserSkills::ProblemSolving),
                2 => Some(UserSkills::ProductKnowledge),
                3 => Some(UserSkills::EffectiveCommunication),
                4 => Some(UserSkills::TimeManagement),
                5 => Some(UserSkills::Adaptability),
                6 => Some(UserSkills::TeamCollaboration),
                7 => Some(UserSkills::FeedbackAnalysis),
                8 => Some(UserSkills::ProactiveEngagement),
                9 => Some(UserSkills::TechnicalProficiency),
                10 => Some(UserSkills::CulturalSensitivity),
                11 => Some(UserSkills::Documentation),
                _ => Some(UserSkills::CustomerService),
            })
            .collect();

        let user_data = UserJson {
            id: user.id,
            name: user.name,
            skills,
            status: match user.status {
                0 => UserStatus::Online,
                1 => UserStatus::Offline,
                _ => UserStatus::Online
            },
            language: match user.language {
                0 => Language::English,
                1 => Language::Spanish,
                _ => Language::English
            }
        };

        match create_user(user_data).await {
            Ok(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "Success adding the new user".to_string(),
                    data: "".to_string()
                };

                Ok(Response::new(response))
            },
            Err(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "Conflicting user".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn update_user(&self, request: Request<UpdateUserRequest>) -> Result<Response<UserResponse>, Status> {
        let user = request.into_inner();
        let skills: Vec<UserSkills> = user.skills.into_iter()
            .filter_map(|skill| match skill {
                0 => Some(UserSkills::CustomerService),
                1 => Some(UserSkills::ProblemSolving),
                2 => Some(UserSkills::ProductKnowledge),
                3 => Some(UserSkills::EffectiveCommunication),
                4 => Some(UserSkills::TimeManagement),
                5 => Some(UserSkills::Adaptability),
                6 => Some(UserSkills::TeamCollaboration),
                7 => Some(UserSkills::FeedbackAnalysis),
                8 => Some(UserSkills::ProactiveEngagement),
                9 => Some(UserSkills::TechnicalProficiency),
                10 => Some(UserSkills::CulturalSensitivity),
                11 => Some(UserSkills::Documentation),
                _ => Some(UserSkills::CustomerService)
            })
            .collect();

        let user_data = UserJson {
            id: user.id,
            name: user.name,
            skills,
            status: match user.status {
                0 => UserStatus::Online,
                1 => UserStatus::Offline,
                _ => UserStatus::Online
            },
            language: match user.language {
                0 => Language::English,
                1 => Language::Spanish,
                _ => Language::English
            }
        };

        match update_user(user_data).await {
            Ok(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "Success updating the user".to_string(),
                    data: "".to_string()
                };

                Ok(Response::new(response))
            },
            Err(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "New user added instead of updating".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }


    async fn delete_user(&self, request: Request<DeleteUserRequest>) -> Result<Response<UserResponse>, Status> {
        let user_id = request.into_inner().id;

        match delete_user(user_id).await {
            Ok(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "Success deleting the user".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            },
            Err(_) => {
                let response = UserResponse {
                    status: 2000,
                    message: "No such user found to delete".to_string(),
                    data: "".to_string()
                };
            
                Ok(Response::new(response))
            }
        }
    }

}