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
	let menus = fetch_menus();

	let longest_meal_name = Menu::longest_menu_names(&menus);
	let most_expensive_price = Menu::most_expensive_meals(&menus);

	render_meta(longest_meal_name);

	render_menus(menus, longest_meal_name, most_expensive_price);
}
