use serde::{Deserialize, Serialize};
use serde_with::DisplayFromStr;
use serde_with::serde_as;
use crate::opening_hours::Location;

#[derive(Clone, Debug)]
pub struct MenuItem {
	pub menu: Menu,
	pub mensa_id: usize,
	pub mensa_name: String,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Menu {
	pub meals: Vec<Meal>,
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