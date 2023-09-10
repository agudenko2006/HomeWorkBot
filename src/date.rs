use regex::Regex;

/// YMD date
#[derive(Debug)]
pub struct Date(u8, u8, u8);

impl Date {
    pub fn new(year: u8, month: u8, day: u8) -> Self {
        Self(year, month, day)
    }

    /// String in any format that contains ymd
    /// todo!("add tests")
    pub fn from(string: &str) -> Self {
        let regex = Regex::new(r"^(\d{2})?-?(\d{2})-?(\d{2})$").unwrap();
        let caps = regex
            .captures(string)
            .expect(&format!("`{}` is not a Date", string));

        let year = caps.get(1);
        let month = caps.get(2).unwrap().as_str().parse().unwrap();
        let day = caps.get(3).unwrap().as_str().parse().unwrap();

        Self(
            match year {
                Some(year) => year.as_str().parse().unwrap(),
                None => 23, // todo!("get the actual year")
            },
            month,
            day,
        )
    }
}
