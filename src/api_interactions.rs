use crate::api_schema::Menu;
use crate::api_schema::MenuItem;
use crate::constants;
use crate::constants::TO_FETCH;

pub fn get_menus() -> (Vec<MenuItem<'static>>, &'static str) {
	// let current_day = time::OffsetDateTime::now_local().unwrap().weekday().to_string().to_lowercase();
	let current_day = "thursday".to_owned();
	let week_days = vec!["monday", "tuesday", "wednesday", "thursday", "friday"]
		.into_iter()
		.skip_while(|day| day != &current_day);

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

pub fn format_todays_menu_url(id: usize, day: &str) -> String {
	format!("{}/v1/locations/{id}/menu/{}", constants::BASE_DOMAIN, day)
}

pub fn fetch_menus<'a>(day: &str) -> Option<Vec<MenuItem<'a>>> {
	let mut threads = vec![];
	for i in TO_FETCH {
		// We own the memory for day here, to safely pass it to the threads
		let day = day.to_owned();
		threads.push(std::thread::spawn(move || {
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