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

use crate::clap::{Action, CliArgs};

fn main() -> ExitCode {
    let (debug, res) = CliArgs::parse().dispatch();
    #[expect(clippy::print_stdout, clippy::print_stderr, reason = "it's a cli")]
    match res.and_then(Action::run) {
        Ok(content) => {
            println!("{content}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}", err.display(debug));
            ExitCode::FAILURE
        }
    }
}
