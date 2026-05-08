//! Module to handle CLI arguments parsing and execution.

use std::fs;

use clap::{ArgGroup, Parser};
use color_eyre::eyre::{Context as _, eyre};

use crate::Res;
use crate::data::Data;
use crate::dialog::Dialog;
use crate::json::JsonArgs;

/// CLI to generate some fake data under JSON format.
#[derive(Parser, Debug)]
#[command(group(
        ArgGroup::new("action")
        .args(["interactive", "pattern", "file", "data_type", "list_types", "values"])
))]
#[command(group(ArgGroup::new("combinable").multiple(true)))]
pub struct CliArgs {
    /// String to print after every output generation
    #[arg(short, long, group = "combinable")]
    after: String,
    /// String to print before every output generation
    #[arg(short, long, group = "combinable")]
    before: String,
    /// Number of times to repeat the output.
    #[arg(short, long, default_value_t = 1, group = "combinable")]
    count: u32,
    /// Deprecated, use `--pattern` instead
    #[arg(short = 't', long = "type", group = "combinable", hide = true)]
    data_type: Option<String>,
    /// Debug errors with more precise information.
    #[arg(short, long, default_value_t = false, hide = true)]
    debug: bool,
    /// Path to the file containing the pattern to use to generate the data.
    /// Supports json, csv, tsv.
    #[arg(short, long, group = "combinable")]
    file: Option<String>,
    /// Select the data type with a dialog and fuzzy search.
    #[arg(short, long, default_value_t = false, conflicts_with_all = ["combinable", "list_types", "values"])]
    interactive: bool,
    /// Deprecrated, use `--pattern` instead
    #[arg(short, long, hide = true)]
    json: Option<String>,
    /// List all available data types.
    #[arg(short, long="list", default_value_t = false, conflicts_with_all = ["combinable", "interactive", "values"])]
    list_types: bool,
    /// Pass a pattern using the CLI argument instead of in a file. Supports
    /// json, csv, tsv.
    #[arg(short, long, group = "combinable")]
    pattern: Option<String>,
    /// Deprecrated, use `--file` instead
    #[arg(long, hide = true)]
    schema: Option<String>,
    /// Generate with a given random seed
    #[arg(short, long, group = "combinable")]
    seed: Option<u64>,
    /// Add custom data types, with the format 'Type:Value1|Value2'
    #[arg(short, long = "user", group = "combinable")]
    user_defined: Vec<String>,
    /// List all values of a type
    #[arg(short, long, conflicts_with_all = ["combinable", "interactive", "list_types"])]
    values: Option<String>,
}

impl CliArgs {
    /// Check if the sequence of commands given are meaningful
    pub fn dispatch(self) -> (bool, Res<Action>) {
        macro_rules! schema {
            ($pat:expr) => {
                Action::Schema {
                    count: self.count,
                    before: self.before.unwrap_or_default(),
                    after: self.after.unwrap_or_default(),
                    user_defined: self.user_defined,
                    seed: self.seed,
                    pattern: $pat,
                }
            };
        }

        (
            self.debug,
            if self.schema.is_some() {
                Err(eyre!("Use of `--schema` is deprecated and was replaced by `--file`."))
            } else if self.json.is_some() {
                Err(eyre!("Use of `--json` is deprecated and was replaced by `--pattern`."))
            } else if self.data_type.is_some() {
                Err(eyre!("Use of `--type` is deprecated and was replaced by `--pattern`."))
            } else if let Some(pattern) = self.pattern {
                Ok(schema!(pattern))
            } else if let Some(file) = self.file {
                fs::read_to_string(&file)
                    .with_context(|| format!("Failed to read {file}"))
                    .map(|content| schema!(content))
            } else if self.interactive {
                Ok(Action::Interactive)
            } else if let Some(values) = self.values {
                Ok(Action::ListValues(values))
            } else if self.list_types {
                Ok(Action::ListTypes)
            } else {
                Err(eyre!(
                    "Nothing to be done: no action provided. Provide one of `--file`, `--pattern, `--list`, `--interactive`, `--values`"
                ))
            },
        )
    }
}

/// Action to be run after decoding the user input.
pub enum Action {
    /// Show the interactive dialog.
    Interactive,
    /// List all the available data types.
    ListTypes,
    /// List all values of a data type.
    ListValues(String),
    /// Schema to produce the random data.
    Schema {
        /// Number of times to repeat the output.
        count: u32,
        /// String to print before every output generation
        before: String,
        /// String to print after every output generation
        after: String,
        /// Pattern to use for data generation. Supports
        /// json, csv, tsv.
        pattern: String,
        /// Add custom data types, with the format 'Type:Value1|Value2'
        user_defined: Vec<String>,
        /// Generate with a given random seed
        seed: Option<u64>,
    },
}

impl Action {
    /// Runs the appropriate action.
    pub fn run(self) -> Res<String> {
        let data = Data::new(vec![], None)?;
        match self {
            Self::Schema { count, before, after, pattern, user_defined, seed } =>
                JsonArgs::new(before, after, count, pattern, Data::new(user_defined, seed)?)
                    .generate(),
            Self::Interactive => Dialog::generate(data),
            Self::ListTypes => Ok(data.list().join("\n")),
            Self::ListValues(ty) => data.values(&ty),
        }
    }
}
