use anyhow::Result;
use num_complex::Complex;
use rand::seq::SliceRandom;
use rand::Rng;
use std::f64::consts::PI;

fn format_double(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value.trunc() as i64)
    } else {
        format!("{:.2}", value)
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

    let real_part = real_part_factor * std::f64::consts::FRAC_PI_2;
    let imaginary_part = imag_part_factor * std::f64::consts::FRAC_PI_2;

    Complex::new(real_part, imaginary_part)
}

pub fn modulus_and_principal_argument() -> Result<(String, String)> {
    let c = generate_random_complex_radians();

    let modulus = c.norm();

    let argument = c.arg();

    let formatted_modulus = format_double(modulus);

    let mut argument_in_terms_of_pi = argument / std::f64::consts::PI;

    argument_in_terms_of_pi = if argument_in_terms_of_pi > 1.0 {
        argument_in_terms_of_pi % 2.0 - 2.0
    } else if argument_in_terms_of_pi < -1.0 {
        argument_in_terms_of_pi % 2.0 + 2.0
    } else {
        argument_in_terms_of_pi
    };

    let formatted_argument = match argument_in_terms_of_pi {
        0.0 => "0".to_string(),
        1.0 => "π".to_string(),
        -1.0 => "-π".to_string(),
        _ => {
            let numerator = (argument_in_terms_of_pi * 2.0).round() as i64;
            let denominator = 2;
            if numerator.abs() == 1 {
                format!("{}π", if numerator == -1 { "-" } else { "" })
            } else {
                format!("{}π/{}", numerator, denominator)
            }
        }
    };

    let question = format!(
        "Given the complex number {}, calculate its modulus and principal argument (in radians).",
        format_complex(c)
    );

    let answer = format!(
        "The modulus is {}, and the principal argument is {} radians.",
        formatted_modulus, formatted_argument
    );

    Ok((question, answer))
}

fn approximate_pi_fraction(theta: f64) -> String {
    let fractions = vec![
        (0.0, "0"),
        (1.0 / 6.0, "π/6"),
        (1.0 / 4.0, "π/4"),
        (1.0 / 3.0, "π/3"),
        (1.0 / 2.0, "π/2"),
        (2.0 / 3.0, "2π/3"),
        (3.0 / 4.0, "3π/4"),
        (5.0 / 6.0, "5π/6"),
        (1.0, "π"),
        (7.0 / 6.0, "7π/6"),
        (5.0 / 4.0, "5π/4"),
        (4.0 / 3.0, "4π/3"),
        (3.0 / 2.0, "3π/2"),
        (5.0 / 3.0, "5π/3"),
        (7.0 / 4.0, "7π/4"),
        (11.0 / 6.0, "11π/6"),
    ];

    let normalized_theta = theta / PI;
    let closest = fractions
        .iter()
        .min_by_key(|(frac, _)| {
            let diff = (normalized_theta - frac).abs();
            (diff * 1_000_000.0) as i64
        })
        .map(|&(_, repr)| repr)
        .unwrap_or("π");

    closest.to_string()
}

pub fn cartesian_to_polar_and_exponential() -> Result<(String, String)> {
    let c = generate_random_complex();
    let r = c.norm();
    let theta = c.arg();

    let formatted_theta = approximate_pi_fraction(theta);

    let formatted_r = format!("{:.2}", r);

    let polar_form = format!(
        "{}(cos {} + i sin {})",
        formatted_r, formatted_theta, formatted_theta
    );

    let exponential_form = format!("{}e^(i{})", formatted_r, formatted_theta);

    let question = format!(
        "Convert the complex number {} to polar and exponential form.",
        format_complex(c)
    );
    let answer = format!(
        "Polar form: {}; Exponential form: {}",
        polar_form, exponential_form
    );

    Ok((question, answer))
}

pub fn polar_to_cartesian() -> Result<(String, String)> {
    let mut rng = rand::thread_rng();

    let angles = vec![
        ("0", 0.0),                                   // 0
        ("π/6", std::f64::consts::PI / 6.0),          // π/6
        ("π/4", std::f64::consts::PI / 4.0),          // π/4
        ("π/3", std::f64::consts::PI / 3.0),          // π/3
        ("π/2", std::f64::consts::PI / 2.0),          // π/2
        ("2π/3", 2.0 * std::f64::consts::PI / 3.0),   // 2π/3
        ("3π/4", 3.0 * std::f64::consts::PI / 4.0),   // 3π/4
        ("5π/6", 5.0 * std::f64::consts::PI / 6.0),   // 5π/6
        ("π", std::f64::consts::PI),                  // π
        ("7π/6", 7.0 * std::f64::consts::PI / 6.0),   // 7π/6
        ("5π/4", 5.0 * std::f64::consts::PI / 4.0),   // 5π/4
        ("4π/3", 4.0 * std::f64::consts::PI / 3.0),   // 4π/3
        ("3π/2", 3.0 * std::f64::consts::PI / 2.0),   // 3π/2
        ("5π/3", 5.0 * std::f64::consts::PI / 3.0),   // 5π/3
        ("7π/4", 7.0 * std::f64::consts::PI / 4.0),   // 7π/4
        ("11π/6", 11.0 * std::f64::consts::PI / 6.0), // 11π/6
    ];

    let r: f64 = rng.gen_range(1.0..=20.0) * 0.5;

    let (theta_repr, theta) = angles.choose(&mut rng).unwrap();

    let real_part = (r * theta.cos()).round();
    let imag_part = (r * theta.sin()).round();

    let question = if rng.gen_bool(0.5) {
        format!(
            "Convert the complex number in exponential polar form {:.2}e^(i{}) to Cartesian form.",
            r, theta_repr
        )
    } else {
        format!(
            "Convert the complex number in standard polar form {:.2}(cos {} + i sin {}) to Cartesian form.",
            r, theta_repr, theta_repr
        )
    };

    let answer = format!("{} + {}i", real_part, imag_part);

    Ok((question, answer))
}
