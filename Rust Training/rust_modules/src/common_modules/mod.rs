use serde;
use serde::{Deserialize, Serialize};


// ============================= Student task ============================= 

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



// ============================= Employee task ============================= 

/// Employee structure to store employee data
#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub age: u8,
    pub skills: Vec<Skills>,
    pub position: Option<Position>,
    #[serde(rename(serialize = "experiance(y)", deserialize = "experiance(y)"))]
    pub experiance: Option<u8>
}


/// Position enum to store a employee's position
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Position {
    #[serde(rename="Software Developer")]
    SoftwareDeveloper,
    #[serde(rename="Jr. Software Developer")]
    JrSoftwareDeveloper,
    #[serde(rename="Sr. Software Developer")]
    SrSoftwareDeveloper,
    #[serde(rename="Team Lead")]
    TeamLead,
    #[serde(rename="Project Manager")]
    ProjectManager
}


/// Skills enum to store skills of a person
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Skills {
    C,
    #[serde(rename="C#")]
    CS,
    Rust,
    Java,
    Python
}