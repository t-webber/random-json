use std::{
    env,
    io::Write as _,
    process::{Command, Stdio},
};

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

fn main() {
    if let Some(input) = env::args().nth(1)
        && let Some(data) = auto::fake(&input)
    {
        println!("=== {}", &data);
    } else {
        dbg!(auto::FAKERS.iter().collect::<Vec<_>>());
        panic!("Invalid data");
    }
}
