use tonic::transport::Server;
use students::MyStudent;
use users::MyUser;
use students::student::student_server::StudentServer;
use users::user::user_server::UserServer;

mod students;
mod users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let student_service = MyStudent::default();
    let user_service = MyUser::default();

    println!("Server listening on {}", addr);
    let _ = Server::builder()
                .add_service(StudentServer::new(student_service))
                .add_service(UserServer::new(user_service))
                .serve(addr)
                .await;

    Ok(())
}