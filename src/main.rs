#![feature(lazy_cell)]

use crate::api_interactions::get_menus;
use crate::gpt_interaction::ask_gpt;

mod api_schema;
mod constants;
mod table_formatting;
mod api_interactions;
mod gpt_interaction;

#[tokio::main]
async fn main() {
	let (menus, _) = get_menus();
	ask_gpt(menus).await;
}