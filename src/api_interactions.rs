use color_eyre::Report;

use crate::api_schema::Menu;
use crate::api_schema::MenuItem;
use crate::constants;
use crate::constants::TO_FETCH;
use crate::rest_api_impl::request_menu;

pub fn format_todays_menu_url(id: usize, day: &str) -> String {
	format!("{}/v1/locations/{id}/menu/{}?time=all", constants::BASE_DOMAIN, day)
}

pub fn fetch_menus<'a>(day: &str) -> Result<Vec<MenuItem>, Report> {
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
		joined.push(thread.join().map_err(|e|Report::msg(format!("Failed to join thread: {:?}", e)))??);
	}
	Ok(joined.into_iter()
		.zip(TO_FETCH.iter())
		.map(|e|MenuItem {
			menu: e.0,
			mensa_id: e.1.0,
			mensa_name: e.1.1.to_owned(),
		})
		.collect())
}