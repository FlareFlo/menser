use cli_table::{Cell, Color, ColorChoice, print_stdout, Style, Table};
use cli_table::Color::Rgb;
use cli_table::format::Justify;
use futures::future::join_all;
use pad::PadStr;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;

const MENSA_1: (usize, &str) = (101, "Mensa 1");
const MENSA_2: (usize, &str) = (105, "Mensa 2");
const MENSA_360: (usize, &str) = (111, "360°");

const TO_FETCH: &[(usize, &str)] = &[MENSA_1, MENSA_2, MENSA_360];

const LOWER_PRICE_THRESHOLD: f64 = 2.0; // Do not display things cheaper than this

const BASE_DOMAIN: &str = "https://sls.api.stw-on.de";

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Menu {
	meals: Vec<Meal>,
}

impl Menu {
	pub fn longest_menu_name(&self) -> usize {
		self.meals
			.iter()
			.max_by(|lhs, rhs|
				lhs.name.len().cmp(&rhs.name.len())
			).unwrap()
			.name
			.len()
	}
	pub fn longest_menu_names(menus: &[MenuItem]) -> usize {
		menus.iter().max_by(|lhs, rhs| {
			lhs.0.longest_menu_name().cmp(&rhs.0.longest_menu_name())
		}).unwrap().0.longest_menu_name()
	}

	pub fn most_expensive_meal(&self) -> f64 {
		self.meals
			.iter()
			.max_by(|lhs, rhs|
				lhs.price.student.total_cmp(&rhs.price.student)
			).unwrap()
			.price.student
	}
	pub fn most_expensive_meals(menus: &[MenuItem]) -> f64 {
		menus.iter().max_by(|lhs, rhs| {
			lhs.0.most_expensive_meal().total_cmp(&rhs.0.most_expensive_meal())
		}).unwrap().0.most_expensive_meal()
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Meal {
	id: usize,
	name: String,
	price: Price,
	tags: Tags,
}

impl Meal {
	pub fn is_lower_saxony_menu(&self) -> bool {
		self.tags.categories.contains(&Category { name: "Niedersachsen Menü".to_string() })
	}
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Price {
	#[serde_as(as = "DisplayFromStr")]
	student: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Tags {
	categories: Vec<Category>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Category {
	name: String,
}

fn format_todays_menu_url(id: usize) -> String {
	format!("{BASE_DOMAIN}/v1/locations/{id}/menu/today")
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

type MenuItem<'a> = (Menu, &'a (usize, &'a str));

async fn fetch_menus<'a>() -> Vec<MenuItem<'a>> {
	let mut threads = vec![];
	for i in TO_FETCH {
		threads.push(request_menu(i.0));
	}
	let menus = join_all(threads).await
		.into_iter()
		.zip(TO_FETCH.into_iter());
	menus.collect()
}


fn render_menus<'a>(menus: impl IntoIterator<Item=MenuItem<'a>>, longest_meal_name: usize, most_expensive_price: f64) {
	let compute_price_color = |price: f64| {
		let lerp_color = |x: f64|(1.1 * x + 33.0).round() as u8;
		let lerp_price = |x: f64|(x - LOWER_PRICE_THRESHOLD)  /  (most_expensive_price - LOWER_PRICE_THRESHOLD) * 100.0;
		Rgb( lerp_color(lerp_price(price)), lerp_color(100.0 - lerp_price(price)), 33)
	};

	for (menu, (_, mensa_name)) in menus {
		let cell_color = Some(Color::Green);
		let table = menu.meals
			.iter()
			.filter(|meal| meal.price.student > LOWER_PRICE_THRESHOLD)
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
				format!("{mensa_name} (excluding {} menu items/side dishes cheaper than {LOWER_PRICE_THRESHOLD}€)",
						menu.meals.iter()
							.filter(|meal| meal.price.student <= LOWER_PRICE_THRESHOLD)
							.count()
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
