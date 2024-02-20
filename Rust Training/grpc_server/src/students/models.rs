use serde::{Serialize, Deserialize};


/// Student structure to store student data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StudentJson {
    pub id: u32,
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
    pub percentage: Option<f64>,
    pub grade: Option<String>
}
