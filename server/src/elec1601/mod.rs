use super::randomize;
use anyhow::{Error, Result};
use maplit::hashmap;
use rand::seq::SliceRandom;
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::trace;

#[path = "01-base_encoding.rs"]
pub mod topic_1;
#[path = "02-fixed_point_numbers.rs"]
pub mod topic_2;
#[path = "03-memory_size.rs"]
pub mod topic_3;
#[path = "04-registers.rs"]
pub mod topic_4;
#[path = "05-stack.rs"]
pub mod topic_5;
#[path = "06-avr.rs"]
pub mod topic_6;

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
        1 => vec![topic_1::base_encoding as FuncType],
        2 => vec![topic_2::fixed_to_dec as FuncType, topic_2::dec_to_fixed as FuncType],
        3 => vec![topic_3::total_size_question as FuncType,
                topic_3::cell_size_question as FuncType,
                topic_3::no_of_cells_question as FuncType,
                topic_3::address_bits_question as FuncType
            ],
        4 => vec![topic_4::find_register_value as FuncType],
        5 => vec![topic_5::track_stack_contents as FuncType],
        6 => vec![topic_6::avr_assembly_andi_breq as FuncType]
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
