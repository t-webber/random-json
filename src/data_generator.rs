//! Wrapper around [`DataGenerator`] for internal usage.

use rand::rngs::ThreadRng;
use rand_chacha::ChaCha20Rng;
use random_data::{DataGenerator, DataType};

/// Wrapper around [`DataGenerator`] to hide a different random generator,
/// depending on whether we wand to generate with a seed or with a non
/// determinastic generator.
pub enum RandomDataGenerator {
    /// Non-deterministic generator
    NonDeterministic(DataGenerator<ThreadRng>),
    /// Determinastic with a seed
    Seeded(Box<DataGenerator<ChaCha20Rng>>),
}

impl RandomDataGenerator {
    /// Creates a new [`RandomDataGenerator`] with a seed or not.
    pub fn new(seed: Option<u64>) -> Self {
        seed.map_or_else(
            || Self::NonDeterministic(DataGenerator::default()),
            |inner| Self::Seeded(Box::new(DataGenerator::new_with_seed(inner))),
        )
    }

    /// Generates a value randomly from the given [`DataType`]
    pub fn random_value(&mut self, data_type: DataType) -> String {
        match self {
            Self::NonDeterministic(data_generator) => data_type.random(data_generator),
            Self::Seeded(data_generator) => data_type.random(data_generator),
        }
    }
}
