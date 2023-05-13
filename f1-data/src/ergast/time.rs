use std::ops::Deref;
use std::result::Result;
use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::de::{Deserialize, Deserializer};

#[derive(Debug)]
pub enum ParseError {
    NoMatch(String),
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

    pub fn from_m_s_ms(minutes: i64, seconds: i64, milliseconds: i64) -> Self {
        Self(
            time::Duration::minutes(minutes)
                + time::Duration::seconds(seconds)
                + time::Duration::milliseconds(milliseconds),
        )
    }

    pub fn parse(time: &str) -> Result<Self, ParseError> {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(Duration::FORMAT_REGEX_STR).unwrap());

        let matches = RE.captures(time).ok_or(ParseError::NoMatch(time.to_string()))?;

        let minutes = matches.get(2).map(|m| m.as_str()).unwrap_or("0").parse().unwrap();
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

impl Deref for Duration {
    type Target = time::Duration;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert!(matches!(Duration::parse("90.203").unwrap_err(), ParseError::NoMatch(_)));
        assert!(matches!(Duration::parse("10.1").unwrap_err(), ParseError::NoMatch(_)));
        assert!(matches!(Duration::parse("40.1111").unwrap_err(), ParseError::NoMatch(_)));
        assert!(matches!(Duration::parse("").unwrap_err(), ParseError::NoMatch(_)));
    }

    #[test]
    fn duration_deserialize() {
        assert_eq!(serde_json::from_str::<Duration>(r#""1:22.327""#).unwrap(), Duration::from_m_s_ms(1, 22, 327));
    }
}
