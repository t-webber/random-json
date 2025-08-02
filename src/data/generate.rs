//! Module to generate data from either custom data types or from those defined
//! in the `fake` library.

use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom as _;

use super::auto::apply_fake;
use crate::errors::Res;

/// List of positions to be used in the manual data generation.
const POSITIONS: [&str; 5] = ["Tr\u{e9}sorier", "VPO", "SecGe", "DirCo", "Info"];

/// Generate a custom data type.
#[expect(clippy::unwrap_used, reason = "POSITIONS is not empty")]
fn manual(data_type: &str, rng: &mut ThreadRng) -> Option<String> {
    match data_type {
        "Position" => Some((*POSITIONS.choose(rng).unwrap()).to_owned()),
        _ => None,
    }
}

/// Generate data based on the provided data type, whether it be a custom type
/// or one from the `fake` library.
pub fn generate_data(data_type: &str, rng: &mut ThreadRng) -> Res<String> {
    manual(data_type, rng).map_or_else(|| apply_fake(data_type), Ok)
}
