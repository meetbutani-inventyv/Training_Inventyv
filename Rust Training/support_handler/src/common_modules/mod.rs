use rand::Rng;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Status {
    Online,
    Offline
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum TaskType {
    Call,
    Chat
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Language {
    English,
    Spanish
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum Skills {
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


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub skills: Vec<Skills>,
    pub status: Status,
    pub language: Language
}


#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub struct Request {
    pub task_type: TaskType,
    pub skills: Skills,
    pub language: Language,
    pub time_stamp: chrono::DateTime<Utc>
}

impl Request {
    pub fn new() -> Request {
        let request = Request {
            task_type: match rand::thread_rng().gen_range(0..=2) {
                0 => TaskType::Call,
                1 => TaskType::Chat,
                _ => TaskType::Chat
            },
            skills: match rand::thread_rng().gen_range(0..12) {
                0 => Skills::CustomerService,
                1 => Skills::ProblemSolving,
                2 => Skills::ProductKnowledge,
                3 => Skills::EffectiveCommunication,
                4 => Skills::TimeManagement,
                5 => Skills::Adaptability,
                6 => Skills::TeamCollaboration,
                7 => Skills::FeedbackAnalysis,
                8 => Skills::ProactiveEngagement,
                9 => Skills::TechnicalProficiency,
                10 => Skills::CulturalSensitivity,
                11 => Skills::Documentation,
                _ => Skills::CustomerService
            },
            language: match rand::thread_rng().gen_range(0..2) {
                0 => Language::English,
                1 => Language::Spanish,
                _ => Language::English
            },
            time_stamp: Utc::now()
        };
        request
    }
}