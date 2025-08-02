//! Generator for when a JSON schema file is provided.

pub mod generator;

use core::fmt::Write as _;

use rand::rngs::ThreadRng;
use serde_json::Value;

use crate::errors::{Error, Res};
use crate::json::generator::NullableGenerator as _;

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
            let generate_json = json.generate_nullable(self.rng)?.unwrap_or_default();
            let generate_json_str =
                serde_json::to_string_pretty(&generate_json).map_err(Error::DeserializeJson)?;
            writeln!(generated_data, "{}{generate_json_str}{}", self.before, self.after)
                .map_err(Error::JsonWriteString)?;
        }

        Ok(generated_data)
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
