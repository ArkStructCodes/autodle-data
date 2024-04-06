use std::str::FromStr;

pub(crate) use once_cell::sync::Lazy;
pub(crate) use scraper::Selector;

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

pub(crate) fn parse<T: FromStr>(s: &str) -> Option<T> {
    s.trim().replace(',', "").parse().ok()
}
