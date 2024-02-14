use std::sync::{Arc, RwLock};
use super::common_modules::{Skills, User};
use rand::Rng;

pub fn update_user_skills(user_ref: &Arc<RwLock<Vec<User>>>) {
        
    let mut users = user_ref.write().unwrap();
    let idx = rand::thread_rng().gen_range(0..users.len());
    let skill_len: i32 = rand::thread_rng().gen_range(1..=4);
    let mut skills: Vec<Skills> = Vec::new();

    for _ in 0..skill_len {
        skills.push(match rand::thread_rng().gen_range(0..12) {
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
        })
    }

    users[idx].skills = skills;
    println!("(/) Skills of User{} updated", idx+1);
}