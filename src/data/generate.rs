use rand::rngs::ThreadRng;
use rand::seq::IndexedRandom as _;

use super::auto::apply_fake;

const POSITIONS: [&str; 5] = ["Trésorier", "VPO", "SecGe", "DirCo", "Info"];

fn manual(data_type: &str, rng: &mut ThreadRng) -> Option<String> {
    match data_type {
        "Position" => Some(
            POSITIONS
                .choose(rng)
                .expect("POSITIONS not empty")
                .to_string(),
        ),
        _ => None,
    }
}

pub fn generate_data(data_type: &str, rng: &mut ThreadRng) -> String {
    manual(data_type, rng).unwrap_or_else(|| apply_fake(data_type).unwrap())
}
