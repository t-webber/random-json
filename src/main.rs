use crate::{dialog::generate::generate_from_dialog, errors::Res, json::generate::JsonArgs};
use clap::Parser;
use rand::{rng, rngs::ThreadRng};
mod data;
mod dialog;
mod errors;
mod json;

/// CLI to generate some fake data under JSON format.
#[derive(Parser, Debug)]
struct Args {
    /// Number of times to repeat the JSON
    #[arg(short, long, default_value_t = 1)]
    count: u32,
    /// String to print before every repetion of the JSON schema.
    #[arg(short, long, default_value_t = String::new())]
    before: String,
    /// String to print after every repetion of the JSON schema.
    #[arg(short, long, default_value_t = String::new())]
    after: String,
    /// Path to the json schema.
    #[arg(short, long, default_value_t = String::from("schema.json"))]
    file: String,
    /// List and select the random generator with a terminal dialog.
    /// This option overrides the others.
    #[arg(short, long, default_value_t = false)]
    dialog: bool,
}

impl Args {
    fn run(self, rng: &mut ThreadRng) -> Res {
        if self.dialog {
            println!("{}", generate_from_dialog(rng)?);
            Ok(())
        } else {
            JsonArgs::new(self.before, self.after, self.count, self.file, rng).generate()
        }
    }
}

fn main() -> Res {
    let mut rng = rng();
    Args::parse().run(&mut rng)
}
