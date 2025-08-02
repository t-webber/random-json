//! Generate data by selecting the data type from a dialogue.

pub mod range;

use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme as ColourfulTheme;
use rand::rngs::ThreadRng;

use crate::errors::Res;
use crate::json::generator::Generator as _;

/// Dialog to fuzzy search, select and generate some data of a data type.
pub struct Dialog<'fakers> {
    /// List of possible data types
    fakers: &'fakers [&'static str],
}

impl<'fakers> From<&'fakers Vec<&'static str>> for Dialog<'fakers> {
    fn from(fakers: &'fakers Vec<&'static str>) -> Self {
        Self { fakers }
    }
}

impl Dialog<'_> {
    /// Generate data from a dialogue selection.
    #[expect(clippy::indexing_slicing, reason = "can't be out of bounds")]
    pub fn generate(&self, rng: &mut ThreadRng) -> Res<String> {
        let selection = FuzzySelect::with_theme(&ColourfulTheme::default())
            .with_prompt("Choose a data type")
            .items(self.fakers)
            .interact()?;

        self.fakers[selection].to_owned().generate(rng)
    }
}
