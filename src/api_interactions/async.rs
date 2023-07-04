use crate::api_interactions::format_todays_menu_url;
use crate::api_schema::{Menu};


pub async fn request_menu(id: usize, day: String) -> Menu {
	let req = reqwest::get(format_todays_menu_url(id, &day)).await.unwrap();

	req.json().await.unwrap()
}