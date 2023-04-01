use std::str::FromStr;

use chrono::{DateTime, Local, NaiveDate, ParseError, TimeZone};
use clap::ArgMatches;

pub fn date_superscript(day: u32) -> &'static str {
    let unit = day % 10;

    match unit {
        1 => "st",
        2 => "nd",
        3 => "rd",
        _ => "th",
    }
}

pub fn parse_date_option(args: &ArgMatches) -> Result<DateTime<Local>, ParseError> {
    Ok(match args.get_one::<String>("date") {
        Some(val) => {
            let date = NaiveDate::from_str(val)?.and_hms_opt(0, 0, 0).unwrap();
            Local.from_utc_datetime(&date)
        }
        _ => Local::now(),
    })
}

#[cfg(test)]
mod tests {
    use super::date_superscript;
    #[test]
    fn date_superscript_st() {
        assert_eq!("st", date_superscript(21));
    }
    #[test]
    fn date_superscript_nd() {
        assert_eq!("nd", date_superscript(12));
    }
    #[test]
    fn date_superscript_rd() {
        assert_eq!("rd", date_superscript(23));
    }
    #[test]
    fn date_superscript_th() {
        assert_eq!("th", date_superscript(17));
    }
}
