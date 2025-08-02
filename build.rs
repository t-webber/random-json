use std::borrow::Cow;
use std::fmt::Write as _;
use std::fs;

fn main() {
    let url = "https://raw.githubusercontent.com/cksac/fake-rs/master/fake/src/faker/mod.rs";
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    let mut mod_name = None;
    let mut simple_fakers = String::new();
    let mut call_fakers_single = String::new();
    let mut call_fakers_multiple = String::new();
    let mut in_mode_section = false;

    for mut line in body.lines() {
        if !in_mode_section {
            if line == "pub mod impls;" {
                in_mode_section = true;
            }
            continue;
        }

        if line.contains("def_fakers") {
            continue;
        }

        line = line.trim();
        if line.starts_with('}') || line.starts_with('#') {
            mod_name = None;
            continue;
        }
        if line.ends_with("();") {
            line = &line[..line.len() - 3];
        }
        if line.ends_with(" {") {
            line = &line[..line.len() - 2];
        }

        if let Some(prefix) = line.strip_prefix("pub mod ") {
            mod_name = Some(prefix.to_owned());
            continue;
        }

        if mod_name.as_ref().is_some_and(|inner| inner == "markdown") {
            continue;
        }

        if let Some(modname) = mod_name.as_ref() {
            if modname == "time" && line == "Duration"
                || line.contains("dt: ")
                || line.contains("start: ")
                || line.contains("fmt: ")
            {
                continue;
            }

            let mut convert_type = Cow::Borrowed(if line == "Duration" { line } else { "String" });

            let prefix = if let Some((prefix, _)) = line.split_once('(') {
                if prefix == "Boolean" {
                    line = prefix;
                    convert_type = Cow::Borrowed("bool, 50");
                    None
                } else if prefix == "Geohash" {
                    line = prefix;
                    convert_type = Cow::Borrowed("String, 255");
                    None
                } else {
                    Some(prefix)
                }
            } else {
                None
            };

            if let Some(prefix) = prefix {
                if prefix.ends_with('s') {
                    writeln!(call_fakers_multiple, "{modname}, {prefix}")
                } else {
                    writeln!(call_fakers_single, "{modname}, {prefix}")
                }
            } else {
                writeln!(simple_fakers, "{modname}, {line}, {convert_type}")
            }
            .unwrap()
        }
    }

    fs::write(
        "src/data/auto.rs",
        format!(
            r##"//! Contains the list of fakers available in the `fake` library.
//!
//! Auto-generated file, do not edit manually.

#![expect(clippy::arbitrary_source_item_ordering, reason = "macro definitions")]

use fake::Fake as _;

use crate::{{call_fakers, simple_fakers}};

/// List of data type that are associated with a faker.
pub fn get_fakers() -> Vec<&'static str> {{
    let mut fakers = Vec::with_capacity(SIMPLE_FAKERS.len().saturating_add(CALL_FAKERS.len()));
    fakers.extend_from_slice(SIMPLE_FAKERS);
    fakers.extend_from_slice(CALL_FAKERS);
    fakers
}}

simple_fakers!(
{simple_fakers}
);

call_fakers!(
{call_fakers_single},
{call_fakers_multiple}
);
"##
        ),
    )
    .unwrap();
}
