mod api_schema;
mod constants;
mod api_interaction;
mod table_formatting;

use crate::api_interaction::{fetch_menus};
use crate::api_schema::{Menu};
use crate::table_formatting::{render_menus, render_meta};


#[tokio::main]
async fn main() {
	let menus = fetch_menus().await;

	let longest_meal_name = Menu::longest_menu_names(&menus);
	let most_expensive_price = Menu::most_expensive_meals(&menus);

	render_meta(longest_meal_name);

	render_menus(menus, longest_meal_name, most_expensive_price);
}
