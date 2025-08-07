//! Module to handle CLI arguments parsing and execution.

use std::fs;

use clap::Parser;
use random_data::DataType;

use crate::dialog::Dialog;
use crate::errors::{Error, Res};
use crate::generator::{Data, Generator as _};
use crate::json::JsonArgs;

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
    #[arg(short = 'f', long = "file", default_value_t = String::from("schema.json"))]
    schema_file: String,
    /// Pass the JSON from stdout instead of via a json file.
    /// Overrides the --file option.
    #[arg(short, long)]
    json: Option<String>,
    /// Generates some data of the given data type.
    /// Overrides the other options.
    #[arg(short = 't', long = "type")]
    data_type: Option<String>,
    /// Add custom data types
    #[arg(short, long = "user")]
    user_defined: Vec<String>,
    /// Select the data type with a dialog and fuzzy search.
    /// Overrides the other options.
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
    /// List all available data types.
    /// Overrides the other options.
    #[arg(short, long, default_value_t = false)]
    list: bool,
    /// Debug errors with more precise information.
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

impl CliArgs {
    /// Parse the CLI arguments and run the appropriate generations.
    pub fn parse_and_run() -> Res<(), ()> {
        let this = Self::parse();
        let debug = this.debug;

        #[expect(clippy::print_stderr, clippy::print_stdout, reason = "it's a cli")]
        match this.run() {
            Ok(content) => {
                println!("{content}");
                Ok(())
            }
            Err(err) => {
                eprintln!("{}", err.display(debug));
                Err(())
            }
        }
    }

    /// Run the generation based on the parsed CLI arguments.
    fn run(self) -> Res<String> {
        if self.interactive && self.list {
            return Err(Error::ListAndInteractiveConflict);
        }

        if self.list {
            return Ok(DataType::list_str().join("\n"));
        }

        let mut data = Data::new(self.user_defined)?;

        if let Some(data_type) = self.data_type {
            return Ok(data_type.generate(&mut data)?.into_string());
        }

        if self.interactive {
            return Dialog::generate(data);
        }

        let json = if let Some(json) = self.json {
            json
        } else {
            fs::read_to_string(&self.schema_file)
                .map_err(Error::file_not_found(self.schema_file))?
        };

        JsonArgs::new(
            self.before.unwrap_or_default(),
            self.after.unwrap_or_default(),
            self.count,
            json,
            data,
        )
        .generate()
    }
}
