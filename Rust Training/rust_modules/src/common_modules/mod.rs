use serde::{Deserialize, Serialize};

/// Student structure to store student data
#[derive(Debug, Serialize, Deserialize)]
pub struct Student {
    pub name: String,
    pub phone: String,
    pub email: String,
    pub city: String,
    pub address: String,
    pub marks: Vec<u32>,
    pub percentage: Option<f64>,
    pub grade: Option<String>
}