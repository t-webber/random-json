//! Implement the generator traits for JSON values

use core::iter::repeat_with;

use rand::Rng as _;
use serde_json::{Map, Value};

use crate::errors::{Error, Res};
use crate::generator::{Fakers, Generator, NullableGenerator};

impl Generator for Map<String, Value> {
    fn generate(&self, fakers: &mut Fakers) -> Res<Self> {
        let mut new_map = Self::with_capacity(self.len());
        for (key, json_value) in self {
            if let Some(generated_value) = json_value.generate_nullable(fakers)? {
                new_map.insert(key.to_owned(), generated_value);
            }
        }
        Ok(new_map)
    }
}

impl Generator for Vec<Value> {
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
    fn generate(&self, fakers: &mut Fakers) -> Res<Self> {
        let mut iter = self.iter();

        let array_item_type = iter.next().ok_or(Error::ArrayMissingDataType)?;

        let len = match (iter.next(), iter.next()) {
            (None, _) => fakers.rng().random_range(1..10),
            (Some(Value::Number(inf)), Some(Value::Number(sup))) => fakers
                .rng()
                .random_range(number_to_int(inf)?..number_to_int(sup)?),
            (Some(Value::Number(inf)), None) => number_to_int(inf)?,
            (Some(Value::Number(_)), Some(value)) | (Some(value), _) =>
                return Err(Error::ExpectedInteger(value.to_owned())),
        };

        repeat_with(|| array_item_type.generate(fakers))
            .take(len)
            .collect()
    }
}

impl Generator for Value {
    fn generate(&self, fakers: &mut Fakers) -> Res<Self> {
        let generated_json = match self {
            Self::Null | Self::Bool(_) | Self::Number(_) =>
                return Err(Error::InvalidSchemaType(format!("{self:?}"))),
            Self::String(data_type) => Self::String(data_type.generate(fakers)?),
            Self::Array(values) => Self::Array(values.generate(fakers)?),
            Self::Object(object) => Self::Object(object.generate(fakers)?),
        };

        Ok(generated_json)
    }
}

impl NullableGenerator for Value {
    fn generate_nullable(&self, fakers: &mut Fakers) -> Res<Option<Self>> {
        let generated_json = match self {
            Self::Null | Self::Bool(_) | Self::Number(_) =>
                return Err(Error::InvalidSchemaType(format!("{self:?}"))),
            Self::String(data_type) =>
                if let Some(data) = data_type.generate_nullable(fakers)? {
                    Self::String(data)
                } else {
                    return Ok(None);
                },
            Self::Array(values) => Self::Array(values.generate(fakers)?),
            Self::Object(object) => Self::Object(object.generate(fakers)?),
        };

        Ok(Some(generated_json))
    }
}

/// Tries to convert a [`serde_json::Number`] to a [`usize`]
fn number_to_int(json_number: &serde_json::Number) -> Res<usize> {
    let number = json_number
        .as_u64()
        .ok_or(Error::NumberNotAnInteger(json_number.to_owned()))?;

    number
        .try_into()
        .map_err(|error| Error::ArrayInvalidLength { original: number, error })
}
