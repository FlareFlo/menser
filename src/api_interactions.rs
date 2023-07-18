use crate::api_schema::MenuItem;
use crate::constants;
use crate::constants::TO_FETCH;
use crate::api_schema::{Menu};


pub fn format_todays_menu_url(id: usize, day: &str) -> String {
	format!("{}/v1/locations/{id}/menu/{}", constants::BASE_DOMAIN, day)
}

pub fn fetch_menus<'a>(day: String) -> Vec<MenuItem<'a>> {
	let mut threads = vec![];
	for i in TO_FETCH {
		let day = day.clone();
		threads.push(std::thread::spawn( move || {
			request_menu(i.0, &day)
		}));
	}
	let mut joined = vec![];
	for thread in threads {
		joined.push(thread.join().unwrap());
	}
	joined.into_iter().zip(TO_FETCH.iter()).collect()
}



pub fn request_menu(id: usize, day: &str) -> Menu {
	let req = ureq::get(&format_todays_menu_url(id, day)).call().unwrap();

	req.into_json().unwrap()
}