use std::env;

#[derive(Default)]
pub struct Args {
	pub tomorrow: bool,
	pub sides: bool,
}

pub fn argparse() -> color_eyre::Result<Args> {
	let mut args = Args::default();
	let raw_args = env::args();

	for raw in raw_args {
		match raw.to_lowercase().as_str() {
			"tomorrow" => args.tomorrow = true,
			"sides" => args.sides= true,
			_ => {}
		}
	}

	Ok(args)
}