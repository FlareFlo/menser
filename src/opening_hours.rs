use serde::{Deserialize, Serialize};
use time::Weekday;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
	pub opening_hours: Vec<OpeningHours>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OpeningHours {
	pub start_day: usize,
	pub end_day: usize,
	pub start_time: String,
	pub end_time: String,
}

impl Location {
	pub fn format_opening_hours(&self, weekday: Weekday) -> String {
		let trim_time = |time: &String| time[..5].to_owned();

		self.opening_hours.iter()
			.filter(|e|e.weekday_in_range(weekday))
			.map(|e|format!("{}-{}", trim_time(&e.start_time), trim_time(&e.end_time)))
			.collect::<Vec<String>>()
			.join(", ")
	}
}

impl OpeningHours {
	pub fn weekday_in_range(&self, weekday: Weekday) -> bool {
		(self.start_day..=self.end_day).contains(&(weekday.number_days_from_sunday() as _))
	}
}