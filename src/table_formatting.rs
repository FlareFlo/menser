use std::collections::HashMap;
use cli_table::{Cell, Color, print_stdout, Style, Table};
use cli_table::format::Justify;
use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use pad::PadStr;
use time::Weekday;

use crate::{COLOR, constants};
use crate::api_schema::{Meal, MensaMenu};
use crate::constants::{colors, compute_price_color};
use crate::opening_hours::OpeningHours;

pub fn render_meta(longest_meal_name: usize, day: &str) -> Result<(), Report> {
	let meta = vec![vec![day.cell(), "".cell()]]
		.table()
		.title(vec![
			"Fetched from".pad_to_width(longest_meal_name).cell().foreground_color(Some(colors::TITLE)),
			"".pad_to_width(7).cell(),
		])
		.color_choice(*COLOR.get().context("COLOR was unset")?);
	Ok(print_stdout(meta)?)
}

pub fn render_menus(
	menus: impl IntoIterator<Item=MensaMenu>,
	longest_meal_name: usize,
	most_expensive_price: f64,
	weekday: Weekday
) -> Result<(), Report> {
	for menu_item in menus {
		let meal_opening_hours = menu_item.menu.meals
			.iter()
			.map(|meal|(meal.todays_opening_hours(weekday), meal));
		let mut grouped_by_opening_hours: HashMap<&OpeningHours, Vec<&Meal>> = HashMap::new();

		for (hours, meal) in meal_opening_hours {
			for open_for in hours {
				grouped_by_opening_hours.entry(open_for)
					.and_modify(|e|e.push(meal))
					.or_insert(vec![meal]);
			}
		}

		for (opening_hours, meals) in grouped_by_opening_hours.into_iter() {
			let fmt_meals = |meal: &&Meal| {
				vec![
					emojify_name(meal.name.clone()).pad_to_width(longest_meal_name).as_str().cell().foreground_color(if meal.is_lower_saxony_menu() {
						Some(colors::LOWER_SAXONY)
					} else {
						Some(compute_cell_color_from_name(meal.name.as_str()))
					}),
					(meal.price.student as f64 / 100.0).cell().justify(Justify::Right).foreground_color(Some(compute_price_color(meal.price.student, most_expensive_price))),
				]
			};

			let title = menu_item.format_title(opening_hours)?;

			let title = vec![
				title.as_str()
					.pad_to_width(longest_meal_name)
					.cell()
					.foreground_color(Some(colors::TITLE)),
				"Price €".cell()
					.foreground_color(Some(colors::TITLE)),
			];

			let table = meals
				.iter()
				.filter(|meal| meal.price.student > constants::get_lower_threshold_int())
				.map(fmt_meals)
				.collect::<Vec<_>>()
				.table()
				.title(title)
				.color_choice(*COLOR.get().context("COLOR was unset")?);

			print_stdout(table)?;
		}
	}
	Ok(())
}

fn compute_cell_color_from_name(name: &str) -> Color {
	let lc = name.to_lowercase();

	if lc.contains("pizza") && !lc.contains("burger") { return colors::PIZZA; }


	colors::DEFAULT_MEAL
}

fn emojify_name(mut name: String) -> String {
	let lc = name.to_lowercase();

	if lc.contains("pizza") && !lc.contains("burger") { name.push('🍕') }

	name
}