use crate::localization::*;
use std::env;
use std::str::FromStr;
use std::sync::OnceLock;

use cli_table::ColorChoice;
use color_eyre::eyre::{bail, ContextCompat};
use color_eyre::Report;

use crate::api_interactions::fetch_menus;
use crate::api_schema::{MensaMenu, Menu};
use crate::constants::set_lower_threshold_int;
use crate::simple_argparse::argparse;
use crate::table_formatting::{render_menus, render_meta};

/// Structs serialized from JSON API to rust representation
mod api_schema;

/// Constants used everywhere
mod constants;

/// Formats and prints output-tables
mod table_formatting;

/// Functions for invoking the API
mod api_interactions;

mod localization;
mod mensa_menu;
mod menu_impl;
mod rest_api_impl;
mod simple_argparse;

static COLOR: OnceLock<ColorChoice> = OnceLock::new();

fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let args = argparse()?;

    let _ = env::var("MENSA_LIMIT").map(|e| set_lower_threshold_int(u16::from_str(&e).unwrap()));

    let days = vec![
        "monday",
        "tuesday",
        "wednesday",
        "thursday",
        "friday",
        "saturday",
        "sunday",
    ];
    let today = time::OffsetDateTime::now_local()?.weekday();

    let current_day = if args.tomorrow { today.next() } else { today }
        .to_string()
        .to_lowercase();

    if !days.contains(&current_day.as_str()) {
        bail!("Unknown weekday argument");
    }

    let week_days = days
        .into_iter()
        .cycle()
        .skip_while(|day| day != &current_day)
        .take(7)
        .collect::<Vec<_>>();

    // Fetch menus from today through all weekdays until a valid menu is found
    let (mut menus, day) = {
        let mut menu = None;
        for query_param in week_days {
            let menus = fetch_menus(query_param)?;
            if Menu::count_meals(menus.iter()) == 0 {
                eprintln!("No food for {query_param}, picking next possible date");
                continue;
            }
            menu = Some((menus, query_param));
            break;
        }
        menu
    }
    .context(no_menus())?;

    for menu in &menus {
        if menu.menu.meals.is_empty() {
            eprintln!("No meals listed for {}", menu.mensa_id)
        }
    }
    menus = menus
        .into_iter()
        .filter(|e| !e.menu.meals.is_empty())
        .collect(); // Filter places without any food

    let longest_meal_name = MensaMenu::longest_menu_name(&menus)?;
    let most_expensive_price = Menu::most_expensive_meals(&menus)?;

    COLOR.get_or_init(|| {
        if std::env::var("NO_COLOR").is_ok() {
            ColorChoice::Never
        } else {
            ColorChoice::Auto
        }
    });

    render_meta(longest_meal_name, &day)?;

    render_menus(menus, longest_meal_name, most_expensive_price)?;
    Ok(())
}
