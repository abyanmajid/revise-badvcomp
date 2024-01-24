use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: u32,
    pub topic: String, // Changed from &'static str to String
}

#[derive(Serialize, Deserialize)]
pub struct Unit {
    pub unit: String, // Changed from &'static str to String
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
