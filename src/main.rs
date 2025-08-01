use std::{
    env,
    io::{self, BufRead, Write as _},
    ops::Range,
    process::{Command, Stdio},
};

use dialoguer::{FuzzySelect, theme::ColorfulTheme};

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
        print!("\x1B[1A\x1b[32m✔\x1b[0m\x1B[1B\r");
        io::stdout().flush().unwrap();
    }

    num1..num2
}

fn copy_to_clipboard(text: &str) {
    let mut process = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(mut stdin) = process.stdin.take() {
        stdin.write_all(text.as_bytes()).unwrap();
    }

    process.wait().unwrap();
}

fn get_data_type() -> String {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a data type")
        .items(&auto::FAKERS)
        .interact()
        .unwrap();

    auto::FAKERS[selection].to_string()
}

fn try_get_faker(data_type: Option<String>) {
    if let Some(data_type) = data_type
        && let Some(data) = auto::fake(&data_type)
    {
        copy_to_clipboard(&data);
        println!("{data}");
    } else {
        try_get_faker(Some(get_data_type()));
    }
}

fn main() {
    try_get_faker(env::args().nth(1));
}
