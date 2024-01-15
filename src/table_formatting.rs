use cli_table::{Cell, Color, print_stdout, Style, Table};
use cli_table::format::Justify;
use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use pad::PadStr;
use time::Weekday;

use crate::{COLOR, constants};
use crate::api_schema::{Meal, MensaMenu};
use crate::constants::{colors, compute_price_color};

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
		let meals = |meal: &Meal|
			vec![
				emojify_name(meal.name.clone()).pad_to_width(longest_meal_name).as_str().cell().foreground_color(if meal.is_lower_saxony_menu() {
					Some(colors::LOWER_SAXONY)
				} else {
					Some(compute_cell_color_from_name(meal.name.as_str()))
				}),
				(meal.price.student as f64 / 100.0).cell().justify(Justify::Right).foreground_color(Some(compute_price_color(meal.price.student, most_expensive_price))),
			];

		let title = menu_item.format_title(weekday)?;

		let title = vec![
			title.as_str()
				.pad_to_width(longest_meal_name)
				.cell()
				.foreground_color(Some(colors::TITLE)),
			"Price €".cell()
				.foreground_color(Some(colors::TITLE)),
		];

		let table = menu_item.menu.meals
			.iter()
			.filter(|meal| meal.price.student > constants::get_lower_threshold_int())
			.map(meals)
			.collect::<Vec<_>>()
			.table()
			.title(title)
			.color_choice(*COLOR.get().context("COLOR was unset")?);

		print_stdout(table)?;
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