
#[cfg(feature = "async-reqwest")]
use crate::api_interactions::format_todays_menu_url;
#[cfg(feature = "async-reqwest")]
use crate::api_schema::{Menu};


#[cfg(feature = "async-reqwest")]
pub async fn request_menu(id: usize) -> Menu {
	let req = reqwest::get(format_todays_menu_url(id)).await.unwrap();

	req.json().await.unwrap()
}