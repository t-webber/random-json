//! CLI to generate some fake data under JSON format.

mod clap;
mod data;
mod data_generator;
mod dialog;
mod errors;
mod generator_trait;
mod json;

use std::process::ExitCode;

fn main() -> ExitCode {
    use crate::clap::CliArgs;

    #[expect(clippy::print_stdout, clippy::print_stderr, reason = "it's a cli")]
    match CliArgs::parse_and_run() {
        Ok(content) => {
            println!("{content}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}
