use rand::Rng;
use std::fmt;

// Define an enum to represent different bases
#[derive(PartialEq, Eq)]
enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Base::Binary => "2",
                Base::Octal => "8",
                Base::Decimal => "10",
                Base::Hexadecimal => "16",
            }
        )
    }
}

pub fn base_encoding_question() -> Result<(String, String), &'static str> {
    let mut rng = rand::thread_rng();

    // Choose two different bases
    let from_base = match rng.gen_range(0..=3) {
        0 => Base::Binary,
        1 => Base::Octal,
        2 => Base::Decimal,
        3 => Base::Hexadecimal,
        _ => return Err("Random number generation failed"),
    };

    let mut to_base;
    loop {
        to_base = match rng.gen_range(0..=3) {
            0 => Base::Binary,
            1 => Base::Octal,
            2 => Base::Decimal,
            3 => Base::Hexadecimal,
            _ => return Err("Random number generation failed"),
        };

        if to_base != from_base {
            break;
        }
    }

    // Generate a random number
    let number_to_convert = rng.gen_range(100..100000);

    // Convert the number to the from_base
    let number_in_base = match from_base {
        Base::Binary => format!("{:b}", number_to_convert),
        Base::Octal => format!("{:o}", number_to_convert),
        Base::Decimal => format!("{}", number_to_convert),
        Base::Hexadecimal => format!("{:X}", number_to_convert),
    };

    // Convert the number to the to_base
    let correct_answer = match to_base {
        Base::Binary => format!(
            "{:b}",
            i64::from_str_radix(
                &number_in_base,
                from_base.to_string().parse::<u32>().unwrap()
            )
            .unwrap()
        ),
        Base::Octal => format!(
            "{:o}",
            i64::from_str_radix(
                &number_in_base,
                from_base.to_string().parse::<u32>().unwrap()
            )
            .unwrap()
        ),
        Base::Decimal => format!(
            "{}",
            i64::from_str_radix(
                &number_in_base,
                from_base.to_string().parse::<u32>().unwrap()
            )
            .unwrap()
        ),
        Base::Hexadecimal => format!(
            "{:X}",
            i64::from_str_radix(
                &number_in_base,
                from_base.to_string().parse::<u32>().unwrap()
            )
            .unwrap()
        ),
    };

    let question = format!(
        "What is the base {} representation of the base {} number {}?",
        to_base, from_base, number_in_base
    );
    Ok((question, correct_answer))
}

fn main() {
    match base_encoding_question() {
        Ok((question, answer)) => {
            println!("Question: {}", question);
            println!("Answer: {}", answer);
        }
        Err(e) => println!("Error: {}", e),
    }
}
