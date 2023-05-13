use std::ops::Deref;
use std::result::Result;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum ParseError {
    InvalidDuration(String),
    InvalidTime(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

#[derive(PartialEq, Clone, Debug)]
pub struct Duration(time::Duration);

impl Duration {
    pub const FORMAT_REGEX_STR: &str = r"^((\d):)?([0-5]?\d)\.(\d{3})$";

    pub fn from_hms_ms(hours: i64, minutes: i64, seconds: i64, milliseconds: i64) -> Self {
        Self(
            time::Duration::hours(hours)
                + time::Duration::minutes(minutes)
                + time::Duration::seconds(seconds)
                + time::Duration::milliseconds(milliseconds),
        )
    }

    pub fn from_m_s_ms(minutes: i64, seconds: i64, milliseconds: i64) -> Self {
        Self::from_hms_ms(0, minutes, seconds, milliseconds)
    }

    pub fn parse(time: &str) -> Result<Self, ParseError> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(Duration::FORMAT_REGEX_STR).unwrap());

        let matches = RE.captures(time).ok_or(ParseError::InvalidDuration(time.to_string()))?;

        let minutes = matches.get(2).map_or("0", |m| m.as_str()).parse().unwrap();
        let seconds = matches[3].parse().unwrap();
        let milliseconds = matches[4].parse().unwrap();

        Ok(Self::from_m_s_ms(minutes, seconds, milliseconds))
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Duration::parse(&String::deserialize(deserializer)?).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl FromStr for Duration {
    type Err = ParseError;

    fn from_str(time: &str) -> Result<Self, Self::Err> {
        Duration::parse(time)
    }
}

impl From<time::Duration> for Duration {
    fn from(dur: time::Duration) -> Self {
        Self(dur)
    }
}

impl Deref for Duration {
    type Target = time::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Time(time::Time);

impl Time {
    const TIME_FORMAT_DESCRIPTION: &'static [time::format_description::FormatItem<'static>] =
        time::macros::format_description!("[hour]:[minute]:[second]Z");

    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Result<Self, time::error::ComponentRange> {
        Ok(Self(time::Time::from_hms(hour, minute, second)?))
    }

    pub fn parse(time: &str) -> Result<Self, ParseError> {
        time::Time::parse(time, &Self::TIME_FORMAT_DESCRIPTION)
            .map(Time)
            .map_err(|err| ParseError::InvalidTime(format!("'{time}': {err}")))
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Time::parse(&String::deserialize(deserializer)?).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl FromStr for Time {
    type Err = ParseError;

    fn from_str(time: &str) -> Result<Self, Self::Err> {
        Time::parse(time)
    }
}

impl From<time::Time> for Time {
    fn from(time: time::Time) -> Self {
        Self(time)
    }
}

impl Deref for Time {
    type Target = time::Time;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type Date = time::Date;

pub mod macros {
    #[macro_export]
    macro_rules! time {
    ($($l:tt)*) => {
        $crate::ergast::time::Time::from(time::macros::time!($($l)*))
    }
}

    pub use ::time::macros::date;
    pub use time;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duration_from_hms_ms() {
        let lap = Duration::from_hms_ms(1, 2, 34, 567);

        assert_eq!(lap.whole_hours(), 1);
        assert_eq!(lap.whole_minutes() - 60, 2);
        assert_eq!(lap.whole_seconds() - (60_i64.pow(2) * 1) - (60 * 2), 34);
        assert_eq!(lap.subsec_milliseconds(), 567);
    }

    #[test]
    fn duration_from_m_s_ms() {
        let lap = Duration::from_m_s_ms(1, 23, 456);

        assert_eq!(lap.whole_minutes(), 1);
        assert_eq!(lap.whole_seconds() - 60, 23);
        assert_eq!(lap.subsec_milliseconds(), 456);
    }

    #[test]
    fn duration_parse() {
        assert_eq!(Duration::parse("1:22.327").unwrap(), Duration::from_m_s_ms(1, 22, 327));
        assert_eq!(Duration::parse("1:41.269").unwrap(), Duration::from_m_s_ms(1, 41, 269));
        assert_eq!(Duration::parse("59.037").unwrap(), Duration::from_m_s_ms(0, 59, 037));
        assert_eq!(Duration::parse("2:01.341").unwrap(), Duration::from_m_s_ms(2, 1, 341));
    }

    #[test]
    fn duration_parse_err() {
        assert!(matches!(Duration::parse("90.203").unwrap_err(), ParseError::InvalidDuration(_)));
        assert!(matches!(Duration::parse("10.1").unwrap_err(), ParseError::InvalidDuration(_)));
        assert!(matches!(Duration::parse("40.1111").unwrap_err(), ParseError::InvalidDuration(_)));
        assert!(matches!(Duration::parse("").unwrap_err(), ParseError::InvalidDuration(_)));
    }

    #[test]
    fn duration_deserialize() {
        assert_eq!(serde_json::from_str::<Duration>(r#""1:22.327""#).unwrap(), Duration::from_m_s_ms(1, 22, 327));
    }

    #[test]
    fn time_from_hms() {
        assert_eq!(Time::from_hms(1, 2, 3).unwrap().0, time::Time::from_hms(1, 2, 3).unwrap());
    }

    #[test]
    fn time_parse() {
        assert_eq!(Time::parse("12:00:00Z").unwrap(), Time::from_hms(12, 0, 0).unwrap());
        assert_eq!(Time::parse("11:30:00Z").unwrap(), Time::from_hms(11, 30, 0).unwrap());
        assert_eq!(Time::parse("15:00:00Z").unwrap(), Time::from_hms(15, 0, 0).unwrap());
        assert_eq!(Time::parse("10:30:00Z").unwrap(), Time::from_hms(10, 30, 0).unwrap());
    }

    #[test]
    fn time_macro() {
        assert_eq!(Time::from_hms(11, 30, 0).unwrap(), macros::time!(11:30:00));
    }

    #[test]
    fn time_deserialize() {
        assert_eq!(serde_json::from_str::<Time>(r#""11:30:00Z""#).unwrap(), Time::from_hms(11, 30, 0).unwrap());
    }
}
