use std::str::FromStr;

pub(crate) use once_cell::sync::Lazy;
pub(crate) use scraper::Selector;

use anyhow::Result;
use scraper::Html;
use ureq::Agent;

pub(crate) type LazySelector = Lazy<Selector>;

#[macro_export]
macro_rules! static_selector {
    ($($name:ident = $expr:literal),+ $(,)+) => {
        $(
            static $name: $crate::utils::LazySelector = $crate::utils::Lazy::new(|| {
                $crate::utils::Selector::parse($expr).unwrap()
            });
        )+
    };
}

static AGENT: Lazy<Agent> = Lazy::new(Agent::new);

pub(crate) fn scrape_url(url: &str) -> Result<Html> {
    let contents = AGENT.get(url).call()?.into_string()?;
    Ok(Html::parse_document(&contents))
}

pub(crate) fn set_param(url: &str, param: &str, value: &str) -> String {
    format!("{url}&{param}={value}")
}

pub(crate) fn parse<T: FromStr>(s: &str) -> Option<T> {
    s.trim().replace(',', "").parse().ok()
}
