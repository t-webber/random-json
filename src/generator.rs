//! Define traits to apply the data generator on all sorts of types.

use std::collections::HashMap;

use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom as _;
use rand::{Rng as _, rng};
use random_data::{DataGenerator, DataType};

use crate::errors::{Error, Res};
///
/// Generate random data of the given type.
pub trait Generator: Sized {
    /// Generate random data of the given type.
    fn generate(&self, data: &mut Data) -> Res<Self>;
}

/// Generate random data of the given type, but with a nullable type.
pub trait NullableGenerator: Sized {
    /// Generate random data of the given type, but with a nullable type.
    ///
    /// This can sometimes returns None.
    fn generate_nullable(&self, data: &mut Data) -> Res<Option<Self>>;
}

/// Contains the list of data types and the random generator to apply
/// generators.
pub struct Data {
    /// Radnom data generator
    random_data_generator: DataGenerator,
    /// Random generator
    rng: ThreadRng,
    /// User-defined data types
    user_defined: HashMap<String, Vec<String>>,
}

impl Data {
    /// Generate non-nullable data of the provided data type.
    fn generate(&mut self, data_type: &str) -> Res<String> {
        if let Some(values) = self.user_defined.get(data_type) {
            values
                .choose(&mut self.rng)
                .ok_or(Error::FakerDefEmpty)
                .map(ToOwned::to_owned)
        } else {
            Ok(DataType::try_from(data_type)
                .map_err(|()| Error::InvalidDataType(data_type.to_owned()))?
                .random(&mut self.random_data_generator))
        }
    }

    /// Generate nullable data of the provided data type.
    fn generate_nullable(&mut self, data_type: &str) -> Res<Option<String>> {
        let parsed_data_type = if let Some(parsed_data_type) = data_type.strip_suffix('?') {
            if self.rng.random_bool(0.3) {
                return Ok(None);
            }
            parsed_data_type
        } else {
            data_type
        };

        self.generate(parsed_data_type).map(Some)
    }

    /// List all the data types, user defined and from `random-data`.
    pub fn list(&self) -> Vec<String> {
        let random_data_types = DataType::list_str();

        let mut list = Vec::with_capacity(
            self.user_defined
                .len()
                .saturating_add(random_data_types.len()),
        );

        self.user_defined.keys().for_each(|key| {
            list.push(key.to_owned());
        });

        for data_type in random_data_types {
            list.push((*data_type).to_owned());
        }

        list
    }

    /// Build the [`Data`] handler from user inputs
    pub fn new(input_data: Vec<String>) -> Res<Self> {
        let mut user_defined = HashMap::new();

        for data_type in input_data {
            let (name, values) = Self::parse_user_defined(&data_type)?;

            if user_defined.insert(name, values).is_some() {
                return Err(Error::DuplicateDataType(data_type));
            }
        }

        Ok(Self { random_data_generator: DataGenerator::new(), user_defined, rng: rng() })
    }

    /// Parse a user-defined data-type, with the format
    /// `Name:Value1|Value2|Value3`.
    #[expect(clippy::unwrap_in_result, reason = "unwrap_used lint is active")]
    fn parse_user_defined(user_input: &str) -> Res<(String, Vec<String>)> {
        let mut split = user_input.split(':');

        #[expect(clippy::unwrap_used, reason = "slipt always has first element")]
        let name = split.next().unwrap();

        let Some(values) = split.next() else {
            return Err(Error::FakerDefMissingColon);
        };

        if split.next().is_some() {
            return Err(Error::FakerDefTooManyColons);
        }

        Ok((name.to_owned(), values.split('|').map(str::to_owned).collect()))
    }

    /// Borrows the random generator as mutable.
    pub const fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }
}

impl Generator for String {
    fn generate(&self, data: &mut Data) -> Res<Self> {
        data.generate(self)
    }
}

impl NullableGenerator for String {
    fn generate_nullable(&self, data: &mut Data) -> Res<Option<Self>> {
        data.generate_nullable(self)
    }
}
