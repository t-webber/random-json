use std::{
    env::{self},
    fs,
    io::{self, BufRead, Write as _},
    mem,
    ops::Range,
};

use rand::{Rng as _, rngs::ThreadRng, seq::IndexedRandom};

mod auto;
mod macros;

pub fn get_range() -> Range<usize> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    print!("\x1b[33m?\x1b[0m Enter lower bound: ");
    io::stdout().flush().unwrap();
    let num1: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    print!("\x1B[1A\x1b[32m✔\x1B[1B\r");

    let mut num2 = num1;
    while num1 >= num2 {
        print!("\x1b[33m?\x1b[0m Enter upper bound: ");
        io::stdout().flush().unwrap();
        num2 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let icon = if num1 >= num2 {
            "\x1b[31m✘"
        } else {
            "\x1b[32m✔"
        };
        print!("\x1B[1A{icon}\x1b[0m\x1B[1B\r");
        io::stdout().flush().unwrap();
    }

    num1..num2
}

fn get_data_type() -> String {
    use dialoguer::{FuzzySelect, theme::ColorfulTheme};
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a data type")
        .items(&auto::FAKERS)
        .interact()
        .unwrap();

    auto::FAKERS[selection].to_string()
}

const POSITIONS: [&str; 5] = ["Trésorier", "VPO", "SecGe", "DirCo", "Info"];

fn manual(data_type: &str, rng: &mut ThreadRng) -> Option<String> {
    match data_type {
        "Position" => Some(POSITIONS.choose(rng).unwrap().to_string()),
        _ => None,
    }
}

fn generate_random(json: &mut serde_json::Value, rng: &mut ThreadRng) -> bool {
    match json {
        serde_json::Value::Null | serde_json::Value::Bool(_) | serde_json::Value::Number(_) => {
            panic!()
        }
        serde_json::Value::String(data_type) => {
            if data_type.ends_with("?") {
                if rng.random_bool(0.3) {
                    return false;
                } else {
                    data_type.pop();
                }
            }
            *data_type = manual(data_type, rng).unwrap_or_else(|| auto::fake(data_type).unwrap());
        }
        serde_json::Value::Array(values) => values.retain_mut(|son| generate_random(son, rng)),
        serde_json::Value::Object(map) => {
            let map_data = mem::take(map);
            for (k, mut v) in map_data {
                if generate_random(&mut v, rng) {
                    map.insert(k, v);
                }
            }
        }
    }
    true
}

fn print_one(mut json: serde_json::Value, rng: &mut ThreadRng) {
    generate_random(&mut json, rng);
    println!("{}, ", serde_json::to_string_pretty(&json).unwrap());
}

fn main() {
    let mut args = env::args().skip(1);
    let mut rng = rand::rng();
    if let Some(count) = args.next() {
        let count = count.parse().unwrap();
        let json: serde_json::Value =
            serde_json::from_str(&fs::read_to_string("schema.json").unwrap()).unwrap();

        if count > 1 {
            println!("[");
        }
        for _ in 0..count {
            print_one(json.clone(), &mut rng);
        }
        if count > 1 {
            println!("]");
        }
    } else {
        get_data_type();
    }
}
