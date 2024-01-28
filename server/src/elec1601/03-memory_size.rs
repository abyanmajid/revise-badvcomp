use anyhow::Result;
use rand::seq::SliceRandom;
use rand::Rng;
use std::convert::TryInto;

const UNITS: &[&str] = &["bytes", "kilobytes", "megabytes"];

pub fn total_size_question() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(8..=32); // in bits
    let m: u32 = rng.gen_range(2..=32); // in bytes

    let correct_answer: f64 = 2f64.powi(n.try_into()?) * m as f64;

    let answer_in_unit = UNITS
        .choose(&mut rng)
        .ok_or_else(|| anyhow::anyhow!("Failed to select a unit"))?;
    let mut correct_answer_adjusted = correct_answer;
    if *answer_in_unit == "kilobytes" {
        correct_answer_adjusted /= 1024.0;
    } else if *answer_in_unit == "megabytes" {
        correct_answer_adjusted /= 1048576.0;
    }

    let question = format!(
        "What is the size of a memory with {} address bits, and cells each sized {} bytes? Your answer should be in {}!",
        n, m, answer_in_unit
    );
    let answer = format!("{}", correct_answer_adjusted);

    Ok((question, answer))
}

pub fn cell_size_question() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let total: f64 = 2f64.powi(rng.gen_range(16..=24));
    let n: u32 = rng.gen_range(12..=16);

    let correct_answer = total / 2f64.powi(n.try_into()?);

    let total_in_unit = UNITS
        .choose(&mut rng)
        .ok_or_else(|| anyhow::anyhow!("Failed to select a unit"))?;
    let mut total_adjusted = total;
    if *total_in_unit == "kilobytes" {
        total_adjusted /= 1024.0;
    } else if *total_in_unit == "megabytes" {
        total_adjusted /= 1048576.0;
    }

    let question = format!(
        "What is the size of each cell in a memory sized {} {} with {} address bits? Your answer should be in bytes!",
        total_adjusted, total_in_unit, n
    );
    let answer = format!("{}", correct_answer);

    Ok((question, answer))
}

pub fn no_of_cells_question() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let m: u32 = rng.gen_range(2..=32);
    let power_of_2: i32 = rng.gen_range(16..=24); // This ensures 'total' is a multiple of 'm'
    let total: f64 = (2f64.powi(power_of_2) * m as f64).into();

    let correct_answer = total / m as f64; // This will always be an integer

    let total_in_unit = UNITS
        .choose(&mut rng)
        .ok_or_else(|| anyhow::anyhow!("Failed to select a unit"))?;
    let mut total_adjusted = total;
    if *total_in_unit == "kilobytes" {
        total_adjusted /= 1024.0;
    } else if *total_in_unit == "megabytes" {
        total_adjusted /= 1048576.0;
    }

    let question = format!(
        "How many cells are there in a memory sized {} {} where each cell is {} bytes?",
        total_adjusted, total_in_unit, m
    );
    let answer = format!("{}", correct_answer);

    Ok((question, answer))
}

pub fn address_bits_question() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();
    let m: u32 = 2u32.pow(rng.gen_range(1..=5)); // ensures to give a value of m that is a power of 2 between 2 and 32

    let power_of_2: i32 = rng.gen_range(16..=24);
    let total: f64 = (2f64.powi(power_of_2) * m as f64).into();

    let correct_answer: f64 = (total / m as f64).log2();

    let total_in_unit = UNITS
        .choose(&mut rng)
        .ok_or_else(|| anyhow::anyhow!("Failed to select a unit"))?;
    let mut total_adjusted = total;
    if *total_in_unit == "kilobytes" {
        total_adjusted /= 1024.0;
    } else if *total_in_unit == "megabytes" {
        total_adjusted /= 1048576.0;
    }

    let question = format!(
        "How many bits are needed to encode the addresses of a memory sized {} {} where each cell is {} bytes?",
        total_adjusted, total_in_unit, m
    );
    let answer = format!("{}", correct_answer);

    Ok((question, answer))
}
