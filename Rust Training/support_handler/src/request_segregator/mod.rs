use super::common_modules::{Request, Language, Skills, TaskType};
use std::{collections::{HashMap, VecDeque}, fs, sync::{Arc, RwLock}};

pub fn split_requests(pending_ref: &Arc<RwLock<VecDeque<Request>>>, waiting_ref: &Arc<RwLock<HashMap<String, VecDeque<Request>>>>) {

    if pending_ref.read().unwrap().len() > 0 {
        let req = pending_ref.write().unwrap().pop_front().unwrap();
        let task = match req.task_type {
            TaskType::Call => "Call",
            TaskType::Chat => "Chat"
        };
        let skls = match req.skills {
            Skills::CustomerService => "CustomerService",
            Skills::ProblemSolving => "ProblemSolving",
            Skills::ProductKnowledge => "ProductKnowledge",
            Skills::EffectiveCommunication => "EffectiveCommunication",
            Skills::TimeManagement => "TimeManagement",
            Skills::Adaptability => "Adaptability",
            Skills::TeamCollaboration => "TeamCollaboration",
            Skills::FeedbackAnalysis => "FeedbackAnalysis",
            Skills::ProactiveEngagement => "ProactiveEngagement",
            Skills::TechnicalProficiency => "TechnicalProficiency",
            Skills::CulturalSensitivity => "CulturalSensitivity",
            Skills::Documentation => "Documentation"
        };
        let lang = match req.language {
            Language::English => "English",
            Language::Spanish => "Spanish"
        };

        
        let k = format!("{}_{}_{}_{}", task, skls, lang, "L1");
        waiting_ref.write().unwrap().entry(k).and_modify(|queue| {
            queue.push_back(req);
        });
        
        
        // let waiting_data = waiting_ref.read().unwrap();
        // match serde_json::to_string_pretty(&*waiting_data) {
        //     Ok(w_req) => {
        //         let _ = fs::write("src/support_handler/waiting_request.json", w_req);
        //     }
        //     Err(err) => {
        //         println!("Error: {}", err);
        //         return;
        //     }
        // };
    }

}