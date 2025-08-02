//! Generator for when a JSON schema file is provided.

use core::fmt::Write as _;
use core::iter::repeat_with;

use rand::Rng as _;
use rand::rngs::ThreadRng;
use serde_json::{Map, Value};

use crate::data::generate::{generate_data_non_nullable, generate_data_nullable};
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
            Value::String(data_type) =>
                if let Some(data) = generate_data_nullable(data_type, rng)? {
                    Value::String(data)
                } else {
                    return Ok(None);
                },
            Value::Array(values) => Value::Array(Self::generate_vec(values, rng)?),
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

    /// Generate a vec with random data
    ///
    /// The vec must have the following format: `[data_type, min_nb_elts,
    /// max_nb_elts+1]`.
    ///
    /// Example:
    ///
    /// ```
    /// ["FreeEmail"] // produce a random number of emails
    /// ["FirstName", 1] // produce 1 first name
    /// ["LicencePlate", 1, 10] // produce between 1 and 9 licence plates
    fn generate_vec(values: &[Value], rng: &mut ThreadRng) -> Res<Vec<Value>> {
        let mut iter = values.iter();

        let data_type_value = iter.next().ok_or(Error::MissingArrayDataType)?;
        let Value::String(data_type_str) = data_type_value else {
            return Err(Error::InvalidArrayDataType(data_type_value.to_owned()));
        };

        let len = match (iter.next(), iter.next()) {
            (None, _) => rng.random_range(1..10),
            (Some(Value::Number(inf)), Some(Value::Number(sup))) =>
                rng.random_range(number_to_int(inf)?..number_to_int(sup)?),
            (Some(Value::Number(inf)), None) => number_to_int(inf)?,
            (Some(Value::Number(_)), Some(value)) | (Some(value), _) =>
                return Err(Error::ExpectedInteger(value.to_owned())),
        };

        repeat_with(|| generate_data_non_nullable(data_type_str, rng).map(Value::String))
            .take(len)
            .collect()
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

/// Tries to convert a [`serde_json::Number`] to a [`usize`]
fn number_to_int(json_number: &serde_json::Number) -> Res<usize> {
    let number = json_number
        .as_u64()
        .ok_or(Error::NumberNotAnInteger(json_number.to_owned()))?;

    number
        .try_into()
        .map_err(|error| Error::U64ToUsize { original: number, error })
}
