//! Module to handle CLI arguments parsing and execution.

use std::fs;

use clap::Parser;
use rand::rngs::ThreadRng;

use crate::data::auto::get_fakers;
use crate::dialogue::generate::generate_from_dialogue;
use crate::errors::{Error, Res};
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
    #[arg(short, long)]
    before: Option<String>,
    /// String to print after every data generation of the JSON schema.
    #[arg(short, long)]
    after: Option<String>,
    /// Path to the json schema.
    #[arg(short, long, default_value_t = String::from("schema.json"))]
    file: String,
    /// Pass the JSON from stdout instead of via a json file
    /// This option overrides --file if both are provided.
    #[arg(short, long)]
    json: Option<String>,
    /// Select the data type with a terminal dialogue with fuzzy search.
    /// This option overrides the others.
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
    /// List all available data types.
    /// This option overrides the others.
    #[arg(short, long, default_value_t = false)]
    list: bool,
    /// Debug errors with more precise information.
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

impl CliArgs {
    /// Parse the CLI arguments and run the appropriate generations.
    pub fn parse_and_run(rng: &mut ThreadRng) {
        let this = Self::parse();
        let debug = this.debug;

        #[expect(clippy::print_stderr, clippy::print_stdout, reason = "it's a cli")]
        match this.run(rng) {
            Ok(content) => println!("{content}"),
            Err(err) => eprintln!("{}", err.display(debug)),
        }
    }

    /// Run the generation based on the parsed CLI arguments.
    fn run(self, rng: &mut ThreadRng) -> Res<String> {
        if self.interactive && self.list {
            return Err(Error::ListAndInteractiveConflict);
        }

        if self.interactive {
            return generate_from_dialogue(rng, &get_fakers());
        }

        if self.list {
            return Ok(get_fakers().join("\n"));
        }

        let json = if let Some(json) = self.json {
            json
        } else {
            fs::read_to_string(&self.file).map_err(Error::file_not_found(self.file))?
        };

        JsonArgs::new(
            self.before.unwrap_or_default(),
            self.after.unwrap_or_default(),
            self.count,
            json,
            rng,
        )
        .generate()
    }
}
