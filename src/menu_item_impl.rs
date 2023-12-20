use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use time::Weekday;
use crate::api_schema::MenuItem;
use crate::constants;

impl MenuItem {
	// Needs weekday to compute opening hours
	pub fn longest_menu_name(selfish: &[Self], weekday: Weekday) -> Result<usize, Report> {
		let res = selfish.iter()
			.map(|e| e.menu.longest_menu_name())
			.collect::<Result<Vec<usize>, Report>>()?
			.into_iter()
			.max()
			.context("Zero menu items")?;
		let title = selfish.iter()
			.next()
			.context("Empty menu item vector")?
			.format_title(weekday)?.len();
		Ok(res.max(title))
	}

	pub fn format_title(&self, weekday: Weekday) -> Result<String, Report> {
		let formatted_opening_hours = self.menu.meals.iter()
			.next()
			.map(|e| e.location.format_opening_hours(weekday))
			.context("Zero meals found for opening hours")?;

		let filtered_meals_count = self.menu.count_filtered_meals();
		Ok(format!("{} | (excluding {filtered_meals_count} item{} less than {}€) | open: {formatted_opening_hours}{}",
				   self.mensa_name,
				   if filtered_meals_count > 1 { "s" } else { "" },
				   constants::get_lower_threshold(),
				   if filtered_meals_count == 0 { " | (presumed closed)" } else { "" }
		))
	}
}