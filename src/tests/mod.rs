#![expect(clippy::panic, reason = "test")]

use clap::Parser as _;
use serde_json::Value;

use crate::clap::CliArgs;

#[test]
fn repeat() {
    let mut out =
        match CliArgs::parse_from(["", "-p", r#"{"name": "FirstName"}"#, "-c", "2", "-a", ","])
            .run()
        {
            Ok(out) => out,
            Err((err, _)) => panic!("{err:?}"),
        };
    assert_eq!(out.pop(), Some('\n'));
    assert_eq!(out.pop(), Some(','));
    let data = match serde_json::from_str::<Value>(&format!("[{out}]")) {
        Ok(Value::Array(data)) => data,
        Ok(other) => panic!("{out} is not an array: {other}"),
        Err(err) => panic!("{out} is not json: {err}"),
    };
    for elt in data {
        assert_ne!(elt.get("name"), None);
    }
}
