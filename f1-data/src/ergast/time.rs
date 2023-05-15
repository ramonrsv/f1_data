use std::ops::{Deref, Sub};
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum ParseError {
    InvalidDuration(String),
    InvalidTime(String),
    InvalidRaceTime(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ParseError {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Duration(time::Duration);

impl Duration {
    pub const ZERO: Self = Self(time::Duration::ZERO);

    pub const fn milliseconds(milliseconds: i64) -> Self {
        Self(time::Duration::milliseconds(milliseconds))
    }

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

    fn parse_hm(hm_str: &str) -> (i64, i64) {
        let hm: Vec<&str> = hm_str.split_at(hm_str.len() - 1).0.split(':').collect();
        debug_assert!(!hm.is_empty() && hm.len() <= 2);

        let hours = if hm.len() == 2 { hm[0].parse().unwrap() } else { 0 };
        let minutes = hm.last().unwrap().parse().unwrap();

        (hours, minutes)
    }

    fn parse_milli(milli_str: &str) -> i64 {
        debug_assert!(!milli_str.is_empty() && milli_str.len() <= 3);

        milli_str.parse::<i64>().unwrap() * (10_i64.pow(3_u32 - milli_str.len() as u32))
    }

    pub fn parse(d_str: &str) -> Result<Self, ParseError> {
        const FORMAT_REGEX_STR: &str = r"^((?:\d:)?(?:[0-5]?\d:)?)(?:([0-5]?\d)\.)(\d{1,3})$";

        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(FORMAT_REGEX_STR).unwrap());

        let matches = RE
            .captures(d_str)
            .ok_or(ParseError::InvalidDuration(d_str.to_string()))?;

        let (hours, minutes) = if matches[1].is_empty() {
            (0, 0)
        } else {
            Self::parse_hm(&matches[1])
        };

        let seconds = matches[2].parse().unwrap();
        let milliseconds = Self::parse_milli(&matches[3]);

        Ok(Self::from_hms_ms(hours, minutes, seconds, milliseconds))
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Duration::parse(&String::deserialize(deserializer)?).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl FromStr for Duration {
    type Err = ParseError;

    fn from_str(d_str: &str) -> Result<Self, Self::Err> {
        Duration::parse(d_str)
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

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.sub(rhs.0))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Time(time::Time);

impl Time {
    const TIME_FORMAT_DESCRIPTION: &'static [time::format_description::FormatItem<'static>] =
        time::macros::format_description!("[hour]:[minute]:[second]Z");

    pub fn from_hms(hour: u8, minute: u8, second: u8) -> Result<Self, time::error::ComponentRange> {
        Ok(Self(time::Time::from_hms(hour, minute, second)?))
    }

    pub fn parse(t_str: &str) -> Result<Self, ParseError> {
        time::Time::parse(t_str, &Self::TIME_FORMAT_DESCRIPTION)
            .map(Time)
            .map_err(|err| ParseError::InvalidTime(format!("'{t_str}': {err}")))
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Time::parse(&String::deserialize(deserializer)?).map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

impl FromStr for Time {
    type Err = ParseError;

    fn from_str(t_str: &str) -> Result<Self, Self::Err> {
        Time::parse(t_str)
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
        let dur = Duration::from_hms_ms(1, 2, 34, 567);

        assert_eq!(dur.whole_hours(), 1);
        assert_eq!(dur.whole_minutes() - 60, 2);
        assert_eq!(dur.whole_seconds() - (60_i64.pow(2) * 1) - (60 * 2), 34);
        assert_eq!(dur.subsec_milliseconds(), 567);
    }

    #[test]
    fn duration_from_m_s_ms() {
        let dur = Duration::from_m_s_ms(1, 23, 456);

        assert_eq!(dur.whole_minutes(), 1);
        assert_eq!(dur.whole_seconds() - 60, 23);
        assert_eq!(dur.subsec_milliseconds(), 456);
    }

    #[test]
    fn duration_parse() {
        let m_s_ms = |m, s, ms| Duration::from_m_s_ms(m, s, ms);
        let hms_ms = |h, m, s, ms| Duration::from_hms_ms(h, m, s, ms);

        let pairs = Vec::from([
            ("1:22.327", m_s_ms(1, 22, 327)),
            ("1:41.269", m_s_ms(1, 41, 269)),
            ("59.037", m_s_ms(0, 59, 037)),
            ("2:01.341", m_s_ms(2, 1, 341)),
            ("10.1", m_s_ms(0, 10, 100)),
            ("1:22.327", m_s_ms(1, 22, 327)),
            ("1:28:12.058", hms_ms(1, 28, 12, 58)),
            ("33:17.667", m_s_ms(33, 17, 667)),
            ("2:02:53.7", hms_ms(2, 2, 53, 700)),
            ("0.4", m_s_ms(0, 0, 400)),
            ("1.882", m_s_ms(0, 1, 882)),
            ("1:08.436", m_s_ms(1, 8, 436)),
            ("40.111", m_s_ms(0, 40, 111)),
        ]);

        for (dur_str, dur) in pairs.iter() {
            assert_eq!(&Duration::parse(dur_str).unwrap(), dur);
        }
    }

    #[test]
    fn duration_parse_err() {
        let bad_dur_strings = Vec::from([
            "90.203",
            "40.1111",
            "",
            ":",
            ":2.100",
            "1::2.100",
            "1:61.100",
            "1:60:30.100",
        ]);

        for bad_dur_str in bad_dur_strings {
            assert!(matches!(Duration::parse(bad_dur_str).unwrap_err(), ParseError::InvalidDuration(_)));
        }
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
    fn time_parse_err() {
        assert!(Time::parse("12:00:00").is_err());
        assert!(Time::parse("12:00:0Z").is_err());
        assert!(Time::parse("25:00:00Z").is_err());
        assert!(Time::parse("12:00Z").is_err());
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
