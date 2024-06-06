use crate::constants::{set_lower_threshold_int, set_upper_threshold_int, DEFAULT_PRICE_THRESHOLD};
use std::env;

#[derive(Default)]
pub struct Args {
    pub tomorrow: bool,
}

pub fn argparse() -> color_eyre::Result<Args> {
    let mut args = Args::default();
    let raw_args = env::args();

    for raw in raw_args {
        match raw.to_lowercase().as_str() {
            "tomorrow" => args.tomorrow = true,
            "sides" => set_lower_threshold_int(0),
            "sides_only" => {
                set_upper_threshold_int(DEFAULT_PRICE_THRESHOLD);
                set_lower_threshold_int(0);
            }
            _ => {}
        }
    }

    Ok(args)
}
