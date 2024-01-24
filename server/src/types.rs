use crate::constants::TOPICS_COUNT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Topic {
    pub id: u32,
    pub topic: &'static str,
}

#[derive(Serialize, Deserialize)]
pub struct Unit {
    pub unit: &'static str,
    pub topics: [Topic; TOPICS_COUNT],
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: &'static str,
}

#[derive(Serialize)]
pub struct Question {
    pub question: String,
    pub answer: String,
}
