use std::{
    env,
    io::Write as _,
    process::{Command, Stdio},
};

use dialoguer::{Select, theme::ColorfulTheme};

mod auto;
mod macros;

fn copy_to_clipboard(text: &str) {
    let mut child = Command::new("xclip")
        .args(["-selection", "clipboard"])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn xclip");

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(text.as_bytes())
            .expect("Failed to write to xclip");
    }

    child.wait().expect("Failed to wait on xclip");
}

fn get_data_type() -> String {
    let selection = Select::with_theme(&ColorfulTheme::default())
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
