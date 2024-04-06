use anyhow::{anyhow, Result};
use scraper::{ElementRef, Html};
use serde::Serialize;

crate::static_selector! {
    CARLIST = "div#carlist",
    NAME = "a.name",
    DRIVETRAIN = "span.tr",
    STATS = "div.tpw",
}

#[derive(Serialize)]
pub(crate) struct Car {
    make: String,
    year: u16,
    name: String,
    drivetrain: String,
    power: u16,
    weight: u16,
}

impl Car {
    fn parse(make: Option<&str>, el: ElementRef) -> Option<Self> {
        // Since the year will always be followed by a space, we let the parse
        // function trim that for us later on.
        let (year, name) = el.select(&NAME).next()?.text().next()?.split_at(5);
        let drivetrain = el.select(&DRIVETRAIN).next()?.text().next()?;
        let mut stats = el.select(&STATS).next()?.text();
        let power = stats.next()?;
        let weight = stats.skip(1).next()?;

        Some(Self {
            make: make?.to_owned(),
            year: crate::utils::parse(year)?,
            name: name.to_owned(),
            drivetrain: drivetrain.to_owned(),
            power: crate::utils::parse(power)?,
            weight: crate::utils::parse(weight)?,
        })
    }
}

pub(crate) fn parse_carlist(html: &str) -> Result<Vec<Car>> {
    let document = Html::parse_document(html);
    let carlist = document
        .select(&CARLIST)
        .next()
        .ok_or(anyhow!("failed to parse the carlist"))?;

    let mut cars = vec![];
    let mut current_make = None;

    for node in carlist.children() {
        if !node.value().is_element() {
            continue;
        }

        // This should not panic since we perform the check above.
        let el = ElementRef::wrap(node).unwrap();

        match el.value().name() {
            "div" => {
                let car = Car::parse(current_make, el);
                cars.extend(car);
            }
            "p" => {
                let next_make = el
                    .text()
                    .next()
                    .ok_or(anyhow!("failed to parse make name"))?;

                current_make = Some(next_make);
            }
            _ => (),
        }
    }

    Ok(cars)
}
