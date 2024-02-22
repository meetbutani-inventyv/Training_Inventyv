use serde::{Serialize, Deserialize};


/// Employee structure to store employee data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmployeeJson {
    pub id: u32,
    pub name: String,
    pub age: u32,
    pub skills: Vec<Skills>,
    pub position: Option<Position>,
    #[serde(rename(serialize = "experiance(y)", deserialize = "experiance(y)"))]
    pub experiance: Option<u32>
}


/// Position enum to store a employee's position
#[derive(Debug, Serialize, Deserialize, Clone)]
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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Skills {
    C,
    #[serde(rename="C#")]
    CS,
    Rust,
    Java,
    Python
}