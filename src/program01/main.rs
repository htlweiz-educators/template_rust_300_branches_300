use std::io::Write;

const MSG_ERR_READLINE_FAILED: &str = "Error: Readline failed!";
const MSG_ERR_PARSING_FAILED: &str = "Error: Parsing failed!";
const MSG_ERR_FLUSH_FAILED: &str = "Error: Flushing failed!";

const MSG_PROMPT_RECT_A: &str = "Please input parameters of the first rectangle.";
const MSG_PROMPT_RECT_B: &str = "Please input parameters of the second rectangle.";
const MSG_PROMPT_SIDE_A: &str = "Side a in mm: ";
const MSG_PROMPT_SIDE_B: &str = "Side b in mm: ";

macro_rules! MSG_FITS_IN {
    () => {
        "The rectangle {} mm x {} mm fits in the rectangle {} mm x {} mm."
    };
}

macro_rules! MSG_DIFFERENCE {
    () => {
        "The size differs by {} mm²."
    };
}

macro_rules! MSG_DOES_NOT_FIT {
    () => {
        "The rectangle {} mm x {} mm doesn't fit in the rectangle {} mm x {} mm."
    };
}

fn main() {
    let mut side_a = String::new();
    let mut side_b = String::new();

    println!("{MSG_PROMPT_RECT_A}");
    print!("{MSG_PROMPT_SIDE_A}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_a)
        .expect(MSG_ERR_READLINE_FAILED);
    let rect_a_side_a: u32 = side_a.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);
    print!("{MSG_PROMPT_SIDE_B}");
    std::io::stdout().flush().expect(MSG_ERR_READLINE_FAILED);
    std::io::stdin()
        .read_line(&mut side_b)
        .expect(MSG_ERR_READLINE_FAILED);
    let rect_a_side_b: u32 = side_b.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    side_a.clear();
    side_b.clear();

    println!("{MSG_PROMPT_RECT_B}");
    print!("{MSG_PROMPT_SIDE_A}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_a)
        .expect(MSG_ERR_READLINE_FAILED);
    let rect_b_side_a: u32 = side_a.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);
    print!("{MSG_PROMPT_SIDE_B}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_b)
        .expect(MSG_ERR_READLINE_FAILED);
    let rect_b_side_b: u32 = side_b.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    if (rect_b_side_b < rect_a_side_b && rect_b_side_a < rect_a_side_a)
        || (rect_b_side_b < rect_a_side_a && rect_b_side_a < rect_a_side_b)
    {
        let area_a = rect_a_side_a * rect_a_side_b;
        let area_b = rect_b_side_a * rect_b_side_a;
        println!(
            MSG_FITS_IN!(),
            rect_b_side_a, rect_b_side_b, rect_a_side_a, rect_a_side_b
        );
        println!(MSG_DIFFERENCE!(), area_a - area_b);
    } else {
        println!(
            MSG_DOES_NOT_FIT!(),
            rect_b_side_a, rect_b_side_b, rect_a_side_a, rect_a_side_b
        );
    }
}
