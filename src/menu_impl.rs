use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use crate::api_schema::{Menu, MenuItem};
use crate::constants;

impl Menu {
	pub fn longest_menu_name(&self) -> Result<usize, Report> {
		let res = self.meals
			.iter()
			.map(|e| e.name.len())
			.max().context("Found zero meals")?;
		Ok(res)
	}

	pub fn most_expensive_meal(&self) -> Result<f64, Report> {
		Ok(self.meals
			.iter()
			.max_by(|lhs, rhs|
				lhs.price.student.total_cmp(&rhs.price.student)
			).context("Found zero meals")?
			.price.student)
	}
	pub fn most_expensive_meals(menus: &[MenuItem]) -> Result<f64, Report> {
		let res = menus.iter()
			.map(|e| e.menu.most_expensive_meal())
			.collect::<Result<Vec<f64>, Report>>()?
			.into_iter()
			.max_by(|l, r| l.total_cmp(r))
			.context("Zero menu items")?;
		Ok(res)
	}

	pub fn count_meals<'a>(menus: impl Iterator<Item=&'a MenuItem>) -> usize {
		menus.map(|menu| menu.menu.meals.len()).sum()
	}

	pub fn count_filtered_meals(&self) -> usize {
		self.meals.iter()
			.filter(|meal| meal.price.student <= constants::LOWER_PRICE_THRESHOLD && meal.price.student != 0.0)
			.count()
	}
}