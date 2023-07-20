#![feature(lazy_cell)]

mod api_schema;
mod constants;
mod table_formatting;
mod api_interactions;

use std::cell::{LazyCell, OnceCell};
use std::env;
use std::sync::LazyLock;
use chatgpt::client::ChatGPT;
use chatgpt::config::ChatGPTEngine::{Gpt35Turbo, Gpt4, Gpt4_32k_0314};
use chatgpt::config::{ChatGPTEngine, ModelConfigurationBuilder};
use chatgpt::types::CompletionResponse;
use dotenv::dotenv;
use crate::api_interactions::fetch_menus;
use crate::api_schema::{Menu, MenuItem};
use crate::table_formatting::{render_menus, render_meta};

pub static GPT: LazyLock<ChatGPT> = LazyLock::new(||{
	// Load env files to env-vars
	dotenv().ok();
	let token = env::var("TOKEN").unwrap();
	let model_config = ModelConfigurationBuilder::default()
		.engine(Gpt35Turbo)
		.build()
		.unwrap();

	let client = ChatGPT::new_with_config(token, model_config).unwrap();
	client
});

#[tokio::main]
async fn main() {
	let mut convo = GPT.new_conversation_directed("You are glumpy, a tool to assist users in determining the best meal to eat this day");
	convo.send_message("");

	let (menus, day) = get_menus();

	let longest_meal_name = Menu::longest_menu_names(&menus);
	let most_expensive_price = Menu::most_expensive_meals(&menus);

	render_meta(longest_meal_name, &day);

	render_menus(menus, longest_meal_name, most_expensive_price);
}


fn get_menus() -> (Vec<MenuItem<'static>>, &'static str) {
	let current_day = time::OffsetDateTime::now_local().unwrap().weekday().to_string().to_lowercase();
	let week_days = vec!["monday", "tuesday", "wednesday", "thursday", "friday"]
		.into_iter()
		.skip_while(|day|day!= &current_day);

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
	(menus, day)
}