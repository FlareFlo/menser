mod api_shema;
mod constants;

use cli_table::{Cell, Color, ColorChoice, print_stdout, Style, Table};
use cli_table::Color::Rgb;
use cli_table::format::Justify;
use futures::future::join_all;
use pad::PadStr;
use crate::api_shema::{Menu, MenuItem};


fn format_todays_menu_url(id: usize) -> String {
	format!("{}/v1/locations/{id}/menu/today", constants::BASE_DOMAIN)
}

async fn request_menu(id: usize) -> Menu {
	let req = reqwest::get(format_todays_menu_url(id)).await.unwrap();

	req.json().await.unwrap()
}

#[tokio::main]
async fn main() {
	let menus = fetch_menus().await;

	let longest_meal_name = Menu::longest_menu_names(&menus);
	let most_expensive_price = Menu::most_expensive_meals(&menus);

	let meta = vec![vec!["today".cell(), "".cell()]]
		.table()
		.title(vec![
			"Fetched from".pad_to_width(longest_meal_name).cell().foreground_color(Some(Color::Cyan)),
			"".pad_to_width(7).cell(),
		]);
	print_stdout(meta).unwrap();

	render_menus(menus, longest_meal_name, most_expensive_price);
}


async fn fetch_menus<'a>() -> Vec<MenuItem<'a>> {
	let mut threads = vec![];
	for i in constants::TO_FETCH {
		threads.push(request_menu(i.0));
	}
	let menus = join_all(threads).await
		.into_iter()
		.zip(constants::TO_FETCH.into_iter());
	menus.collect()
}


fn render_menus<'a>(menus: impl IntoIterator<Item=MenuItem<'a>>, longest_meal_name: usize, most_expensive_price: f64) {
	let compute_price_color = |price: f64| {
		let lerp_color = |x: f64|(1.1 * x + 33.0).round() as u8;
		let lerp_price = |x: f64|(x - constants::LOWER_PRICE_THRESHOLD)  /  (most_expensive_price - constants::LOWER_PRICE_THRESHOLD) * 100.0;
		Rgb( lerp_color(lerp_price(price)), lerp_color(100.0 - lerp_price(price)), 33)
	};

	for (menu, (_, mensa_name)) in menus {
		let cell_color = Some(Color::Green);
		let table = menu.meals
			.iter()
			.filter(|meal| meal.price.student > constants::LOWER_PRICE_THRESHOLD)
			.map(|meal|
				vec![
					meal.name.pad_to_width(longest_meal_name).as_str().cell().foreground_color(if meal.is_lower_saxony_menu() {
						Some(Rgb(255,233,0))
					} else {
						cell_color
					}),
					meal.price.student.cell().justify(Justify::Right).foreground_color(Some(compute_price_color(meal.price.student))),
				])
			.collect::<Vec<_>>()
			.table()
			.title(vec![
				format!("{mensa_name} (excluding {} menu items/side dishes cheaper than {}€)",
						menu.meals.iter()
							.filter(|meal| meal.price.student <= constants::LOWER_PRICE_THRESHOLD)
							.count(),
						constants::LOWER_PRICE_THRESHOLD,
				).as_str()
					.cell()
					.foreground_color(Some(Color::Cyan)),
				"Price €".cell()
					.foreground_color(Some(Color::Cyan)),
			])
			.color_choice(ColorChoice::Auto);

		print_stdout(table).unwrap();
	}
}
