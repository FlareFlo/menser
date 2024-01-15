use std::sync::atomic::AtomicU16;
use std::sync::atomic::Ordering::Relaxed;

use cli_table::Color;
use cli_table::Color::Rgb;

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

pub mod colors {
	use cli_table::Color;
	use cli_table::Color::Rgb;

	pub const LOWER_SAXONY: Color = Rgb(255, 233, 0);
	pub const PIZZA: Color = Color::Magenta;
	pub const DEFAULT_MEAL: Color = Color::Green;
	pub const TITLE: Color = Color::Cyan;
}

pub fn compute_price_color(price: u16, most_expensive_price: f64) -> Color {
	let price = price as f64 / 100.0;
	let lerp_color = |x: f64| (1.1 * x + 33.0).round() as u8;
	let lerp_price = |x: f64| (x - get_lower_threshold_float()) / (most_expensive_price - get_lower_threshold_float()) * 100.0;
	Rgb(lerp_color(lerp_price(price)), lerp_color(100.0 - lerp_price(price)), 33)
}