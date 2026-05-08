//! Generator for when a JSON schema file is provided.

mod generator;

use color_eyre::eyre::Context as _;
use serde_json::Value;

use crate::Res;
use crate::data::Data;
use crate::generator_trait::NullableGenerator as _;

/// Arguments for generating JSON data based on a schema file.
pub struct JsonArgs {
    /// String to print after every data generation of the JSON schema.
    after: String,
    /// String to print before every data generation of the JSON schema.
    before: String,
    /// Number of times to repeat the JSON generation.
    count: u32,
    /// Data generator
    data: Data,
    /// JSON schema content
    json: String,
}

impl JsonArgs {
    /// Generate the JSON data based on the schema file and the provided
    /// parameters.
    pub fn generate(mut self) -> Res<String> {
        let json: Value = serde_json::from_str(&self.json).context("Failed to deserialise json")?;

        let mut generated_data = String::new();
        let len = self
            .before
            .len()
            .saturating_add(self.after.len())
            .saturating_add(1);
        for _ in 0..self.count {
            let generate_json = json.generate_nullable(&mut self.data)?.unwrap_or_default();
            let generate_json_str =
                serde_json::to_string_pretty(&generate_json).context("Failed to serialise json")?;
            generated_data.reserve(generate_json_str.len().saturating_add(len));
            generated_data.push_str(&self.before);
            generated_data.push_str(&generate_json_str);
            generated_data.push_str(&self.after);
            generated_data.push('\n');
        }

        Ok(generated_data)
    }

    /// Create a new instance of `JsonArgs` with the provided parameters.
    pub const fn new(before: String, after: String, count: u32, json: String, data: Data) -> Self {
        Self { after, before, count, data, json }
    }
}
