use std::cell::Cell;

use anyhow::{anyhow, Result};
use scraper::{ElementRef, Html};
use serde::Serialize;

crate::static_selector! {
    CAR_LIST = "div#carlist",
    NAME = "a.name",
    DRIVETRAIN = "span.tr",
    STATS = "div.tpw",
    COUNTRIES = "select[name=country]",
}

#[derive(Default, Serialize)]
pub(crate) struct Car<'a> {
    pub country: &'a str,
    pub make: &'a str,
    pub year: u16,
    pub name: &'a str,
    pub drivetrain: &'a str,
    pub power: u16,
    pub weight: u16,
}

impl<'a> Car<'a> {
    fn parse(make: &'a str, el: ElementRef<'a>) -> Option<Self> {
        // since the year is always followed by a space, we let the parse function
        // trim that for us later on
        let (year, name) = el.select(&NAME).next()?.text().next()?.split_at(5);
        let drivetrain = el.select(&DRIVETRAIN).next()?.text().next()?;
        let mut stats = el.select(&STATS).next()?.text();
        let power = stats.next()?;
        let weight = stats.nth(1)?;

        Some(Self {
            make,
            year: crate::utils::parse(year)?,
            name,
            drivetrain,
            power: crate::utils::parse(power)?,
            weight: crate::utils::parse(weight)?,
            ..Default::default()
        })
    }
}

pub(crate) struct CarList<'a> {
    dom: &'a Html,
    el: ElementRef<'a>,
}

#[rustfmt::skip]
impl<'a> CarList<'a> {
    pub fn parse(dom: &'a Html) -> Result<Self> {
        let el = dom
            .select(&CAR_LIST)
            .next()
            .ok_or(anyhow!("failed to parse the carlist page"))?;

        Ok(Self { dom, el })
    }

    pub fn get_countries(&self) -> Option<impl Iterator<Item = &str>> {
        Some(self
            .dom
            .select(&COUNTRIES)
            .next()?
            .children()
            .filter_map(ElementRef::wrap)
            .filter_map(|element| element.text().next())
            // the first option is ----- so we skip it
            .skip(1))
    }

    pub fn get_makes(&self) -> Option<impl Iterator<Item = &str>> {
        Some(self.el
            .children()
            .filter_map(ElementRef::wrap)
            .filter_map(|element| match element.value().name() {
                "p" => Some(element.text().next()?),
                 _  => None,
            }))
    }

    pub fn get_cars(&self) -> Option<impl Iterator<Item = Car>> {
        let make = Cell::new(None);
        Some(self
            .el
            .children()
            .filter_map(ElementRef::wrap)
            .filter_map(move |element| match element.value().name() {
                "div" => Some(Car::parse(make.get()?, element)?),
                "p" => {
                    make.set(Some(element.text().next()?));
                    None
                }
                _ => None,
            }))
    }
}
