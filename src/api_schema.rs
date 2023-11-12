use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use crate::constants;
use crate::opening_hours::Location;

pub type MenuItem<'a> = (Menu, &'a (usize, &'a str));

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
	pub meals: Vec<Meal>,
}

impl Menu {
	pub fn longest_menu_name(&self) -> Result<usize, Report> {
		let res = self.meals
			.iter()
			.map(|e|e.name.len())
			.max().context("Found zero meals")?;
		Ok(res)
	}
	pub fn longest_menu_names(menus: &[MenuItem]) -> Result<usize, Report> {
		let res = menus.iter()
			.map(|e|e.0.longest_menu_name())
			.collect::<Result<Vec<usize>, Report>>()?
			.into_iter()
			.max()
			.context("Zero menu items")?;
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
			.map(|e|e.0.most_expensive_meal())
			.collect::<Result<Vec<f64>, Report>>()?
			.into_iter()
			.max_by(|l, r|l.total_cmp(r))
			.context("Zero menu items")?;
		Ok(res)
	}

	pub fn count_meals<'a>(menus: impl Iterator<Item=&'a MenuItem<'a>>) -> usize {
		menus.map(|menu|menu.0.meals.len()).sum()
	}

	pub fn count_filtered_meals(&self) -> usize {
		self.meals.iter()
			.filter(|meal| meal.price.student <= constants::LOWER_PRICE_THRESHOLD && meal.price.student != 0.0)
			.count()
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meal {
	pub id: usize,
	pub name: String,
	pub price: Price,
	pub tags: Tags,
	pub location: Location,
}

impl Meal {
	pub fn is_lower_saxony_menu(&self) -> bool {
		self.tags.categories.contains(&Category { name: "Niedersachsen Menü".to_string() })
	}
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Price {
	#[serde_as(as = "DisplayFromStr")]
	pub student: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tags {
	pub categories: Vec<Category>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
	pub name: String,
}