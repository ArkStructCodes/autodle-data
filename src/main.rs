mod parser;
mod utils;

use std::collections::HashMap;

use anyhow::{anyhow, Result};
use parser::CarList;

use crate::utils::{scrape_url, set_param};

const BASE_URL: &str = "https://www.kudosprime.com/fh5/carlist.php?range=2000";

fn main() -> Result<()> {
    let dom = scrape_url(BASE_URL)?;
    let carlist = CarList::parse(&dom)?;

    let country_map = carlist
        .get_countries()
        .ok_or(anyhow!("failed to parse the country list"))?
        .filter_map(|country| {
            let url = set_param(BASE_URL, "country", country);
            let dom = scrape_url(&url).ok()?;
            let carlist = CarList::parse(&dom).ok()?;
            let makes = carlist.get_makes()?.map(String::from).collect::<Vec<_>>();

            Some((country, makes))
        })
        .flat_map(|(country, makes)| makes.into_iter().map(move |make| (make, country)))
        .collect::<HashMap<_, _>>();

    let mut cars = carlist
        .get_cars()
        .ok_or(anyhow!("failed to parse car data"))?
        .filter(|car| {
            !(car.year == 2554
                || car.make == "Hoonigan"
                || car.make == "Alumi Craft"
                || car.make == "Deberti"
                || car.make == "Forsberg Racing"
                || car.make == "Penhall"
                || car.make == "Funco Motorsports"
                || car.make == "Hot Wheels"
                || car.make.contains("Universal")
                || car.make.contains("Racing")
                || car.name.contains(['#', '\'', '"'])
                || car.name.contains("Concept")
                || car.name.contains("Hot Wheels")
                || car.name.contains("Raid")
                || car.name.contains("W154")
                || car.name.contains("Baja")
                || car.name.contains("M-Sport")
                || car.name.contains("Limousine")
                || car.name.contains("Pro Stock")
                || car.name.contains("Edition")
                || car.name.contains("Movie")
                || car.name.contains("Singer")
                || car.name.contains("Gunther Werks")
                || car.name.contains("Mission R")
                || car.name.contains("ARX"))
        })
        .collect::<Vec<_>>();

    cars.iter_mut().for_each(|car| {
        car.country = country_map
            .get(car.make)
            .expect("failed to get the country for the given make");
    });

    let json = serde_json::to_string_pretty(&cars)?;
    println!("{json}");

    Ok(())
}
