use crate::api_interactions::format_todays_menu_url;
use crate::api_schema::{Menu};


pub fn request_menu(id: usize, day: Option<&str>) -> Menu {
	let req = ureq::get(&format_todays_menu_url(id, day)).call().unwrap();

	req.into_json().unwrap()
}