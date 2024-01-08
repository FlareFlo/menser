#[cfg(feature = "curl")]
pub use crate::rest_api_impl::curl::request_menu;
#[cfg(feature = "ureq")]
pub use crate::rest_api_impl::ureq::request_menu;

#[cfg(feature = "ureq")]
#[cfg(feature = "curl")]
compile_error!("Curl and ureq cannot be enabled together");


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

#[cfg(feature = "curl")]
mod curl {
	use std::process::Command;

	use color_eyre::Report;

	use crate::api_interactions::format_todays_menu_url;
	use crate::api_schema::Menu;

	pub fn request_menu(id: usize, day: String) -> Result<Menu, Report> {
		let url = format_todays_menu_url(id, &day);
		let output = Command::new("curl")
			.args(&[
				"--silent",
				&url,
			]).output()?;
		let stdout = String::from_utf8(output.stdout)?;

		Ok(serde_json::from_str(&stdout)?)
	}
}
