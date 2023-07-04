mod api_schema;
mod constants;
mod table_formatting;
mod api_interactions;

use crate::api_interactions::fetch_menus;
use crate::api_schema::{Menu};
use crate::constants::WEEKDAYS;
use crate::table_formatting::{render_menus, render_meta};

#[cfg(all(feature = "async-reqwest", feature = "sync-ureq"))]
compile_error!("Only either async-reqwest or sync-ureq may be enabled at once time");


fn main() {
	let mut day = "today";
	let mut menus = fetch_menus(day);
	if Menu::count_meals(&menus) == 0 {
		for weekday in WEEKDAYS {
			let next_day_menus = fetch_menus(day);
			if Menu::count_meals(&next_day_menus) != 0 {
				menus = next_day_menus;
				day = weekday;
				break;
			}
		}
	}

	let longest_meal_name = Menu::longest_menu_names(&menus);
	let most_expensive_price = Menu::most_expensive_meals(&menus);

	render_meta(longest_meal_name, day);

	render_menus(menus, longest_meal_name, most_expensive_price);
}
