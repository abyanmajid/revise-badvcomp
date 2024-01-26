use anyhow::{anyhow, Result};
use rand::Rng;
use std::fmt;
use tracing::trace;

#[derive(PartialEq, Eq, Debug)]
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

pub fn base_encoding() -> Result<(String, String)> {
    trace!("Running generator: ELEC1601 base encoding question");

    let mut rng = rand::thread_rng();

    // Choose two different bases
    let from_base = match rng.gen_range(0..=3) {
        0 => Base::Binary,
        1 => Base::Octal,
        2 => Base::Decimal,
        3 => Base::Hexadecimal,
        _ => return Err(anyhow!("Random number generation failed")),
    };

    trace!("Selected from_base: {:?}", from_base);

    let to_base = loop {
        let base = match rng.gen_range(0..=3) {
            0 => Base::Binary,
            1 => Base::Octal,
            2 => Base::Decimal,
            3 => Base::Hexadecimal,
            _ => return Err(anyhow!("Random number generation failed")),
        };

        if base != from_base {
            break base;
        }
    };

    trace!("Selected to_base: {:?}", to_base);

    // Generate a random number
    let number_to_convert = rng.gen_range(100..100000);

    trace!("Selected random decimal number: {:?}", number_to_convert);

    // Convert the number to the from_base
    let number_in_base = match from_base {
        Base::Binary => format!("{:b}", number_to_convert),
        Base::Octal => format!("{:o}", number_to_convert),
        Base::Decimal => format!("{}", number_to_convert),
        Base::Hexadecimal => format!("{:X}", number_to_convert),
    };

    trace!(
        "Representing selected decimal number in from_base: {:?}",
        number_in_base
    );

    // Convert the number to the to_base
    let correct_answer = match to_base {
        Base::Binary => format!(
            "{:b}",
            i64::from_str_radix(&number_in_base, from_base.to_string().parse::<u32>()?)?
        ),
        Base::Octal => format!(
            "{:o}",
            i64::from_str_radix(&number_in_base, from_base.to_string().parse::<u32>()?)?
        ),
        Base::Decimal => format!(
            "{}",
            i64::from_str_radix(&number_in_base, from_base.to_string().parse::<u32>()?)?
        ),
        Base::Hexadecimal => format!(
            "{:X}",
            i64::from_str_radix(&number_in_base, from_base.to_string().parse::<u32>()?)?
        ),
    };

    trace!("Derived correct_answer in to_base: {:?}", correct_answer);

    let question = format!(
        "What is the base {} representation of the base {} number {}?",
        to_base, from_base, number_in_base
    );

    trace!("Finished generating base_encoding problem");

    Ok((question, correct_answer))
}
