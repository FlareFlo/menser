use crate::api_interactions::format_todays_menu_url;
use crate::api_schema::{Menu, MenuItem};
use crate::constants;


pub fn fetch_menus<'a>() -> Vec<MenuItem<'a>> {
	#[cfg(feature = "async-reqwest")]
	{
		let rt = tokio::runtime::Runtime::new().unwrap();
		let mut threads = vec![];
		for i in constants::TO_FETCH {
			threads.push(rt.spawn(request_menu(i.0)));
		}
		let menus = rt.block_on(futures::future::join_all(threads))
			.into_iter()
			.map(|future|future.unwrap())
			.zip(constants::TO_FETCH.iter());
		rt.shutdown_timeout(std::time::Duration::from_secs(10));
		menus.collect()
	}
	#[cfg(feature = "sync-ureq")]
	{
		constants::TO_FETCH.iter()
			.map(|i|(request_menu(i.0), i))
			.collect()
	}
}

#[cfg(feature = "async-reqwest")]
pub async fn request_menu(id: usize) -> Menu {
	let req = reqwest::get(format_todays_menu_url(id)).await.unwrap();

	req.json().await.unwrap()
}

#[cfg(feature = "sync-ureq")]
pub fn request_menu(id: usize) -> Menu {
	let req = ureq::get(&format_todays_menu_url(id)).call().unwrap();

	req.into_json().unwrap()
}