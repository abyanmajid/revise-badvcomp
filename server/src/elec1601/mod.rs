use serde_json::{json, Value};
use tracing::trace;

#[path = "01-base_encoding.rs"]
pub mod topic_1;

pub fn generate_problem(id: u32) -> Option<Value> {
    trace!("Generating problem for id: {}", id);
    match id {
        1 => {
            let (question, answer) = topic_1::base_encoding().unwrap();
            trace!("Problem generated successfully.");
            Some(json!({
                "question": question,
                "answer": answer
            }))
        }
        _ => {
            trace!("Unsupported id: {}", id);
            None
        }
    }
}
