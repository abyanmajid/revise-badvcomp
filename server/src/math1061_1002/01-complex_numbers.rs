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
