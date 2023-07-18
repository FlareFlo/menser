use crate::api_schema::MenuItem;
use crate::constants;
use crate::constants::TO_FETCH;
use crate::api_schema::{Menu};


pub fn format_todays_menu_url(id: usize, day: &str) -> String {
	format!("{}/v1/locations/{id}/menu/{}", constants::BASE_DOMAIN, day)
}

pub fn fetch_menus<'a>(day: &str) -> Option<Vec<MenuItem<'a>>> {
	let mut threads = vec![];
	for i in TO_FETCH {
		// We own the memory for day here, to safely pass it to the threads
		let day = day.to_owned();
		threads.push(std::thread::spawn( move || {
			request_menu(i.0, day)
		}));
	}
	let mut joined: Vec<Menu> = vec![];
	for thread in threads {
		joined.push(thread.join().ok()??);
	}
	Some(joined.into_iter().zip(TO_FETCH.iter()).collect())
}



pub fn request_menu(id: usize, day: String) -> Option<Menu> {
	let req = ureq::get(&format_todays_menu_url(id, &day)).call().ok()?;

	req.into_json().ok()?
}