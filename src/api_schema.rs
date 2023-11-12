use color_eyre::eyre::ContextCompat;
use color_eyre::Report;
use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use time::Weekday;
use crate::constants;
use crate::opening_hours::Location;

#[derive(Clone, Debug)]
pub struct MenuItem {
	pub menu: Menu,
	pub mensa_id: usize,
	pub mensa_name: String,
}

impl MenuItem {
	// Needs weekday to compute opening hours
	pub fn longest_menu_name(selfish: &[Self], weekday: Weekday) -> Result<usize, Report> {
		let res = selfish.iter()
			.map(|e| e.menu.longest_menu_name())
			.collect::<Result<Vec<usize>, Report>>()?
			.into_iter()
			.max()
			.context("Zero menu items")?;
		let title = selfish.iter()
			.next()
			.context("Empty menu item vector")?
			.format_title(weekday)?.len();
		Ok(res.max(title))
	}

	pub fn format_title(&self, weekday: Weekday) -> Result<String, Report> {
		let formatted_opening_hours = self.menu.meals.iter()
			.next()
			.map(|e| e.location.format_opening_hours(weekday))
			.context("Zero meals found for opening hours")?;

		let filtered_meals_count = self.menu.count_filtered_meals();
		Ok(format!("{} | (excluding {filtered_meals_count} item{} less than {}€) | open: {formatted_opening_hours}{}",
				   self.mensa_name,
				   if filtered_meals_count > 1 { "s" } else { "" },
				   constants::LOWER_PRICE_THRESHOLD,
				   if filtered_meals_count == 0 { " | (presumed closed)" } else { "" }
		))
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
	pub meals: Vec<Meal>,
}

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