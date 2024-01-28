use anyhow::Result;
use rand::Rng;

pub fn avr_assembly_andi_breq() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let pc_initial: u16 = rng.gen_range(0x100..=0x3FF); // Sensible range for AVR program memory
    let r20_initial: u8 = rng.gen_range(0..=255);
    let andi_mask: u8 = rng.gen_range(1..=255); // Immediate value for ANDI
    let mut pc_new = pc_initial + 2; // Initialize PC to the next instruction address

    let result = r20_initial & andi_mask;

    if result == 0 {
        let breq_offset: i8 = rng.gen_range(-64..64);
        let offset_in_bytes = (breq_offset as i16) * 2;
        pc_new = pc_new.wrapping_add(offset_in_bytes as u16);
    }

    let problem_statement = format!(
        "Suppose the following two instructions are executed:\n\
        ANDI R20, {:#04X}\n\
        BREQ Destination\n\
        The machine code for the BREQ instruction is as follows:\n\
        1111 0000 0010 1001\n\
        Assume the following register values:\n\
        Program counter = 0x{:X} (This corresponds to the location of the ANDI instruction, before execution)\n\
        R20 = 0x{:X}\n\
        What is the new value of the PC? (write your answer in Decimal)",
        andi_mask, pc_initial, r20_initial
    );

    let correct_answer = format!("{}", pc_new);
    Ok((problem_statement, correct_answer))
}
