use color_eyre::eyre::ContextCompat;
use color_eyre::Report;

use crate::api_schema::MensaMenu;
use crate::constants;
use crate::opening_hours::OpeningHours;

impl MensaMenu {
	// Needs weekday to compute opening hours
	pub fn longest_menu_name(selfish: &[Self]) -> Result<usize, Report> {
		let res = selfish.iter()
			.map(|e| e.menu.longest_menu_name())
			.collect::<Result<Vec<usize>, Report>>()?
			.into_iter()
			.max()
			.context("Zero menu items")?;
		let title = selfish.iter()
			.next()
			.context("Empty menu item vector")?
			.format_title("noon")?.len();
		Ok(res.max(title))
	}

	pub fn format_title(&self, daytime: &str) -> Result<String, Report> {
		let filtered_meals_count = self.menu.count_filtered_meals();
		Ok(format!("{} | (excluding {filtered_meals_count} item{} less than {}€) | open: {daytime}{}",
				   self.mensa_name,
				   if filtered_meals_count > 1 { "s" } else { "" },
				   constants::get_lower_threshold_float(),
				   if filtered_meals_count == 0 { " | (presumed closed)" } else { "" }
		))
	}
}