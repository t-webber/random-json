//! Module to handle CLI arguments parsing and execution.

use clap::Parser;
use rand::rngs::ThreadRng;

use crate::data::auto::get_fakers;
use crate::dialogue::generate::generate_from_dialogue;
use crate::errors::Res;
use crate::json::generate::JsonArgs;

/// CLI to generate some fake data under JSON format.
#[expect(
    clippy::arbitrary_source_item_ordering,
    reason = "order matters for CLI arguments ordering"
)]
#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Number of times to repeat the JSON
    #[arg(short, long, default_value_t = 1)]
    count: u32,
    /// String to print before every data generation of the JSON schema.
    #[arg(short, long, default_value_t = String::new())]
    before: String,
    /// String to print after every data generation of the JSON schema.
    #[arg(short, long, default_value_t = String::new())]
    after: String,
    /// Path to the json schema.
    #[arg(short, long, default_value_t = String::from("schema.json"))]
    file: String,
    /// List and select the random generator with a terminal dialogue.
    /// This option overrides the others.
    #[arg(short, long, default_value_t = false)]
    dialogue: bool,
}

impl CliArgs {
    /// Parse the CLI arguments and run the appropriate generations.
    pub fn parse_and_run(rng: &mut ThreadRng) -> Res<String> {
        Self::parse().run(rng)
    }

    /// Run the generation based on the parsed CLI arguments.
    fn run(self, rng: &mut ThreadRng) -> Res<String> {
        if self.dialogue {
            let fakers = get_fakers();
            generate_from_dialogue(rng, &fakers)
        } else {
            JsonArgs::new(self.before, self.after, self.count, self.file, rng).generate()
        }
    }
}
