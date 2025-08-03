//! Define traits to apply the data generator on all sorts of types.

use std::collections::HashMap;

use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom as _;
use rand::{Rng as _, rng};

use crate::data::auto::apply_fake;
use crate::errors::{Error, Res};
///
/// Generate random data of the given type.
pub trait Generator: Sized {
    /// Generate random data of the given type.
    fn generate(&self, fakers: &mut Fakers) -> Res<Self>;
}

/// Generate random data of the given type, but with a nullable type.
pub trait NullableGenerator: Sized {
    /// Generate random data of the given type, but with a nullable type.
    ///
    /// This can sometimes returns None.
    fn generate_nullable(&self, fakers: &mut Fakers) -> Res<Option<Self>>;
}

/// Contains the list of data types and the random generator to apply
/// generators.
pub struct Fakers {
    /// Data types from the `fake` crate
    fake_crate: Vec<&'static str>,
    /// Random generator
    rng: ThreadRng,
    /// User-defined data types
    user_defined: HashMap<String, Vec<String>>,
}

impl Fakers {
    /// Generate non-nullable data of the provided data type.
    fn generate(&mut self, data_type: &str) -> Res<String> {
        if let Some(faker) = self.user_defined.get(data_type) {
            faker
                .choose(&mut self.rng)
                .ok_or(Error::FakerDefEmpty)
                .map(ToOwned::to_owned)
        } else {
            apply_fake(data_type)
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

    /// List all the data types, user defined and from `fake`.
    pub fn list(&self) -> Vec<String> {
        let len = self
            .fake_crate
            .len()
            .saturating_add(self.user_defined.len());
        let mut fakers_list = Vec::with_capacity(len);
        fakers_list.extend(self.user_defined.keys().map(String::to_owned));
        for faker in &self.fake_crate {
            fakers_list.push((*faker).to_owned());
        }
        fakers_list
    }

    /// Build the [`Faker`] from arguments
    #[expect(clippy::unwrap_in_result, reason = "unwrap_used lint is active")]
    pub fn new(fake_crate: Vec<&'static str>, input_fakers: Vec<String>) -> Res<Self> {
        let mut user_defined = HashMap::new();
        for faker in input_fakers {
            let mut split = faker.split(':');
            #[expect(clippy::unwrap_used, reason = "slipt always has first element")]
            let faker_name = split.next().unwrap();
            let Some(faker_values) = split.next() else {
                return Err(Error::FakerDefMissingColon);
            };
            if split.next().is_some() {
                return Err(Error::FakerDefTooManyColons);
            }
            user_defined.insert(
                faker_name.to_owned(),
                faker_values.split('|').map(str::to_owned).collect(),
            );
        }

        Ok(Self { user_defined, rng: rng(), fake_crate })
    }

    /// Borrows the random generator as mutable.
    pub const fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }
}

impl Generator for String {
    fn generate(&self, fakers: &mut Fakers) -> Res<Self> {
        fakers.generate(self)
    }
}

impl NullableGenerator for String {
    fn generate_nullable(&self, fakers: &mut Fakers) -> Res<Option<Self>> {
        fakers.generate_nullable(self)
    }
}
