use dialoguer::{FuzzySelect, theme::ColorfulTheme};
use rand::rngs::ThreadRng;

use crate::data::{auto::FAKERS, generate::generate_data};

pub fn generate_from_dialog(rng: &mut ThreadRng) -> String {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a data type")
        .items(&FAKERS)
        .interact()
        .unwrap();

    generate_data(FAKERS[selection], rng)
}
