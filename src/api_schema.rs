use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MensaMenu {
    pub menu: Menu,
    pub mensa_id: usize,
    pub mensa_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Menu {
    pub meals: Vec<Meal>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Meal {
    pub id: usize,
    pub name: String,
    pub price: Price,
    pub tags: Tags,
    pub time: String,
}

impl Meal {
    pub fn is_lower_saxony_menu(&self) -> bool {
        self.tags.categories.contains(&Category {
            name: "Niedersachsen Menü".to_string(),
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Price {
    #[serde(deserialize_with = "price_deserialize")]
    pub student: u16,
}

fn price_deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u16, D::Error> {
    let string = String::deserialize(deserializer)?;
    Ok((f64::from_str(&string).map_err(|e| serde::de::Error::custom(e))? * 100.0) as u16)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Tags {
    pub categories: Vec<Category>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Hash)]
pub struct Category {
    pub name: String,
}
