mod parser;
mod utils;

use anyhow::Result;

use crate::parser::Car;

fn main() -> Result<()> {
    let html = ureq::get("https://www.kudosprime.com/fh5/carlist.php?range=2000")
        .call()?
        .into_string()?;

    let cars = parser::parse_carlist(&html)?;
    let filtered = cars
        .into_iter()
        .filter(|car| {
            !(car.year == 2554
                || car.name.contains(&['#', '\'', '"'])
                || car.name.contains("Edition")
                || car.name.contains("Movie")
                || car.name.contains("Hot Wheels")
                || car.make.contains("Universal")
                || car.make == "Hoonigan")
        })
        .collect::<Vec<Car>>();

    let json = serde_json::to_string_pretty(&filtered)?;
    println!("{json}");

    Ok(())
}
