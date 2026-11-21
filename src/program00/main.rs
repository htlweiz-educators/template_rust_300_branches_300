use std::io::Write;

const MSG_ERR_READLINE_FAILED: &str = "Error: Readline failed!";
const MSG_ERR_PARSING_FAILED: &str = "Error: Parsing failed!";
const MSG_ERR_FLUSH_FAILED: &str = "Error: Flushing failed!";

const MSG_PROMPT_FOR_YEAR: &str = "Please enter your year of birth: ";
const MSG_PROMPT_ALPHA: &str = "Congratulations, you are an Alpha.";
const MSG_PROMPT_MILLENIAL: &str = "Congratulations, you are a Millenial.";
const MSG_PROMPT_Z: &str = "Congratulations, you are a Gen-Z.";
const MSG_PROMPT_X: &str = "Congratulations, you are a Gen-X.";
const MSG_PROMPT_BOOMER: &str = "Congratulations, you are a Boomer.";
const MSG_PROMPT_NO_GEN: &str = "I have no generation for you!";

fn main() {
    print!("{MSG_PROMPT_FOR_YEAR}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    let mut year_of_birth = String::new();
    std::io::stdin()
        .read_line(&mut year_of_birth)
        .expect(MSG_ERR_READLINE_FAILED);
    let year_of_birth: i32 = year_of_birth
        .trim_end()
        .parse()
        .expect(MSG_ERR_PARSING_FAILED);

    match year_of_birth {
        1946..=1964 => println!("{MSG_PROMPT_BOOMER}"),
        1965..=1980 => println!("{MSG_PROMPT_X}"),
        1981..=1995 => println!("{MSG_PROMPT_MILLENIAL}"),
        1996..=2012 => println!("{MSG_PROMPT_Z}"),
        2013..2027 => println!("{MSG_PROMPT_ALPHA}"),
        _ => println!("{MSG_PROMPT_NO_GEN}"),
    }
}
