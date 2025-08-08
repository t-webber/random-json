//! Module to handle CLI arguments parsing and execution.

use std::fs;

use clap::Parser;

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
    /// String to print before every JSON generation
    #[arg(short, long)]
    before: Option<String>,
    /// String to print after every JSON generation
    #[arg(short, long)]
    after: Option<String>,
    /// Path to the json schema.
    #[arg(short = 'f', long = "schema", default_value_t = String::from("schema.json"))]
    schema_file: String,
    /// Pass the JSON from stdout instead of via a json file.
    #[arg(short, long)]
    json: Option<String>,
    /// Generates some data of the given data type.
    #[arg(short = 't', long = "type")]
    data_type: Option<String>,
    /// Add custom data types, with the format 'Type:Value1|Value2'
    #[arg(short, long = "user")]
    user_defined: Vec<String>,
    /// Select the data type with a dialog and fuzzy search.
    #[arg(short, long, default_value_t = false)]
    interactive: bool,
    /// List all available data types.
    #[arg(short, long, default_value_t = false)]
    list: bool,
    /// List all values of a type
    #[arg(short, long)]
    values: Option<String>,
    /// Debug errors with more precise information.
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

impl CliArgs {
    /// Check if the sequence of commands given are meaningful
    fn check_arguments(&self) -> Res<()> {
        let provided = self.list_provided();

        Err(
            if self.list
                && let Some(other) = find_other_than(&provided, &["list", "user", "type"])
            {
                Error::ConflictingArgs("list", other)
            } else if self.interactive
                && let Some(other) = find_other_than(&provided, &["interactive", "user", "type"])
            {
                Error::ConflictingArgs("interactive", other)
            } else if self.values.is_some()
                && let Some(other) = find_other_than(&provided, &["values", "user", "type"])
            {
                Error::ConflictingArgs("values", other)
            } else if self.json.is_some() && self.schema_file != "schema.json" {
                Error::ConflictingArgs("json", "schema")
            } else {
                return Ok(());
            },
        )
    }

    /// Count the number of provided arguments
    fn list_provided(&self) -> Vec<&'static str> {
        let mut provided = Vec::with_capacity(5);

        if self.count != 1 {
            provided.push("count");
        }
        if self.before.is_some() {
            provided.push("before");
        }
        if self.after.is_some() {
            provided.push("after");
        }
        if self.schema_file != "schema.json" {
            provided.push("schema");
        }
        if self.json.is_some() {
            provided.push("json");
        }
        if self.data_type.is_some() {
            provided.push("type");
        }
        if !self.user_defined.is_empty() {
            provided.push("user");
        }
        if self.interactive {
            provided.push("interactive");
        }
        if self.list {
            provided.push("list");
        }
        if self.values.is_some() {
            provided.push("values");
        }

        provided
    }

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
        self.check_arguments()?;
        let mut data = Data::new(self.user_defined)?;

        if let Some(data_type) = self.values {
            return data.values(&data_type);
        }

        if self.list {
            return Ok(data.list().join("\n"));
        }

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

/// Find an element in a slice that is different than a given value
fn find_other_than(list: &[&'static str], allowed: &[&'static str]) -> Option<&'static str> {
    list.iter().find(|&entry| !allowed.contains(entry)).copied()
}
