mod api_schema;
mod constants;
mod table_formatting;
mod api_interactions;

use crate::api_interactions::fetch_menus;
use crate::api_schema::{Menu};
use crate::table_formatting::{render_menus, render_meta};

#[cfg(all(feature = "async-reqwest", feature = "sync-ureq"))]
compile_error!("Only either async-reqwest or sync-ureq may be enabled at once time");


fn main() {
	let fetch_order = vec!["today", "monday", "tuesday", "wednesday", "thursday", "friday"];

	// Fetch menus from today through all weekdays until a valid menu is found
	let (menus, day) = {
		let mut menu = None;
		for query_param in fetch_order {
			let menus = fetch_menus(query_param.to_owned());
			if Menu::count_meals(&menus) == 0 {
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

	render_meta(longest_meal_name, &day);

	render_menus(menus, longest_meal_name, most_expensive_price);
}
