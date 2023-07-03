use futures::future::join_all;
use crate::api_schema::{Menu, MenuItem};
use crate::constants;

fn format_todays_menu_url(id: usize) -> String {
	format!("{}/v1/locations/{id}/menu/today", constants::BASE_DOMAIN)
}

pub async fn fetch_menus<'a>() -> Vec<MenuItem<'a>> {
	let mut threads = vec![];
	for i in constants::TO_FETCH {
		threads.push(request_menu(i.0));
	}
	let menus = join_all(threads).await
		.into_iter()
		.zip(constants::TO_FETCH.iter());
	menus.collect()
}

pub async fn request_menu(id: usize) -> Menu {
	let req = reqwest::get(format_todays_menu_url(id)).await.unwrap();

	req.json().await.unwrap()
}