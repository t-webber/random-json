//! Generator for when a JSON schema file is provided.

use core::fmt::Write as _;
use std::fs;

use rand::Rng as _;
use rand::rngs::ThreadRng;
use serde_json::{Map, Value};

use crate::data::generate::generate_data;
use crate::errors::{Error, Res};

/// Arguments for generating JSON data based on a schema file.
pub struct JsonArgs<'rng> {
    /// String to print after every data generation of the JSON schema.
    after: String,
    /// String to print before every data generation of the JSON schema.
    before: String,
    /// Number of times to repeat the JSON generation.
    count: u32,
    /// JSON schema content
    json: String,
    /// Random number generator to use for generating data.
    rng: &'rng mut ThreadRng,
}

impl<'rng> JsonArgs<'rng> {
    /// Generate the JSON data based on the schema file and the provided
    /// parameters.
    pub fn generate(self) -> Res<String> {
        let json: Value = serde_json::from_str(&self.json).map_err(Error::InvalidJson)?;

        let mut generated_data = String::new();
        for _ in 0..self.count {
            let generate_json = Self::generate_json(&json, self.rng).unwrap_or_default();
            let generate_json_str =
                serde_json::to_string_pretty(&generate_json).map_err(Error::DeserializeJson)?;
            writeln!(generated_data, "{}{generate_json_str}{}", self.before, self.after)
                .map_err(Error::JsonWriteString)?;
        }

        Ok(generated_data)
    }

    /// Generate JSON data based on the provided JSON schema.
    fn generate_json(json: &Value, rng: &mut ThreadRng) -> Res<Option<Value>> {
        let generated_json = match json {
            Value::Null | Value::Bool(_) | Value::Number(_) =>
                return Err(Error::InvalidSchemaType(format!("{json:?}"))),
            Value::String(data_type) => {
                let parsed_data_type = if let Some(parsed_data_type) = data_type.strip_suffix('?') {
                    if rng.random_bool(0.3) {
                        return Ok(None);
                    }
                    parsed_data_type
                } else {
                    data_type
                };
                Value::String(generate_data(parsed_data_type, rng)?)
            }
            Value::Array(values) => {
                let new_values = values
                    .iter()
                    .filter_map(|son| match Self::generate_json(son, rng) {
                        Ok(None) => None,
                        Ok(Some(value)) => Some(Ok(value)),
                        Err(err) => Some(Err(err)),
                    })
                    .collect::<Res<Vec<_>>>()?;
                Value::Array(new_values)
            }
            Value::Object(map) => {
                let mut new_map = Map::with_capacity(map.len());
                for (key, json_value) in map {
                    if let Some(generated_value) = Self::generate_json(json_value, rng)? {
                        new_map.insert(key.to_owned(), generated_value);
                    }
                }
                Value::Object(new_map)
            }
        };

        Ok(Some(generated_json))
    }

    /// Create a new instance of `JsonArgs` with the provided parameters.
    pub const fn new(
        before: String,
        after: String,
        count: u32,
        json: String,
        rng: &'rng mut ThreadRng,
    ) -> Self {
        Self { after, before, count, json, rng }
    }
}
