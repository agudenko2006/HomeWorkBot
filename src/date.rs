use chrono::{Datelike, Local};

use color_eyre::eyre::{Result, ContextCompat};
use regex::Regex;

/// YMD date
#[derive(Debug, PartialEq)]
pub struct Date(pub u8, pub u8, pub u8);

impl Date {
    pub fn new(year: u8, month: u8, day: u8) -> Self {
        Self(year, month, day)
    }

    /// String in any format that contains ymd
    /// todo!("add tests")
    pub fn from(string: &str) -> Result<Self> {
        let regex = Regex::new(r"^(\d{2})?-?(\d{2})-?(\d{2})$")?;
        let caps = regex
            .captures(string).wrap_err(format!("`{}` is not a Date", string))?;

        let year = caps.get(1);
        let month = caps.get(2).unwrap().as_str().parse().unwrap();
        let day = caps.get(3).unwrap().as_str().parse().unwrap();

        Ok(Self(
            match year {
                Some(year) => year.as_str().parse().unwrap(),
                None => 23, // todo!("get the actual year")
            },
            month,
            day,
        ))
    }

    pub fn now() -> Self {
        let system_date = Local::now();
        let year = system_date.year() - 2000;

        Self(
            year.try_into().unwrap(),
            system_date.month().try_into().unwrap(),
            system_date.day().try_into().unwrap(),
        )
    }

    pub fn comes_before(&self, another: Self) -> bool {
        // Please create an issue if you know how to do it properly
        self.0 < another.0 || self.1 < another.1 || self.2 < another.2
    }

    pub fn had_passed(&self) -> bool {
        let now = Date::now();
        self.comes_before(now)
    }
}
