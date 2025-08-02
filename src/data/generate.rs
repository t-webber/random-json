//! Module to generate data from either custom data types or from those defined
//! in the `fake` library.

use rand::Rng as _;
use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom as _;

use super::auto::apply_fake;
use crate::errors::Res;
use crate::json::generator::{Generator, NullableGenerator};

/// List of positions to be used in the manual data generation.
const POSITIONS: [&str; 5] = ["Tr\u{e9}sorier", "VPO", "SecGe", "DirCo", "Info"];

impl NullableGenerator for String {
    fn generate_nullable(&self, rng: &mut ThreadRng) -> Res<Option<Self>> {
        let parsed_data_type = if let Some(parsed_data_type) = self.strip_suffix('?') {
            if rng.random_bool(0.3) {
                return Ok(None);
            }
            parsed_data_type
        } else {
            self
        };

        manual(parsed_data_type, rng)
            .map_or_else(|| apply_fake(parsed_data_type), Ok)
            .map(Some)
    }
}

impl Generator for String {
    fn generate(&self, rng: &mut ThreadRng) -> Res<Self> {
        manual(self, rng).map_or_else(|| apply_fake(self), Ok)
    }
}

/// Generate a custom data type.
#[expect(clippy::unwrap_used, reason = "POSITIONS is not empty")]
fn manual(data_type: &str, rng: &mut ThreadRng) -> Option<String> {
    match data_type {
        "Position" => Some((*POSITIONS.choose(rng).unwrap()).to_owned()),
        _ => None,
    }
}
