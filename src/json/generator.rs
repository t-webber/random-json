//! Implement the generator traits for JSON values

use core::iter::repeat_with;

use serde_json::{Map, Value};

use crate::data::Data;
use crate::errors::{Error, Res};
use crate::generator_trait::{Generator, NullableGenerator};

impl Generator<Value> for Map<String, Value> {
    fn generate(&self, data: &mut Data) -> Res<Value> {
        let mut new_map = Self::with_capacity(self.len());
        for (key, json_value) in self {
            if let Some(parsed_key) = key.strip_suffix('!') {
                new_map.insert(parsed_key.to_owned(), json_value.to_owned());
                continue;
            }

            let parsed_key = if let Some(parsed_key) = key.strip_suffix('?') {
                if data.random_null() {
                    continue;
                }
                parsed_key
            } else {
                key
            };
            if let Some(generated_value) = json_value.generate_nullable(data)? {
                new_map.insert(parsed_key.to_owned(), generated_value);
            }
        }
        Ok(Value::Object(new_map))
    }
}

impl Generator<Value> for Vec<Value> {
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
    fn generate(&self, data: &mut Data) -> Res<Value> {
        let mut iter = self.iter();

        let array_item_type = iter.next().ok_or(Error::ArrayMissingDataType)?;

        let len = match (iter.next(), iter.next()) {
            (None, _) => data.random_range(1..10),
            (Some(Value::Number(inf)), Some(Value::Number(sup))) =>
                data.random_range(number_to_int(inf)?..number_to_int(sup)?),
            (Some(Value::Number(inf)), None) => number_to_int(inf)?,
            (Some(Value::Number(_)), Some(value)) | (Some(value), _) =>
                return Err(Error::ExpectedInteger(value.to_owned())),
        };

        repeat_with(|| array_item_type.generate(data))
            .take(len)
            .collect()
    }
}

impl Generator<Self> for Value {
    fn generate(&self, data: &mut Data) -> Res<Self> {
        match self {
            Self::Null | Self::Bool(_) | Self::Number(_) =>
                Err(Error::InvalidSchemaType(format!("{self:?}"))),
            Self::String(data_type) => data_type.generate(data).map(TryInto::try_into)?,
            Self::Array(values) => values.generate(data),
            Self::Object(object) => object.generate(data),
        }
    }
}

impl NullableGenerator<Self> for Value {
    fn generate_nullable(&self, data: &mut Data) -> Res<Option<Self>> {
        let generated_json = match self {
            Self::Null | Self::Bool(_) | Self::Number(_) =>
                return Err(Error::InvalidSchemaType(format!("{self:?}"))),
            Self::String(data_type) =>
                if let Some(value) = data_type.generate_nullable(data)? {
                    value.try_into()?
                } else {
                    return Ok(None);
                },
            Self::Array(values) => values.generate(data)?,
            Self::Object(object) => object.generate(data)?,
        };

        Ok(Some(generated_json))
    }
}

/// Tries to convert a [`serde_json::Number`] to a [`usize`]
fn number_to_int(json_number: &serde_json::Number) -> Res<usize> {
    let number = json_number
        .as_u64()
        .ok_or_else(|| Error::NumberNotAnInteger(json_number.to_owned()))?;

    number
        .try_into()
        .map_err(|error| Error::ArrayInvalidLength { original: number, error })
}
