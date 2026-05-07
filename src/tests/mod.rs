#![expect(clippy::panic, clippy::unwrap_used, reason = "test")]

use core::iter::once;
use std::collections::HashSet;

use clap::Parser as _;
use serde_json::Value;

use crate::clap::{Action, CliArgs};

#[test]
fn repeat() {
    let mut out = run(["", "-p", r#"{"name": "FirstName"}"#, "-c", "2", "-a", ","]);
    assert_eq!(out.pop(), Some('\n'));
    assert_eq!(out.pop(), Some(','));
    let data = match serde_json::from_str::<Value>(&format!("[{out}]")) {
        Ok(Value::Array(data)) => data,
        Ok(other) => panic!("{out} is not an array: {other}"),
        Err(err) => panic!("{out} is not json: {err}"),
    };
    for elt in data {
        let Value::Object(obj) = &elt else {
            panic!("{elt} is not an object")
        };
        assert_eq!(
            obj.keys().map(String::as_str).collect::<HashSet<&str>>(),
            once("name").collect()
        );
        let Some(Value::String(_)) = obj.get("name") else {
            panic!("{elt}'s name isn't a string");
        };
    }
}

fn run<const N: usize>(args: [&str; N]) -> String {
    match CliArgs::parse_from(args).dispatch().1.and_then(Action::run) {
        Ok(out) => out,
        Err(err) => panic!("{err:?}"),
    }
}

#[test]
fn schema() {
    let schema = r#"{
            "name": "FirstName",
            "other_name": "FirstName",
            "address": "Address"
        }"#;
    let out = run(["", "-p", schema]);
    let data = match serde_json::from_str::<Value>(&out) {
        Ok(Value::Object(data)) => data,
        Ok(other) => panic!("{out} is not an object: {other}"),
        Err(err) => panic!("{out} is not json: {err}"),
    };
    assert_eq!(
        data.keys().map(String::as_str).collect::<HashSet<&str>>(),
        ["name", "other_name", "address"].into_iter().collect()
    );
}

#[test]
fn conflict() {
    for (first, second) in [("-p", "-f"), ("-s", "-l"), ("-p", "-i")] {
        CliArgs::try_parse_from(["", first, "", second, ""]).unwrap_err();
    }
}

#[test]
fn list() {
    let out = run(["", "-l"]);
    assert!(out.contains("Country"));
    assert!(out.contains("FirstName"));
}
