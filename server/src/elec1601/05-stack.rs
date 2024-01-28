use anyhow::Result;
use rand::Rng;

pub fn track_stack_contents() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let mut stack: Vec<i32> = Vec::new();
    let mut operations = Vec::new();
    let mut result_stack = Vec::new();

    // Randomly generate the number of operations
    let num_operations = rng.gen_range(3..=10);

    for _ in 0..num_operations {
        if rng.gen_bool(0.5) || stack.is_empty() {
            // Perform a push operation 50% of the time or if the stack is empty
            let value = rng.gen_range(1..=100);
            stack.push(value);
            operations.push(format!("push({})", value));
            result_stack.push(value);
        } else {
            // Perform a pop operation
            stack.pop();
            operations.push("pop()".to_string());
            result_stack.pop();
        }
    }

    // Generate the problem statement
    let problem_statement = format!(
        "Given an empty stack, perform the following operations:\n{}\nWhat are the contents of the stack after these operations (from top to bottom)?",
        operations.join(", ")
    );

    // Generate the answer
    let correct_answer = if result_stack.is_empty() {
        "empty".to_string()
    } else {
        result_stack
            .into_iter()
            .rev() // Stack contents are displayed from top to bottom
            .map(|num| num.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    };

    Ok((problem_statement, correct_answer))
}
