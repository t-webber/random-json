use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme;
use rand::rngs::ThreadRng;

use crate::data::auto::FAKERS;
use crate::data::generate::generate_data;
use crate::errors::Res;

pub fn generate_from_dialog(rng: &mut ThreadRng) -> Res<String> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose a data type")
        .items(&FAKERS)
        .interact()?;

    Ok(generate_data(FAKERS[selection], rng))
}
