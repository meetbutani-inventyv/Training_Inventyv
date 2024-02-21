use serde::{Serialize, Deserialize};


/// User structure to store the user data
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserJson {
    pub id: u32,
    pub name: String,
    pub skills: Vec<UserSkills>,
    pub status: Status,
    pub language: Language
}

/// Status enum to store whether user is Online or Offline
#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Status {
    Online,
    Offline
}

/// Language enum to store the language which the user speaks
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Language {
    English,
    Spanish
}

/// UserSkills enum to store the skills of an user
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum UserSkills {
    #[serde(rename="Customer Service")]
    CustomerService,
    #[serde(rename="Problem-solving")]
    ProblemSolving,
    #[serde(rename="Product Knowledge")]
    ProductKnowledge,
    #[serde(rename="Effective Communication")]
    EffectiveCommunication,
    #[serde(rename="Time Management")]
    TimeManagement,
    Adaptability,
    #[serde(rename="Team Collaboration")]
    TeamCollaboration,
    #[serde(rename="Feedback Analysis")]
    FeedbackAnalysis,
    #[serde(rename="Proactive Engagement")]
    ProactiveEngagement,
    #[serde(rename="Technical Proficiency")]
    TechnicalProficiency,
    #[serde(rename="Cultural Sensitivity")]
    CulturalSensitivity,
    Documentation,
}