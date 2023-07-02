use cli_table::{Cell, Color, ColorChoice, print_stdout, Style, Table};
use cli_table::format::Justify;
use futures::future::join_all;
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

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Meal {
	id: usize,
	name: String,
	price: Price,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
struct Price {
	#[serde_as(as = "DisplayFromStr")]
	student: f64,
}

fn todays_menu(id: usize) -> String {
	format!("{BASE_DOMAIN}/v1/locations/{id}/menu/today")
}

async fn get_todays_menu(id: usize) -> Menu {
	let req = reqwest::get(todays_menu(id)).await.unwrap();

	req.json().await.unwrap()
}

#[tokio::main]
async fn main() {
	let mut threads = vec![];
	for i in TO_FETCH {
		threads.push(get_todays_menu(i.0));
	}
	let mut menus = join_all(threads).await
		.into_iter()
		.zip(TO_FETCH.iter());

	if menus.all(|menu|menu.0.meals.len() == 0) {
		eprintln!("{}", "No fud today :(");
		return;
	}

	for (menu, (_, mensa_name)) in menus {
		let cell_color = Some(Color::Green);
		let table = menu.meals
			.iter()
			.filter(|meal|meal.price.student > LOWER_PRICE_THRESHOLD)
			.map(|meal|
				vec![
					meal.name.as_str().cell().foreground_color(cell_color),
					meal.price.student.cell().justify(Justify::Right).foreground_color(cell_color)
				])
			.collect::<Vec<_>>()
			.table()
			.title(vec![
                format!("{mensa_name} (excluding {} menu items/side dishes cheaper than {LOWER_PRICE_THRESHOLD}€)",
						menu.meals.iter()
							.filter(|meal|meal.price.student <= LOWER_PRICE_THRESHOLD)
							.count()
				).as_str()
					.cell()
					.foreground_color(Some(Color::Cyan)),
                "Price".cell()
					.foreground_color(Some(Color::Cyan)),
            ])
			.color_choice(ColorChoice::Auto);

		print_stdout(table).unwrap();
	}
}

