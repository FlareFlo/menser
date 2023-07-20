use std::{env, fs};
use std::sync::LazyLock;

use chatgpt::client::ChatGPT;
use chatgpt::config::ModelConfigurationBuilder;
use chatgpt::config::ChatGPTEngine::Gpt35Turbo;
use dotenv::dotenv;
use time::Instant;

use crate::api_schema::{Menu, MenuItem};

pub static GPT: LazyLock<ChatGPT> = LazyLock::new(|| {
	// Load env files to env-vars
	dotenv().ok();
	let token = env::var("TOKEN").unwrap();
	let model_config = ModelConfigurationBuilder::default()
		.engine(Gpt35Turbo)
		.temperature(0.1)
		.build()
		.unwrap();

	let client = ChatGPT::new_with_config(token, model_config).unwrap();
	client
});

pub async fn ask_gpt<'a>(menus: Vec<MenuItem<'a>>) {
	let personal_preferences = fs::read_to_string("food-preferences.txt").unwrap();

	let mut convo = GPT.new_conversation_directed(include_str!("../gpt-config/conversation-direction.txt"));
	let query = format!("Following meals are available today: {}. i would like to eat something that fits my personal-preferences {personal_preferences}", Menu::format_gpt_readable(&menus));

	let start = Instant::now();
	let res = convo.send_message(&query).await.unwrap();
	println!("{}", &res.message().content);
	let input_cost = 0.03;
	let output_cost = 0.06;
	let in_cost = res.usage.prompt_tokens as f64 / 1000.0 * input_cost;
	let out_cost = res.usage.completion_tokens as f64 / 1000.0 * output_cost;
	let est_cost = in_cost + out_cost;
	println!();
	println!("Gpt-4 took: {:.2} seconds", start.elapsed().as_seconds_f64());
	println!("Estimated API cost: {:.6}$", est_cost);
	println!("Using one dollar, you could make {:.1} requests", 1.0 / est_cost);
}