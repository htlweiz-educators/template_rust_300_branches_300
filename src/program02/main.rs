use std::{f32::consts::PI, io::Write};

const MSG_ERR_READLINE_FAILED: &str = "Error: Readline failed!";
const MSG_ERR_PARSING_FAILED: &str = "Error: Parsing failed!";
const MSG_ERR_FLUSH_FAILED: &str = "Error: Flushing failed!";

const MSG_PROMPT_SIDE_A: &str = "Please enter the side of a cube shaped box in mm: ";
const MSG_PROMPT_RADIUS: &str = "Please enter the radius of a sphere in mm: ";

macro_rules! MSG_FITS_IN {
    () => {
        "The sphere with an radius of {} mm does fit in a cube shaped box with length of {} mm."
    };
}

macro_rules! MSG_AIR_REMAINS {
    () => {
        "In the box {:.3} mm³ of air remain."
    };
}

macro_rules! MST_DOES_NOT_FIT {
    () => {
        "The sphere with an radius of {} mm does not fit in a cube shaped box with length of {} mm."
    };
}

fn main() {
    print!("{MSG_PROMPT_SIDE_A}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    let mut side_a = String::new();
    std::io::stdin()
        .read_line(&mut side_a)
        .expect(MSG_ERR_READLINE_FAILED);
    let side_a: u32 = side_a.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    print!("{MSG_PROMPT_RADIUS}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    let mut radius = String::new();
    std::io::stdin()
        .read_line(&mut radius)
        .expect(MSG_ERR_READLINE_FAILED);
    let radius: u32 = radius.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    if (2 * radius) < side_a {
        let volume_cube = (side_a * side_a * side_a) as f64;
        let volume_sphere = 4.0 / 3.0 * (radius * radius * radius) as f64 * PI as f64;
        println!(MSG_FITS_IN!(), radius, side_a);
        println!(MSG_AIR_REMAINS!(), volume_cube - volume_sphere);
    } else {
        println!(MST_DOES_NOT_FIT!(), radius, side_a);
    }
}
