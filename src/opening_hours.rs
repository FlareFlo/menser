use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Location {
	pub opening_hours: Vec<OpeningHours>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OpeningHours {
	pub start_day: usize,
	pub end_day: usize,
	pub start_time: String,
	pub end_time: String,
}

impl Location {

}

impl OpeningHours {
}

impl Display for OpeningHours {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let trim_time = |time: &String| time[..5].to_owned();
		write!(f, "{}-{}", trim_time(&self.start_time), trim_time(&self.end_time))
	}
}