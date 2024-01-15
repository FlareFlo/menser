use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};
use time::Weekday;

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
	pub fn format_opening_hours(&self, weekday: Weekday) -> String {
		self.opening_hours.iter()
			.filter(|e| e.weekday_in_range(weekday))
			.map(|e| e.to_string())
			.collect::<Vec<String>>()
			.join(", ")
	}
}

impl OpeningHours {
	pub fn weekday_in_range(&self, weekday: Weekday) -> bool {
		(self.start_day..=self.end_day).contains(&(weekday.number_days_from_sunday() as _))
	}
	pub fn formatting_dummy() -> Self {
		OpeningHours {
			start_day: 0,
			end_day: 7,
			start_time: "00:00".to_string(),
			end_time: "24:59".to_string(),
		}
	}
}

impl Display for OpeningHours {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let trim_time = |time: &String| time[..5].to_owned();
		write!(f, "{}-{}", trim_time(&self.start_time), trim_time(&self.end_time))
	}
}