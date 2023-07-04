use crate::api_interactions::format_todays_menu_url;
use crate::api_schema::{Menu};


pub fn request_menu(id: usize) -> Menu {
	let req = ureq::get(&format_todays_menu_url(id)).call().unwrap();

	req.into_json().unwrap()
}