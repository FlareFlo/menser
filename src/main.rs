mod api_schema;
mod constants;
mod table_formatting;
mod api_interactions;
mod opening_hours;

static COLOR: OnceLock<ColorChoice> = OnceLock::new();

use std::env::{args};
use std::process::exit;
use std::sync::OnceLock;
use cli_table::ColorChoice;
use time::Weekday;
use crate::api_interactions::fetch_menus;
use crate::api_schema::{Menu};
use crate::table_formatting::{render_menus, render_meta};

fn main() {
	color_eyre::install().unwrap();
	let days = vec!["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
	let today = time::OffsetDateTime::now_local().unwrap().weekday();

	let current_day = match args().nth(1) {
		Some(value) => {
			let value = value.to_lowercase();
			if value == "tomorrow" {
				today.next().to_string().to_lowercase()
			} else {
				value
			}
		}
		None => { today.to_string().to_lowercase() }
	};

	if !days.contains(&current_day.as_str()) {
		eprintln!("Unrecognized day/option: {}", args().nth(1).unwrap_or("".to_owned()));
		exit(1);
	}

	let week_days = days
		.into_iter()
		.cycle()
		.skip_while(|day| day != &current_day)
		.take(7)
		.collect::<Vec<_>>();

	// Fetch menus from today through all weekdays until a valid menu is found
	let (menus, day) = {
		let mut menu = None;
		for query_param in week_days {
			let menus = fetch_menus(query_param).unwrap();
			if Menu::count_meals(menus.iter()) == 0 {
				eprintln!("No food for {query_param}, picking next possible date");
				continue;
			}
			menu = Some((menus, query_param));
			break;
		}
		menu
	}.expect("No menus in weekdays at all");

	let longest_meal_name = Menu::longest_menu_names(&menus);
	let most_expensive_price = Menu::most_expensive_meals(&menus);

	COLOR.get_or_init(|| {
		if std::env::var("NO_COLOR").is_ok() {
			ColorChoice::Never
		} else {
			ColorChoice::Auto
		}
	});

	render_meta(longest_meal_name, &day);

	render_menus(menus, longest_meal_name, most_expensive_price, weekday_from_str(day));
}

fn weekday_from_str(input: &str) -> Weekday {
	match input {
		"monday" => Weekday::Monday,
		"tuesday" => Weekday::Tuesday,
		"wednesday" => Weekday::Wednesday,
		"thursday" => Weekday::Thursday,
		"friday" => Weekday::Friday,
		"saturday" => Weekday::Saturday,
		"sunday" => Weekday::Sunday,
		_ => { panic!("Unrecognized weekday: {input}") }
	}
}