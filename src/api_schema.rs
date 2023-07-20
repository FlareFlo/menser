use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;

pub type MenuItem<'a> = (Menu, &'a (usize, &'a str));

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
	pub meals: Vec<Meal>,
}

impl Menu {
	pub fn longest_menu_name(&self) -> usize {
		self.meals
			.iter()
			.max_by(|lhs, rhs|
				lhs.name_en.len().cmp(&rhs.name_en.len())
			).unwrap()
			.name_en
			.len()
	}
	pub fn longest_menu_names(menus: &[MenuItem]) -> usize {
		menus.iter().max_by(|lhs, rhs| {
			lhs.0.longest_menu_name().cmp(&rhs.0.longest_menu_name())
		}).unwrap().0.longest_menu_name()
	}

	pub fn most_expensive_meal(&self) -> f64 {
		self.meals
			.iter()
			.max_by(|lhs, rhs|
				lhs.price.student.total_cmp(&rhs.price.student)
			).unwrap()
			.price.student
	}
	pub fn most_expensive_meals(menus: &[MenuItem]) -> f64 {
		menus.iter().max_by(|lhs, rhs| {
			lhs.0.most_expensive_meal().total_cmp(&rhs.0.most_expensive_meal())
		}).unwrap().0.most_expensive_meal()
	}

	pub fn count_meals<'a>(menus: impl Iterator<Item=&'a MenuItem<'a>>) -> usize {
		menus.map(|menu| menu.0.meals.len()).sum()
	}

	pub fn format_gpt_readable<'a>(menus: &[MenuItem]) -> String {
		menus.iter().map(|menu| format!("Menu from cafeteria {}: {}", menu.1.1, menu.0.meals.iter().map(|e| e.format_gpt_readable()).collect::<Vec<_>>().join(", ")))
			.collect::<Vec<String>>()
			.join("\n")
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meal {
	pub id: usize,
	pub name_en: String,
	pub price: Price,
	pub tags: Tags,
}

impl Meal {
	pub fn is_lower_saxony_menu(&self) -> bool {
		self.tags.categories.contains(&Category { name: "Niedersachsen Menü".to_string() })
	}
	pub fn format_gpt_readable(&self) -> String {
		format!("{} for {}€ and the tags {}", self.name_en, self.price.student, self.tags.format_gpt_readable())
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

impl Tags {
	pub fn format_gpt_readable(&self) -> String {
		self.categories.iter().map(|category| category.name.clone()).collect::<Vec<_>>().join(" ")
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Category {
	pub name: String,
}