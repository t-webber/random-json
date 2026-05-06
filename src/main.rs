//! CLI to generate some fake data under JSON format.

mod clap;
mod data;
mod data_generator;
mod dialog;
mod errors;
mod generator_trait;
mod json;
#[cfg(test)]
mod tests;

use std::process::ExitCode;

use ::clap::Parser as _;

use crate::clap::CliArgs;

fn main() -> ExitCode {
    #[expect(clippy::print_stdout, clippy::print_stderr, reason = "it's a cli")]
    match CliArgs::parse().run() {
        Ok(content) => {
            println!("{content}");
            ExitCode::SUCCESS
        }
        Err((err, debug)) => {
            eprintln!("{}", err.display(debug));
            ExitCode::FAILURE
        }
    }
}
