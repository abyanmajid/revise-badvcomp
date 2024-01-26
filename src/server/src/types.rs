use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionType {
    pub id: u32,
    pub qtype: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Topic {
    pub id: u32,
    pub topic: String,
    pub question_types: Vec<QuestionType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub unit: String,
    pub syllabus: String,
    pub topics: Vec<Topic>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub question: String,
    pub answer: String,
}

#[derive(Serialize, Deserialize)]
pub struct UnitSummary {
    pub unit: &'static str,
    pub topics: usize,
}
