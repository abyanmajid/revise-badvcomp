use anyhow::Result;
use rand::Rng;

pub fn find_register_value() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();

    let reg_a: i32 = rng.gen_range(0..=255);
    let reg_b: i32 = rng.gen_range(0..=255);
    let reg_c: i32 = rng.gen_range(0..=255);
    let reg_d: i32 = rng.gen_range(0..=255);
    let letters = ['A', 'B', 'C', 'D'];
    let group_to_trace = letters[rng.gen_range(0..letters.len())];
    let num_raising_edges: i32 = rng.gen_range(4..=12);

    let registers = [reg_a, reg_b, reg_c, reg_d];
    let effective_edges = num_raising_edges % 4;
    let mut shifted_registers = registers;

    for _ in 0..effective_edges {
        shifted_registers.rotate_right(1);
    }

    let group_index = letters.iter().position(|&r| r == group_to_trace).unwrap();
    let correct_answer = shifted_registers[group_index].to_string();

    let question = format!(
        "Given four groups of rising-edge sensitive registers with the following details,

Regs A -----> Regs B
  ^            |
  |            |
  |            v
Regs D <----- Regs C

Current values:
- Register Group A = {}
- Register Group B = {}
- Register Group C = {}
- Register Group D = {}

What would be the value of registers group {} after {} rising edges of the clock?",
        reg_a, reg_b, reg_c, reg_d, group_to_trace, num_raising_edges
    );

    Ok((question, correct_answer))
}
