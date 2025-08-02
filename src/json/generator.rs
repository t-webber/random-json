//! Define traits to apply the data generator on all sorts of types.

use core::iter::repeat_with;

use rand::Rng as _;
use rand::rngs::ThreadRng;
use serde_json::{Map, Value};

use crate::errors::{Error, Res};

/// Generate random data of the given type.
pub trait Generator: Sized {
    /// Generate random data of the given type.
    fn generate(&self, rng: &mut ThreadRng) -> Res<Self>;
}

/// Generate random data of the given type, but with a nullable type.
pub trait NullableGenerator: Sized {
    /// Generate random data of the given type, but with a nullable type.
    ///
    /// This can sometimes returns None.
    fn generate_nullable(&self, rng: &mut ThreadRng) -> Res<Option<Self>>;
}

impl Generator for Map<String, Value> {
    fn generate(&self, rng: &mut ThreadRng) -> Res<Self> {
        let mut new_map = Self::with_capacity(self.len());
        for (key, json_value) in self {
            if let Some(generated_value) = json_value.generate_nullable(rng)? {
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
    fn generate(&self, rng: &mut ThreadRng) -> Res<Self> {
        let mut iter = self.iter();

        let array_item_type = iter.next().ok_or(Error::MissingArrayDataType)?;

        let len = match (iter.next(), iter.next()) {
            (None, _) => rng.random_range(1..10),
            (Some(Value::Number(inf)), Some(Value::Number(sup))) =>
                rng.random_range(number_to_int(inf)?..number_to_int(sup)?),
            (Some(Value::Number(inf)), None) => number_to_int(inf)?,
            (Some(Value::Number(_)), Some(value)) | (Some(value), _) =>
                return Err(Error::ExpectedInteger(value.to_owned())),
        };

        repeat_with(|| array_item_type.generate(rng))
            .take(len)
            .collect()
    }
}

impl Generator for Value {
    fn generate(&self, rng: &mut ThreadRng) -> Res<Self> {
        let generated_json = match self {
            Self::Null | Self::Bool(_) | Self::Number(_) =>
                return Err(Error::InvalidSchemaType(format!("{self:?}"))),
            Self::String(data_type) => Self::String(data_type.generate(rng)?),
            Self::Array(values) => Self::Array(values.generate(rng)?),
            Self::Object(object) => Self::Object(object.generate(rng)?),
        };

        Ok(generated_json)
    }
}

impl NullableGenerator for Value {
    fn generate_nullable(&self, rng: &mut ThreadRng) -> Res<Option<Self>> {
        let generated_json = match self {
            Self::Null | Self::Bool(_) | Self::Number(_) =>
                return Err(Error::InvalidSchemaType(format!("{self:?}"))),
            Self::String(data_type) =>
                if let Some(data) = data_type.generate_nullable(rng)? {
                    Self::String(data)
                } else {
                    return Ok(None);
                },
            Self::Array(values) => Self::Array(values.generate(rng)?),
            Self::Object(object) => Self::Object(object.generate(rng)?),
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
        .map_err(|error| Error::U64ToUsize { original: number, error })
}
