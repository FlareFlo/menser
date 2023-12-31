use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::Relaxed;

const MENSA_1: (usize, &str) = (102, "Mensa 1");
const MENSA_2: (usize, &str) = (105, "Mensa 2");

// const MENSA_360: (usize, &str) = (111, "360°"); 360 is dead :(

pub const TO_FETCH: &[(usize, &str)] = &[MENSA_1, MENSA_2];

pub static LOWER_PRICE_THRESHOLD: AtomicU16 = AtomicU16::new(200); // Do not display things cheaper than this

pub fn get_lower_threshold_float() -> f64 {
	LOWER_PRICE_THRESHOLD.load(Relaxed) as f64 / 100.0
}

pub fn get_lower_threshold_int() -> u16 {
	LOWER_PRICE_THRESHOLD.load(Relaxed)
}


pub const BASE_DOMAIN: &str = "https://sls.api.stw-on.de";