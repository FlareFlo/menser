use crate::constants;

mod a_sync;
mod sync;

pub fn format_todays_menu_url(id: usize) -> String {
	format!("{}/v1/locations/{id}/menu/today", constants::BASE_DOMAIN)
}