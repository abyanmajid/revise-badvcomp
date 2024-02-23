use anyhow::Result;
use num_complex::Complex;
use rand::Rng;

fn format_double(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value.trunc() as i64) // Remove fractional part if it's an integer
    } else {
        format!("{:.2}", value) // Keep two decimal places if it's not an integer
    }
}

fn format_complex(c: Complex<f64>) -> String {
    let real_part = format_double(c.re);
    let imag_part = match c.im.abs() {
        1.0 => "i".to_string(),
        _ => format_double(c.im.abs()) + "i",
    };

    if c.re == 0.0 && c.im == 0.0 {
        "0".to_string()
    } else if c.im == 0.0 {
        real_part
    } else if c.re == 0.0 {
        if c.im > 0.0 {
            imag_part
        } else {
            "-i".to_string()
        }
    } else {
        real_part + " " + if c.im > 0.0 { "+" } else { "-" } + " " + &imag_part
    }
}

fn generate_random_complex() -> Complex<f64> {
    let mut rng = rand::thread_rng();
    let real_part: f64 = rng.gen_range(-10..=10) as f64; // Adjusted range to -10 to 10
    let imag_part: f64 = rng.gen_range(-10..=10) as f64; // Adjusted range to -10 to 10
    Complex::new(real_part, imag_part)
}

pub fn identify_parts_complex() -> Result<(String, String)> {
    let z = generate_random_complex();
    let question = format!("Given z = {} + {}i, what is Re(z) and Im(z)?", z.re, z.im);
    let answer = format!("Re(z) = {}, Im(z) = {}", z.re, z.im);
    Ok((question, answer))
}

pub fn add_complex() -> Result<(String, String)> {
    let z = generate_random_complex();
    let w = generate_random_complex();
    let result = z + w;
    let question = format!("What is ({}) + ({})?", format_complex(z), format_complex(w));
    let answer = format_complex(result);
    Ok((question, answer))
}

pub fn subtract_complex() -> Result<(String, String)> {
    let z = generate_random_complex();
    let w = generate_random_complex();
    let result = z - w;
    let question = format!("What is ({}) - ({})?", format_complex(z), format_complex(w));
    let answer = format_complex(result);
    Ok((question, answer))
}

pub fn multiply_complex() -> Result<(String, String)> {
    let z = generate_random_complex();
    let w = generate_random_complex();
    let result = z * w;
    let question = format!("What is ({}) * ({})?", format_complex(z), format_complex(w));
    let answer = format_complex(result);
    Ok((question, answer))
}

pub fn divide_complex() -> Result<(String, String)> {
    let z = generate_random_complex();
    let w = generate_random_complex();
    let result = if w != Complex::new(0.0, 0.0) {
        z / w
    } else {
        Complex::new(0.0, 0.0)
    };
    let question = format!("What is ({}) / ({})?", format_complex(z), format_complex(w));
    let answer = format_complex(result);
    Ok((question, answer))
}

fn generate_random_complex_radians() -> Complex<f64> {
    let mut rng = rand::thread_rng();
    let real_part_factor: f64 = rng.gen_range(-4..=4) as f64;
    let imag_part_factor: f64 = rng.gen_range(-4..=4) as f64;

    // Multiply by pi/2 to get the actual values
    let real_part = real_part_factor * std::f64::consts::FRAC_PI_2;
    let imaginary_part = imag_part_factor * std::f64::consts::FRAC_PI_2;

    Complex::new(real_part, imaginary_part)
}

pub fn modulus_and_principal_argument() -> Result<(String, String)> {
    // Use the function to generate a random complex number with angles in terms of pi
    let c = generate_random_complex_radians();

    // Calculate the modulus
    let modulus = c.norm();

    // Calculate the argument, ensuring it's in terms of pi
    let argument = c.arg();

    // Format the modulus using your format_double function
    let formatted_modulus = format_double(modulus);

    // Since the argument is already a multiple of pi/2, we can represent it as a fraction of pi
    let mut argument_in_terms_of_pi = argument / std::f64::consts::PI;

    // Normalize the argument to be within the range [-π, π]
    argument_in_terms_of_pi = if argument_in_terms_of_pi > 1.0 {
        argument_in_terms_of_pi % 2.0 - 2.0
    } else if argument_in_terms_of_pi < -1.0 {
        argument_in_terms_of_pi % 2.0 + 2.0
    } else {
        argument_in_terms_of_pi
    };

    // Generate the formatted argument
    let formatted_argument = match argument_in_terms_of_pi {
        0.0 => "0".to_string(),
        1.0 => "π".to_string(),
        -1.0 => "-π".to_string(),
        _ => {
            // For other cases, express the angle as a fraction of π
            let numerator = (argument_in_terms_of_pi * 2.0).round() as i64;
            let denominator = 2;
            // Handle cases where the numerator is 1 or -1
            if numerator.abs() == 1 {
                format!("{}π", if numerator == -1 { "-" } else { "" })
            } else {
                format!("{}π/{}", numerator, denominator)
            }
        }
    };

    // Generate the question using your format_complex function to display the complex number
    let question = format!(
        "Given the complex number {}, calculate its modulus and principal argument (in radians).",
        format_complex(c)
    );

    // Generate the answer
    let answer = format!(
        "The modulus is {}, and the principal argument is {} radians.",
        formatted_modulus, formatted_argument
    );

    Ok((question, answer))
}
