use std::io::Write;

const MSG_ERR_READLINE_FAILED: &str = "Error: Readline failed!";
const MSG_ERR_PARSING_FAILED: &str = "Error: Parsing failed!";
const MSG_ERR_FLUSH_FAILED: &str = "Error: Flushing failed!";

const MSG_PROMPT_NUMBER_PEOPLE: &str = "Please enter the number of persons traveling: ";
const MSG_TO_FEW_PEOPLE: &str = "You cannot travel with no people.";
const MSG_TO_MANY_PEOPLE: &str = "We cannot serve over 100 people.";

macro_rules! MSG_UP_TO_FIVE {
    () => {
        "The journey with {} people costs you {:.2} EUR each. In total {:.2} EUR."
    };
}

macro_rules! MSG_UP_TO_TWENTY {
    () => {
        "The journey with {} people costs you {:.2} EUR each. In total {:.2} EUR."
    };
}

macro_rules! MSG_UP_TO_ONEHUNDRET {
    () => {
        "The journey with {} people costs you {:.2} EUR each. Plus a {:.2} EUR per bus for {} busses. In total {:.2} EUR."
    };
}

fn main() {
    print!("{MSG_PROMPT_NUMBER_PEOPLE}");
    std::io::stdout().flush().expect(MSG_ERR_FLUSH_FAILED);
    let mut number_of_people = String::new();
    std::io::stdin()
        .read_line(&mut number_of_people)
        .expect(MSG_ERR_READLINE_FAILED);
    let number_of_people: u32 = number_of_people
        .trim_end()
        .parse()
        .expect(MSG_ERR_PARSING_FAILED);

    match number_of_people {
        0 => println!("{MSG_TO_FEW_PEOPLE}"),
        1..=5 => {
            let price = 12.5 * number_of_people as f64;
            println!(MSG_UP_TO_FIVE!(), number_of_people, 12.5, price);
        }
        6..=20 => {
            let price = 9.5 * number_of_people as f64;
            println!(MSG_UP_TO_TWENTY!(), number_of_people, 9.5, price);
        }
        21..=100 => {
            let mut busses = number_of_people / 20;
            if !number_of_people.is_multiple_of(20) {
                busses += 1;
            }
            let bus_price = 5.0;
            let person_price = 6.5;

            let price = person_price * number_of_people as f64 + bus_price * busses as f64;
            println!(
                MSG_UP_TO_ONEHUNDRET!(),
                number_of_people, person_price, bus_price, busses, price
            );
        }
        _ => println!("{MSG_TO_MANY_PEOPLE}"),
    }
}
