//! Module to handle CLI arguments parsing and execution.

use std::fs;

use clap::{ArgGroup, Parser};

use crate::data::Data;
use crate::dialog::Dialog;
use crate::errors::{Error, Res};
use crate::generator_trait::Generator as _;
use crate::json::JsonArgs;

/// CLI to generate some fake data under JSON format.
#[expect(
    clippy::arbitrary_source_item_ordering,
    reason = "order matters for CLI arguments ordering"
)]
#[derive(Parser, Debug)]
#[command(group(
        ArgGroup::new("action")
        .args(["interactive", "pattern", "file", "data_type", "list", "values"])
))]
#[command(group(ArgGroup::new("combinable").multiple(true)))]
pub struct CliArgs {
    /// Number of times to repeat the JSON
    #[arg(short, long, default_value_t = 1, group = "combinable")]
    count: u32,
    /// String to print before every JSON generation
    #[arg(short, long, group = "combinable")]
    before: Option<String>,
    /// String to print after every JSON generation
    #[arg(short, long, group = "combinable")]
    after: Option<String>,
    /// Deprecrated, use `--file` instead
    #[arg(long, hide = true)]
    schema: Option<String>,
    /// Deprecrated, use `--pattern` instead
    #[arg(short, long, hide = true)]
    json: Option<String>,
    /// Path to the json schema.
    #[arg(short, long, group = "combinable")]
    file: Option<String>,
    /// Pass the JSON from stdout instead of via a json file.
    #[arg(short, long, group = "combinable")]
    pattern: Option<String>,
    /// Generates some data of the given data type.
    #[arg(short = 't', long = "type", group = "combinable")]
    data_type: Option<String>,
    /// Add custom data types, with the format 'Type:Value1|Value2'
    #[arg(short, long = "user", group = "combinable")]
    user_defined: Vec<String>,
    /// Select the data type with a dialog and fuzzy search.
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["combinable", "list", "values"])]
    interactive: bool,
    /// List all available data types.
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["combinable", "interactive", "values"])]
    list: bool,
    /// List all values of a type
    #[arg(short, long, conflicts_with_all = ["combinable", "interactive", "list"])]
    values: Option<String>,
    /// Debug errors with more precise information.
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    /// Generate with a given random seed
    #[arg(short, long, group = "combinable")]
    seed: Option<u64>,
}

impl CliArgs {
    /// Check if the sequence of commands given are meaningful
    const fn check_arguments(&self) -> Res<()> {
        if self.schema.is_some() {
            return Err(Error::DeprecatedArg("schema", "file"));
        }
        if self.json.is_some() {
            return Err(Error::DeprecatedArg("json", "pattern"));
        }
        Ok(())
    }

    /// Run the generation based on the parsed CLI arguments.
    pub fn run(self) -> Res<String, (Error, bool)> {
        let debug = self.debug;
        self.run_no_debug().map_err(|err| (err, debug))
    }

    /// Run the generation based on the parsed CLI arguments.
    fn run_no_debug(self) -> Res<String> {
        self.check_arguments()?;
        let mut data = Data::new(self.user_defined, self.seed)?;

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

        let pattern = if let Some(pattern) = self.pattern {
            pattern
        } else if let Some(path) = self.file {
            fs::read_to_string(&path).map_err(Error::file_not_found(path))?
        } else {
            return Err(Error::NoPattern);
        };

        JsonArgs::new(
            self.before.unwrap_or_default(),
            self.after.unwrap_or_default(),
            self.count,
            pattern,
            data,
        )
        .generate()
    }
}
