use crate::api_schema::MenuItem;
use crate::constants;

#[cfg(feature = "async-reqwest")]
mod r#async;
#[cfg(feature = "sync-ureq")]
mod sync;

pub fn format_todays_menu_url(id: usize) -> String {
	format!("{}/v1/locations/{id}/menu/today", constants::BASE_DOMAIN)
}

pub fn fetch_menus<'a>() -> Vec<MenuItem<'a>> {
	#[cfg(feature = "async-reqwest")]
	{
		let rt = tokio::runtime::Runtime::new().unwrap();
		let mut threads = vec![];
		for i in constants::TO_FETCH {
			threads.push(rt.spawn(crate::api_interactions::r#async::request_menu(i.0)));
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
			.map(|i|(crate::api_interactions::sync::request_menu(i.0), i))
			.collect()
	}
}