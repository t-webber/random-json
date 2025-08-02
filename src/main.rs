use crate::clap::CliArgs;
use crate::errors::Res;
mod clap;
mod data;
mod dialog;
mod errors;
mod json;
use rand::rng;

fn main() -> Res {
    let mut rng = rng();
    CliArgs::parse_and_run(&mut rng)
}
