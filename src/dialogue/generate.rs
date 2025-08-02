//! Generate data by selecting the data type from a dialogue.

use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme as ColourfulTheme;
use rand::rngs::ThreadRng;

use crate::data::generate::generate_data;
use crate::errors::Res;

/// Generate data from a dialogue selection.
#[expect(clippy::indexing_slicing, reason = "can't be out of bounds")]
pub fn generate_from_dialogue(rng: &mut ThreadRng, fakers: &[&str]) -> Res<String> {
    let selection = FuzzySelect::with_theme(&ColourfulTheme::default())
        .with_prompt("Choose a data type")
        .items(fakers)
        .interact()?;

    generate_data(fakers[selection], rng)
}
