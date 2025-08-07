//! Generate data by selecting the data type from a dialogue.

//pub mod range;

use dialoguer::FuzzySelect;
use dialoguer::theme::ColorfulTheme as ColourfulTheme;

use crate::errors::Res;
use crate::generator::{Data, Generator as _};

/// Dialog to fuzzy search, select and generate some data of a data type.
pub struct Dialog;

impl Dialog {
    /// Generate data from a dialogue selection.
    #[expect(clippy::indexing_slicing, reason = "can't be out of bounds")]
    pub fn generate(mut data: Data) -> Res<String> {
        let data_list = data.list();

        let selection = FuzzySelect::with_theme(&ColourfulTheme::default())
            .with_prompt("Choose a data type")
            .items(&data_list)
            .interact()?;

        data_list[selection].generate(&mut data)
    }
}
