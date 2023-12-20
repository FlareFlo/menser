use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

const MENSA_1: (usize, &str) = (101, "Mensa 1");
const MENSA_2: (usize, &str) = (105, "Mensa 2");
const MENSA_360: (usize, &str) = (111, "360°");

pub const TO_FETCH: &[(usize, &str)] = &[MENSA_1, MENSA_2, MENSA_360];

pub static LOWER_PRICE_THRESHOLD: AtomicU32 = AtomicU32::new(200); // Do not display things cheaper than this

pub fn get_lower_threshold() -> f64 {
	LOWER_PRICE_THRESHOLD.load(Relaxed) as f64 / 100.0
}

pub const BASE_DOMAIN: &str = "https://sls.api.stw-on.de";