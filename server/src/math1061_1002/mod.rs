use super::randomize;
use anyhow::{Error, Result};
use maplit::hashmap;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::trace;

#[path = "01-complex_numbers.rs"]
pub mod topic_1;

type FuncType = fn() -> Result<(String, String), Error>;

pub fn generate_problem(mut topic_id: u32, subtopic_id: u32) -> Option<Value> {
    trace!(
        "Entering generate_problem with topic_id: {}, subtopic_id: {}",
        topic_id,
        subtopic_id
    );

    if subtopic_id == 0 {
        trace!("Generating RANDOM problem for topic id {}", topic_id);
    } else {
        trace!(
            "Generating specific problem for topic id: {}, sub-topic id: {}",
            topic_id,
            subtopic_id
        );
    };

    trace!("Creating topic_functions hashmap");
    let topic_functions: HashMap<u32, Vec<FuncType>> = hashmap! {
        1 => vec![
            topic_1::identify_parts_complex as FuncType,
            topic_1::add_complex as FuncType,
            topic_1::subtract_complex as FuncType,
            topic_1::multiply_complex as FuncType,
            topic_1::divide_complex as FuncType,
        ],
    };

    if topic_id == 0 {
        trace!("Selecting a random topic_id because the given topic_id is 0");
        let mut rng = rand::thread_rng(); // Create a new RNG for random selection.
        let topic_keys: Vec<&u32> = topic_functions.keys().collect(); // Collect keys to choose from.
        topic_id = **topic_keys.choose(&mut rng).expect("No topics available");
    }

    trace!("Looking up functions for topic_id: {}", topic_id);
    topic_functions.get(&topic_id).and_then(|functions| {
        trace!(
            "Found functions for topic_id: {}. Processing subtopic_id: {}",
            topic_id,
            subtopic_id
        );
        let result = match subtopic_id {
            0 => {
                trace!(
                    "Randomizing selection of functions for topic_id: {}",
                    topic_id
                );
                randomize(functions).ok()
            }
            subtopic => {
                trace!(
                    "Selecting specific function for topic_id: {}, subtopic_id: {}",
                    topic_id,
                    subtopic
                );
                functions
                    .get(subtopic as usize - 1)
                    .map(|&func| {
                        trace!(
                            "Executing function for topic_id: {}, subtopic_id: {}",
                            topic_id,
                            subtopic
                        );
                        func()
                    })
                    .and_then(Result::ok)
            }
        };

        trace!(
            "Processing result for topic_id: {}, subtopic_id: {}",
            topic_id,
            subtopic_id
        );
        result.map(|(question, answer)| {
            trace!(
                "Returning result for topic_id: {}, subtopic_id: {}",
                topic_id,
                subtopic_id
            );
            json!({
                "question": question,
                "answer": answer
            })
        })
    })
}
