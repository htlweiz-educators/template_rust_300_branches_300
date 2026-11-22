use std::{f64::consts::PI, io::Write};

const MSG_ERR_READLINE_FAILED: &str = "Error: Readline failed!";
const MSG_ERR_PARSING_FAILED: &str = "Error: Parsing failed!";
const MSG_ERR_FLUSH_FAILED: &str = "Error: Flushing failed!";

const MSG_PROMPT_SHAPE: &str = "Choose a shape to calculate the area:\n1) Circle\n2) Rectangle\n3) Triangle\nSelection: ";
const MSG_PROMPT_RADIUS: &str = "Enter the radius in mm: ";
const MSG_PROMPT_LEN: &str = "Enter the length in mm: ";
const MSG_PROMPT_WID: &str = "Enter the width in mm: ";
const MSG_PROMPT_BASE: &str = "Enter the base in mm: ";
const MSG_PROMPT_HEIGHT: &str = "Enter the height in mm: ";
const MSG_INVALID_SHAPE: &str = "Invalid selection!";

fn main() {
    print!("{MSG_PROMPT_SHAPE}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    let mut choice = String::new();
    std::io::stdin()
        .read_line(&mut choice)
        .expect(MSG_ERR_READLINE_FAILED);
    let choice: u32 = choice.trim().parse().expect(MSG_ERR_PARSING_FAILED);

    match choice {
        1 => {
            print!("{MSG_PROMPT_RADIUS}");
            std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
            let mut r = String::new();
            std::io::stdin()
                .read_line(&mut r)
                .expect(MSG_ERR_READLINE_FAILED);
            let r: u32 = r.trim().parse().expect(MSG_ERR_PARSING_FAILED);
            let area = PI * (r as f64) * (r as f64);
            println!("The area of the circle is {:.3} mm².", area);
        }
        2 => {
            print!("{MSG_PROMPT_LEN}");
            std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
            let mut l = String::new();
            std::io::stdin()
                .read_line(&mut l)
                .expect(MSG_ERR_READLINE_FAILED);
            let l: u32 = l.trim().parse().expect(MSG_ERR_PARSING_FAILED);

            print!("{MSG_PROMPT_WID}");
            std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
            let mut w = String::new();
            std::io::stdin()
                .read_line(&mut w)
                .expect(MSG_ERR_READLINE_FAILED);
            let w: u32 = w.trim().parse().expect(MSG_ERR_PARSING_FAILED);
            let area = (l as f64) * (w as f64);
            println!("The area of the rectangle is {:.3} mm².", area);
        }
        3 => {
            print!("{MSG_PROMPT_BASE}");
            std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
            let mut b = String::new();
            std::io::stdin()
                .read_line(&mut b)
                .expect(MSG_ERR_READLINE_FAILED);
            let b: u32 = b.trim().parse().expect(MSG_ERR_PARSING_FAILED);

            print!("{MSG_PROMPT_HEIGHT}");
            std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
            let mut h = String::new();
            std::io::stdin()
                .read_line(&mut h)
                .expect(MSG_ERR_READLINE_FAILED);
            let h: u32 = h.trim().parse().expect(MSG_ERR_PARSING_FAILED);
            let area = 0.5 * (b as f64) * (h as f64);
            println!("The area of the triangle is {:.3} mm².", area);
        }
        _ => println!("{MSG_INVALID_SHAPE}"),
    }
}
