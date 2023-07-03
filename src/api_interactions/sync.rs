#[cfg(feature = "sync-ureq")]
use crate::api_interactions::format_todays_menu_url;
#[cfg(feature = "sync-ureq")]
use crate::api_schema::{Menu};


#[cfg(feature = "sync-ureq")]
pub fn request_menu(id: usize) -> Menu {
	let req = ureq::get(&format_todays_menu_url(id)).call().unwrap();

	req.into_json().unwrap()
}