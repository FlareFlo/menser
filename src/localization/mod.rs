use crate::locale_fn;
use std::ops::Deref;
use std::sync::RwLock;
mod english;
mod pirate;

#[derive(Copy, Clone, Debug, Default)]
pub enum Localization {
    #[default]
    English,
    Pirate,
}

static LOCALE: RwLock<Localization> = RwLock::new(Localization::English);

locale_fn!(no_menus);
locale_fn!(price_cell_label);
locale_fn!(fetched_from);

pub trait Localizer {
    fn no_menus() -> &'static str;
    fn price_cell_label() -> &'static str;
    fn fetched_from() -> &'static str;
}

#[macro_export]
macro_rules! locale_fn {
    ($fn_name:ident) => {
        pub fn $fn_name() -> &'static str {
            match &LOCALE.read().unwrap().deref() {
                Localization::English => english::ENLocalizer::$fn_name(),
                Localization::Pirate => pirate::PirateLocalizer::$fn_name(),
            }
        }
    };
}
