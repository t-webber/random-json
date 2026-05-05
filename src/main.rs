//! CLI to generate some fake data under JSON format.

mod clap;
mod data;
mod data_generator;
mod dialog;
mod errors;
mod generator_trait;
mod json;

fn main() {
    use std::process::exit;

    use crate::clap::CliArgs;

    if CliArgs::parse_and_run().is_err() {
        exit(1)
    }
}
