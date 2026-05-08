//! CLI to generate some fake data under JSON format.

mod clap;
mod data;
mod data_generator;
mod dialog;
mod generator_trait;
mod json;
#[cfg(test)]
mod tests;

use std::process::ExitCode;

use ::clap::Parser as _;

use crate::clap::{Action, CliArgs};

/// Colour eyre result short-hand that doesn't conflict with [`Result`]
type Res<T = ()> = color_eyre::Result<T>;

fn main() -> Res<ExitCode> {
    color_eyre::install()?;
    let (debug, action) = CliArgs::parse().dispatch();
    let res = action.and_then(Action::run);
    #[expect(clippy::print_stdout, clippy::print_stderr, reason = "it's a cli")]
    match res {
        Ok(content) => {
            println!("{content}");
            Ok(ExitCode::SUCCESS)
        }
        Err(err) =>
            if debug {
                Err(err)
            } else {
                let mut prefix = "";
                for er in err.chain() {
                    eprintln!("{prefix}\x1b[31m{er}\x1b[0m");
                    prefix = "  ";
                }
                Ok(ExitCode::FAILURE)
            },
    }
}
