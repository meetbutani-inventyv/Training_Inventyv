use tonic::transport::Server;
use students::MyStudent;
use students::student::student_server::StudentServer;

mod students;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse().unwrap();
    let student_service = MyStudent::default();

    println!("Server listening on {}", addr);
    let _ = Server::builder()
                .add_service(StudentServer::new(student_service))
                .serve(addr)
                .await;

    Ok(())
}