use clap::Parser;
use rand::rngs::ThreadRng;

use crate::{dialog::generate::generate_from_dialog, errors::Res, json::generate::JsonArgs};

/// CLI to generate some fake data under JSON format.
#[derive(Parser, Debug)]
pub struct CliArgs {
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

impl CliArgs {
    pub fn parse_and_run(rng: &mut ThreadRng) -> Res {
        Self::parse().run(rng)
    }

    fn run(self, rng: &mut ThreadRng) -> Res {
        if self.dialog {
            println!("{}", generate_from_dialog(rng)?);
            Ok(())
        } else {
            JsonArgs::new(self.before, self.after, self.count, self.file, rng).generate()
        }
    }
}
