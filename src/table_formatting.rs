use cli_table::{Cell, Color, print_stdout, Style, Table};
use cli_table::Color::Rgb;
use cli_table::format::{Justify};
use pad::PadStr;
use time::Weekday;
use crate::api_schema::MenuItem;
use crate::{COLOR, constants};

pub fn render_meta(longest_meal_name: usize, day: &str) {
	let meta = vec![vec![day.cell(), "".cell()]]
		.table()
		.title(vec![
			"Fetched from".pad_to_width(longest_meal_name).cell().foreground_color(Some(Color::Cyan)),
			"".pad_to_width(7).cell(),
		])
		.color_choice(*COLOR.get().unwrap());
	print_stdout(meta).unwrap();
}

pub fn render_menus<'a>(menus: impl IntoIterator<Item=MenuItem<'a>>, longest_meal_name: usize, most_expensive_price: f64, weekday: Weekday) {
	let compute_price_color = |price: f64| {
		let lerp_color = |x: f64| (1.1 * x + 33.0).round() as u8;
		let lerp_price = |x: f64| (x - constants::LOWER_PRICE_THRESHOLD) / (most_expensive_price - constants::LOWER_PRICE_THRESHOLD) * 100.0;
		Rgb(lerp_color(lerp_price(price)), lerp_color(100.0 - lerp_price(price)), 33)
	};

	for (menu, (_, mensa_name)) in menus {
		let formatted_opening_hours = menu.meals.iter().next().map(|e| e.location.format_opening_hours(weekday)).unwrap_or(String::new());
		let filtered_meals_count = menu.count_filtered_meals();

		let table = menu.meals
			.iter()
			.filter(|meal| meal.price.student > constants::LOWER_PRICE_THRESHOLD)
			.map(|meal|
				vec![
					emojify_name(meal.name.clone()).pad_to_width(longest_meal_name).as_str().cell().foreground_color(if meal.is_lower_saxony_menu() {
						Some(Rgb(255, 233, 0))
					} else {
						Some(compute_cell_color_from_name(meal.name.as_str()))
					}),
					meal.price.student.cell().justify(Justify::Right).foreground_color(Some(compute_price_color(meal.price.student))),
				])
			.collect::<Vec<_>>()
			.table()
			.title(vec![
				format!("{mensa_name} | (excluding {filtered_meals_count} item{} less than {}€) | open: {formatted_opening_hours}{}",
						if filtered_meals_count > 1 { "s" } else { "" },
						constants::LOWER_PRICE_THRESHOLD,
						if filtered_meals_count == 0 { " | (presumed closed)" } else { "" }
				).as_str()
					.pad_to_width(longest_meal_name)
					.cell()
					.foreground_color(Some(Color::Cyan)),
				"Price €".cell()
					.foreground_color(Some(Color::Cyan)),
			])
			.color_choice(*COLOR.get().unwrap());

		print_stdout(table).unwrap();
	}
}

fn compute_cell_color_from_name(name: &str) -> Color {
	let lc = name.to_lowercase();

	if lc.contains("pizza") && !lc.contains("burger") { return Color::Magenta }



	Color::Green
}

fn emojify_name(mut name: String) -> String {
	let lc = name.to_lowercase();

	if lc.contains("pizza") && !lc.contains("burger") { name.push('🍕') }

	name
}