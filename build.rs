use std::{borrow::Cow, fmt::Write as _, fs};

const API: &str = include_str!("faker_api.txt");

fn main() {
    let mut mod_name = None;
    let mut simple_fakers = String::new();
    let mut call_fakers_single = String::new();
    let mut call_fakers_multiple = String::new();
    for mut line in API.lines() {
        if line.starts_with('}') {
            mod_name = None;
            continue;
        }

        if let Some(prefix) = line.strip_prefix("pub mod ") {
            mod_name = Some(prefix.to_owned());
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
        } else {
            panic!()
        }
    }

    fs::write(
        "src/auto.rs",
        format!(
            r##"use crate::{{call_fakers, simple_fakers}};
use fake::Fake;
use std::cell::LazyCell;

pub const FAKERS: LazyCell<Vec<&'static str>> = LazyCell::new(|| {{
    let mut fakers = Vec::with_capacity(SIMPLE_FAKERS.len() + CALL_FAKERS.len());
    fakers.extend_from_slice(SIMPLE_FAKERS);
    fakers.extend_from_slice(CALL_FAKERS);
    fakers
}});

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
