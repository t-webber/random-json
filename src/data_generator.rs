//! Wrapper around [`DataGenerator`] for internal usage.

use rand::Rng as _;
use rand::distr::uniform::{SampleRange, SampleUniform};
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom as _;
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
    /// Chooses an element from a slice with the random generator
    pub fn choose<'values, T>(&mut self, values: &'values [T]) -> Option<&'values T> {
        match self {
            Self::NonDeterministic(data_generator) => values.choose(data_generator.rng()),
            Self::Seeded(data_generator) => values.choose(data_generator.rng()),
        }
    }

    /// Creates a new [`RandomDataGenerator`] with a seed or not.
    pub fn new(seed: Option<u64>) -> Self {
        seed.map_or_else(
            || Self::NonDeterministic(DataGenerator::default()),
            |inner| Self::Seeded(Box::new(DataGenerator::new_with_seed(inner))),
        )
    }

    /// Chooses a boolean with the given weight with the random generator
    pub fn random_bool(&mut self, weight: f64) -> bool {
        match self {
            Self::NonDeterministic(generator) => generator.rng().random_bool(weight),
            Self::Seeded(generator) => generator.rng().random_bool(weight),
        }
    }

    /// Chooses an element from a range with the random generator
    pub fn random_range<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        match self {
            Self::NonDeterministic(generator) => generator.rng().random_range(range),
            Self::Seeded(generator) => generator.rng().random_range(range),
        }
    }

    /// Generates a value randomly from the given [`DataType`]
    pub fn random_value(&mut self, data_type: DataType) -> String {
        match self {
            Self::NonDeterministic(data_generator) => data_type.random(data_generator),
            Self::Seeded(data_generator) => data_type.random(data_generator),
        }
    }
}
