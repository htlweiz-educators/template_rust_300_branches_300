use std::io::Write;

const MSG_ERR_READLINE_FAILED: &str = "Error: Readline failed!";
const MSG_ERR_PARSING_FAILED: &str = "Error: Parsing failed!";
const MSG_ERR_FLUSH_FAILED: &str = "Error: Flushing failed!";

const MSG_PROMPT_BOX_A: &str = "Please input parameters of the first box.";
const MSG_PROMPT_BOX_B: &str = "Please input parameters of the second box.";
const MSG_PROMPT_SIDE_A: &str = "a in mm: ";
const MSG_PROMPT_SIDE_B: &str = "b in mm: ";
const MSG_PROMPT_SIDE_C: &str = "c in mm: ";

macro_rules! MSG_FITS_IN {
    () => {
        "The box of {}x{}x{} mm does fit in the box of {}x{}x{} mm."
    };
}

macro_rules! MSG_AIR_REMAINS {
    () => {
        "In the box {:.3} mm³ of air remains."
    };
}

macro_rules! MST_DOES_NOT_FIT {
    () => {
        "The box of {}x{}x{} mm does not fit in the box of {}x{}x{} mm."
    };
}

fn main() {
    let mut side_a = String::new();
    let mut side_b = String::new();
    let mut side_c = String::new();

    println!("{MSG_PROMPT_BOX_A}");
    print!("{MSG_PROMPT_SIDE_A}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_a)
        .expect(MSG_ERR_READLINE_FAILED);
    let box_a_side_a: u32 = side_a.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    print!("{MSG_PROMPT_SIDE_B}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_b)
        .expect(MSG_ERR_READLINE_FAILED);
    let box_a_side_b: u32 = side_b.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    print!("{MSG_PROMPT_SIDE_C}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_c)
        .expect(MSG_ERR_READLINE_FAILED);
    let box_a_side_c: u32 = side_c.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    side_a.clear();
    side_b.clear();
    side_c.clear();

    println!("{MSG_PROMPT_BOX_B}");
    print!("{MSG_PROMPT_SIDE_A}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_a)
        .expect(MSG_ERR_READLINE_FAILED);
    let box_b_side_a: u32 = side_a.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    print!("{MSG_PROMPT_SIDE_B}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_b)
        .expect(MSG_ERR_READLINE_FAILED);
    let box_b_side_b: u32 = side_b.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    print!("{MSG_PROMPT_SIDE_C}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    std::io::stdin()
        .read_line(&mut side_c)
        .expect(MSG_ERR_READLINE_FAILED);
    let box_b_side_c: u32 = side_c.trim_end().parse().expect(MSG_ERR_PARSING_FAILED);

    if false
        || (box_b_side_a < box_a_side_a
            && box_b_side_b < box_a_side_b
            && box_b_side_c < box_a_side_c)
        || (box_b_side_a < box_a_side_a
            && box_b_side_b < box_a_side_c
            && box_b_side_c < box_a_side_b)
        || (box_b_side_a < box_a_side_b
            && box_b_side_b < box_a_side_c
            && box_b_side_c < box_a_side_a)
        || (box_b_side_a < box_a_side_b
            && box_b_side_b < box_a_side_a
            && box_b_side_c < box_a_side_c)
        || (box_b_side_a < box_a_side_c
            && box_b_side_b < box_a_side_a
            && box_b_side_c < box_a_side_b)
        || (box_b_side_a < box_a_side_c
            && box_b_side_b < box_a_side_b
            && box_b_side_c < box_a_side_a)
    {
        let vol_a = box_a_side_a * box_a_side_b * box_a_side_c;
        let vol_b = box_b_side_a * box_b_side_b * box_b_side_c;
        println!(
            MSG_FITS_IN!(),
            box_b_side_a, box_b_side_b, box_b_side_c, box_a_side_a, box_a_side_b, box_a_side_c
        );
        println!(MSG_AIR_REMAINS!(), vol_a - vol_b);
    } else {
        println!(
            MST_DOES_NOT_FIT!(),
            box_a_side_a, box_a_side_b, box_a_side_c, box_b_side_a, box_b_side_b, box_b_side_c
        )
    }
}
