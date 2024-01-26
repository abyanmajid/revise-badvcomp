use anyhow::Result;
use rand::seq::SliceRandom;
use rand::Rng;
use tracing::trace;

pub fn fixed_to_dec() -> Result<(String, String)> {
    trace!("Entering fixed_to_dec function");

    let mut rng = rand::thread_rng();
    let n: usize = rng.gen_range(6..=12);
    trace!("Generated n value: {}", n);

    let fixed_point_number: String = (0..n).map(|_| rng.gen_range(0..=1).to_string()).collect();
    trace!("Generated fixed point number: {}", fixed_point_number);

    let no_of_int_bits = (n as f64 * 2.0 / 3.0).ceil() as usize;
    let no_of_frac_bits = (n as f64 * 1.0 / 3.0).floor() as usize;
    trace!(
        "Number of integer bits: {}, Number of fractional bits: {}",
        no_of_int_bits,
        no_of_frac_bits
    );

    let integer_slice = &fixed_point_number[..no_of_int_bits];
    let fractional_slice = &fixed_point_number[no_of_int_bits..];
    trace!(
        "Integer slice: {}, Fractional slice: {}",
        integer_slice,
        fractional_slice
    );

    let integer = isize::from_str_radix(integer_slice, 2)?;
    trace!("Converted integer part: {}", integer);

    let fraction: f64 = fractional_slice
        .chars()
        .enumerate()
        .fold(0.0, |acc, (i, bit)| {
            let value = (bit.to_digit(10).unwrap() as f64) * 2.0_f64.powi(-(i as i32 + 1));
            trace!("Processing bit: {}, Value: {}", bit, value);
            acc + value
        });
    trace!("Calculated fraction part: {}", fraction);

    let correct_answer = format!("{}", integer as f64 + fraction);
    let question = format!("Given a fixed-point system with {} integer bits and {} fractional bits, what is the decimal representation of the fixed-point number {}?", no_of_int_bits, no_of_frac_bits, fixed_point_number);
    trace!("Question: {}, Correct Answer: {}", question, correct_answer);

    Ok((question, correct_answer))
}
pub fn dec_to_fixed() -> Result<(String, String)> {
    trace!("Entering dec_to_fixed function");

    let mut rng = rand::thread_rng();
    let int_powers = vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512];
    let frac_powers = vec![0.5, 0.25, 0.125, 0.0625, 0.03125];

    let a = rng.gen_range(3..=int_powers.len());
    let b = rng.gen_range(0..=frac_powers.len());
    trace!("Generated random values a: {}, b: {}", a, b);

    let int_sample: Vec<usize> = int_powers.choose_multiple(&mut rng, a).cloned().collect();
    let integer: usize = int_sample.iter().sum();
    trace!("Integer part generated: {}", integer);

    let (fraction, fractional_slice) = if b != 0 {
        let frac_sample: Vec<f64> = frac_powers.choose_multiple(&mut rng, b).cloned().collect();
        let fraction: f64 = frac_sample.iter().sum();
        let fractional_slice: String = frac_powers
            .iter()
            .map(|power| {
                if frac_sample.contains(power) {
                    '1'
                } else {
                    '0'
                }
            })
            .collect();
        trace!(
            "Fractional part generated: {}, Slice: {}",
            fraction,
            fractional_slice
        );
        (fraction, fractional_slice)
    } else {
        trace!("No fractional part generated");
        (0.0, "".to_string())
    };

    let decimal_num = integer as f64 + fraction;
    trace!("Decimal number generated: {}", decimal_num);

    let integer_slice = format!("{:b}", integer);
    trace!("Binary integer slice: {}", integer_slice);

    let correct_answer = if fractional_slice.is_empty() {
        integer_slice.clone()
    } else {
        format!("{}.{}", integer_slice, fractional_slice)
    };
    trace!("Correct answer generated: {}", correct_answer);

    let question = format!("What is the fixed-point representation of the decimal number {}? If your answer includes a fractional part, please separate the integer and fractional parts with a dot '.' (e.g., 3.5 = 11.1)", decimal_num);
    trace!("Question generated: {}", question);

    Ok((question, correct_answer))
}
