use super::types::Question;

#[path = "01-base_encoding.rs"]
pub mod topic_1;

pub fn generate_problem(id: u32) {
    // match id {
    //     1 => match topic_1::base_encoding() {
    //         Ok((question, answer)) => {
    //             // Create a response object
    //             let response = Question { question, answer };
    //
    //             // Serialize the response object to JSON
    //             match serde_json::to_string(&response) {
    //                 Ok(json) => println!("{}", json),
    //                 Err(e) => eprintln!("Error serializing to JSON: {}", e),
    //             }
    //         }
    //         Err(e) => eprintln!("Error: {}", e),
    //     },
    // }
}
