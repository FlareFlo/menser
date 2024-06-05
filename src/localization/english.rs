use crate::localization::{Localization, Localizer};

pub struct ENLocalizer {}

impl Localizer for ENLocalizer {
    fn no_menus() -> &'static str {
        "No menus in any weekday found"
    }

    fn price_cell_label() -> &'static str {
        "Price €"
    }

    fn fetched_from() -> &'static str {
        "Fetched from"
    }
}
