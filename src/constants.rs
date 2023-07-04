const MENSA_1: (usize, &str) = (101, "Mensa 1");
const MENSA_2: (usize, &str) = (105, "Mensa 2");
const MENSA_360: (usize, &str) = (111, "360°");

pub const WEEKDAYS: &[&str] = &["monday", "tuesday", "wednesday", "thursday", "friday"];

pub const TO_FETCH: &[(usize, &str)] = &[MENSA_1, MENSA_2, MENSA_360];

pub const LOWER_PRICE_THRESHOLD: f64 = 2.0; // Do not display things cheaper than this

pub const BASE_DOMAIN: &str = "https://sls.api.stw-on.de";