use crate::api_schema::MenuItem;
use crate::constants;
use crate::constants::TO_FETCH;

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
		for i in TO_FETCH {
			threads.push(rt.spawn(crate::api_interactions::r#async::request_menu(i.0)));
		}
		let menus = rt.block_on(futures::future::join_all(threads))
			.into_iter()
			.map(|future|future.unwrap())
			.zip(TO_FETCH.iter());
		rt.shutdown_timeout(std::time::Duration::from_secs(10));
		menus.collect()
	}
	#[cfg(feature = "sync-ureq")]
	{
		let mut threads = vec![];
		for i in TO_FETCH {
			threads.push(std::thread::spawn(|| {
				sync::request_menu(i.0)
			}));
		}
		let mut joined = vec![];
		for thread in threads {
			joined.push(thread.join().unwrap());
		}
		joined.into_iter().zip(TO_FETCH.iter()).collect()
	}
}