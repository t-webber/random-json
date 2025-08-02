use crate::clap::CliArgs;
use crate::errors::Res;
use rand::rng;
mod clap;
mod data;
mod dialog;
mod errors;
mod json;

fn main() -> Res {
    let mut rng = rng();
    CliArgs::parse_and_run(&mut rng)
}
