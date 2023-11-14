#[cfg(feature = "ureq")]
pub use crate::rest_api_impl::ureq::request_menu;

#[cfg(feature = "ureq")]
mod ureq {
	use color_eyre::Report;
	use crate::api_interactions::format_todays_menu_url;
	use crate::api_schema::Menu;

	pub fn request_menu(id: usize, day: String) -> Result<Menu, Report> {
		let req = ureq::get(&format_todays_menu_url(id, &day)).call()?;

		Ok(req.into_json()?)
	}
}

