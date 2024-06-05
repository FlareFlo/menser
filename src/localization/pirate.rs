use crate::localization::{Localization, Localizer};

pub struct PirateLocalizer {}

impl Localizer for PirateLocalizer {
    fn no_menus() -> &'static str {
        "Barnacle! There be no meals today!"
    }

    fn price_cell_label() -> &'static str {
        "Doubloons"
    }

    fn fetched_from() -> &'static str {
        "Plundered from"
    }
}
