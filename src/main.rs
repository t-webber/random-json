//! CLI to generate some fake data under JSON format.

#![warn(
    missing_docs,
    warnings,
    deprecated_safe,
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    refining_impl_trait,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,
    clippy::all,
    clippy::pedantic,
    clippy::style,
    clippy::perf,
    clippy::complexity,
    clippy::correctness,
    clippy::restriction,
    clippy::nursery,
    clippy::cargo
)]
#![expect(clippy::multiple_crate_versions, reason = "needed by used crates")]
#![expect(clippy::blanket_clippy_restriction_lints, reason = "enable all lints")]
#![expect(
    clippy::single_call_fn,
    clippy::implicit_return,
    clippy::pattern_type_mismatch,
    clippy::question_mark_used,
    reason = "bad lints"
)]
#![expect(clippy::mod_module_files, reason = "chosen style")]

mod clap;
mod dialog;
mod errors;
mod generator;
mod json;

fn main() {
    use std::process::exit;

    use crate::clap::CliArgs;

    if CliArgs::parse_and_run().is_err() {
        exit(1)
    }
}
