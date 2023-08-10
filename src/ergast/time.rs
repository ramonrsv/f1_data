use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr};

/// These aliases represent the underlying time/date/duration/etc. types used within the crate to
/// represent such values from the Ergast API, sometimes as direct aliases, e.g. for a [`Date`], or
/// used to compose more complex types, e.g. [`DateTime], [`QualifyingTime`], [`RaceTime`], etc.
// @todo Enable features to use different underlying libraries, e.g. [`time`], `chrono`, etc.
use time as underlying;

pub type Time = underlying::Time;
pub type Date = underlying::Date;
pub type Duration = underlying::Duration;

pub mod macros {
    pub use super::underlying::macros::date;
    pub use super::underlying::macros::time;
}

/// Construct a [`Duration`] from a number of hours, minutes, seconds, and milliseconds.
pub fn duration_hms_ms(hours: i64, minutes: i64, seconds: i64, milliseconds: i64) -> Duration {
    Duration::hours(hours)
        + Duration::minutes(minutes)
        + Duration::seconds(seconds)
        + Duration::milliseconds(milliseconds)
}

/// Construct a [`Duration`] from a number of minutes, seconds, and milliseconds.
pub fn duration_m_s_ms(minutes: i64, seconds: i64, milliseconds: i64) -> Duration {
    Duration::minutes(minutes) + Duration::seconds(seconds) + Duration::milliseconds(milliseconds)
}

/// Construct a [`Duration`] from a number of seconds and milliseconds.
pub fn duration_s_ms(seconds: i64, milliseconds: i64) -> Duration {
    Duration::seconds(seconds) + Duration::milliseconds(milliseconds)
}

#[cfg(test)]
/// Construct a [`Duration`] from a number of milliseconds.
pub(crate) fn duration_millis(milliseconds: i64) -> Duration {
    Duration::milliseconds(milliseconds)
}

/// Parse an integer element from a time string, e.g. `"41"` from `"1:41.269"` -> `41`.
/// Panics if the string cannot be parsed into an integer, i.e. no empty strings allowed.
fn parse_integer(s: &str) -> i64 {
    s.parse::<i64>().unwrap()
}

/// Parse an integer from an optional regex match, returning the default if the match is `None`.
fn parse_integer_or(mtch: Option<regex::Match<'_>>, default: i64) -> i64 {
    mtch.map_or(default, |mtch| parse_integer(mtch.as_str()))
}

/// Parse a `[subseconds]` string into milliseconds, e.g. `"123"` -> `123`, `"12"` -> `120`, etc.
/// See <https://time-rs.github.io/book/api/format-description.html> for more format information.
fn parse_subsecond_into_milli(subsec_str: &str) -> i64 {
    debug_assert!(!subsec_str.is_empty() && subsec_str.len() <= 3);

    subsec_str.parse::<i64>().unwrap() * (10_i64.pow(3_u32 - u32::try_from(subsec_str.len()).unwrap()))
}

/// Parse a [`Time`] from a string in the format `HH:MM:SSZ`, e.g. `11:00:00Z`.
/// This format represents times of day in the Ergast API, e.g. the start time of an event.
fn parse_time(raw_str: &str) -> Result<Time, underlying::error::Parse> {
    const TIME_FORMAT_DESCRIPTION: &[underlying::format_description::FormatItem<'static>] =
        underlying::macros::format_description!("[hour]:[minute]:[second]Z");

    Time::parse(raw_str, &TIME_FORMAT_DESCRIPTION)
}

/// Parse a [`Duration`] from a string in the format `H:MM:SS.SSS`, e.g. `"2:05:05.152"`.
/// This format represents durations the Ergast API, i.e. the duration of a single lap or race.
/// Note that the parsing is very permissive, allowing the `[hour]` and `[minute]` components to be
/// omitted, and allowing all other components to have fewer than the maximum number of digits.
fn parse_duration(raw_str: &str) -> Result<Duration, String> {
    const FORMAT_REGEX_STR: &str = r"^(?:(\d{1,2}):)?(?:([0-5]?\d):)?([0-5]?\d)\.(\d{1,3})$";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(FORMAT_REGEX_STR).unwrap());

    let matches = RE
        .captures(raw_str)
        .ok_or_else(|| format!("Invalid duration: \"{raw_str}\""))?;

    // Group 1 matches `[minute]` if `[hour]` is not present, otherwise matches `[hour]`.
    // Group 2 matches `[minute]` if `[hour]` is present, otherwise there is no group 2 match.
    let has_hours = matches.get(2).is_some();

    let hours = if has_hours { parse_integer(&matches[1]) } else { 0 };
    let minutes = parse_integer_or(matches.get(if has_hours { 2 } else { 1 }), 0);

    let seconds = parse_integer(&matches[3]);
    let milliseconds = parse_subsecond_into_milli(&matches[4]);

    Ok(duration_hms_ms(hours, minutes, seconds, milliseconds))
}

/// Parse a [`Duration`] from a string in one of the following formats: `+SSS.SSS` OR `+M:SS.SSS`,
/// for example `+0.4`, `+1.882`, `+21.217`, `+89.241`, `+103.796`, `+1:14.240`, etc.
/// These formats represent delta times in the Ergast API, i.e. the difference between lap times.
// @todo There is no consistent format for delta times in the Ergast API. Should that be fixed?
fn parse_delta(raw_str: &str) -> Result<Duration, String> {
    const FORMAT_REGEX_STR: &str = r"^\+(?:(\d{1,2}):)?(\d{1,3})\.(\d{1,3})$";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(FORMAT_REGEX_STR).unwrap());

    let matches = RE
        .captures(raw_str)
        .ok_or_else(|| format!("Invalid delta time: \"{raw_str}\""))?;

    let minutes = parse_integer_or(matches.get(1), 0);
    let seconds = parse_integer(&matches[2]);
    let milliseconds = parse_subsecond_into_milli(&matches[3]);

    Ok(duration_m_s_ms(minutes, seconds, milliseconds))
}

/// Deserialize an optional [`Time`] via [`parse_time`].
pub(crate) fn deserialize_optional_time<'de, D>(deserializer: D) -> Result<Option<Time>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|s| parse_time(&s).map_err(serde::de::Error::custom))
        .transpose()
}

/// Deserialize a [`Duration`] via [`parse_duration`].
pub(crate) fn deserialize_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    parse_duration(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
/// Represents a date and optional time in the Ergast API, e.g. the date and start time of an event.
/// This is similar to, say [`time::PrimitiveDateTime`], but the time may not always be present.
pub struct DateTime {
    /// The date component of the date-time.
    pub date: Date,
    /// The optional time component of the date-time.
    #[serde(default, deserialize_with = "deserialize_optional_time")]
    pub time: Option<Time>,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Represents the duration of the best qualifying lap set by a driver in a qualifying session,
/// e.g. Q1, Q2, etc. A lap time is represented by the [`QualifyingTime::Time`]. If a driver took
/// part in a qualifying session but did not set a lap time, then [`QualifyingTime::NoTimeSet`].
pub enum QualifyingTime {
    /// The duration of the best qualifying lap set by a driver in a qualifying session.
    Time(Duration),
    /// The driver took part in a qualifying session but did not set a lap time.
    NoTimeSet,
}

impl QualifyingTime {
    /// Returns `true` if the driver set a lap time in a qualifying session.
    pub fn has_time(&self) -> bool {
        matches!(self, Self::Time(_))
    }

    /// Returns `true` if the driver took part in a qualifying session but did not set a lap time.
    pub fn no_time_set(&self) -> bool {
        matches!(self, Self::NoTimeSet)
    }

    /// Returns the lap time, as [`Duration`], if the driver set a lap time in a qualifying session.
    ///
    /// # Panics
    ///
    /// Panics if the driver took part in a qualifying session but did not set a lap time.
    pub fn time(&self) -> &Duration {
        match &self {
            Self::Time(time) => time,
            Self::NoTimeSet => panic!("Cannot get time of NoTimeSet"),
        }
    }
}

impl<'de> Deserialize<'de> for QualifyingTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw_str = &String::deserialize(deserializer)?;

        if raw_str.is_empty() {
            Ok(Self::NoTimeSet)
        } else {
            parse_duration(raw_str)
                .map(Self::Time)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
/// Represents the full race duration for a single driver, including a delta to the race leader/P1.
/// This is only present if a driver finished in the lead lap, if their race status is `"Finished"`.
pub struct RaceTime {
    /// Total race duration for the driver.
    total: Duration,
    /// Delta to the leader's race duration, which is zero for the leader/P1's [`RaceTime`].
    delta: Duration,
}

impl RaceTime {
    /// Construct a [`RaceTime`] for the leader/P1, i.e. with a zero delta to the leader's time.
    pub fn lead(total: Duration) -> Self {
        Self {
            total,
            delta: Duration::ZERO,
        }
    }

    /// Construct a [`RaceTime`] for a driver other than the leader/P1, i.e. with a non-zero delta.
    pub fn with_delta(total: Duration, delta: Duration) -> Self {
        assert!(delta < total);

        Self { total, delta }
    }

    /// Returns `true` if this [`RaceTime`] is for the leader/P1, i.e. with a zero delta.
    pub fn is_lead(&self) -> bool {
        self.delta == Duration::ZERO
    }

    /// Get the total race duration for a driver, as [`Duration`].
    pub fn total(&self) -> &Duration {
        &self.total
    }

    /// Get the delta to the leader/P1's race duration, as [`Duration`]. It's zero for the leader.
    pub fn delta(&self) -> &Duration {
        &self.delta
    }
}

impl<'de> Deserialize<'de> for RaceTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[serde_as]
        #[derive(Deserialize, Debug)]
        struct Proxy {
            #[serde_as(as = "DisplayFromStr")]
            millis: u32,
            time: String,
        }

        let proxy = Proxy::deserialize(deserializer)?;

        if proxy.time.is_empty() {
            return Err(serde::de::Error::custom("Unexpected empty 'time' in RaceTime".to_string()));
        }

        let has_delta = proxy.time.starts_with('+');

        let total = Duration::milliseconds(i64::from(proxy.millis));

        let delta = if has_delta {
            parse_delta(&proxy.time)
        } else {
            parse_duration(&proxy.time)
        }
        .map_err(serde::de::Error::custom)?;

        if !has_delta && (total != delta) {
            return Err(serde::de::Error::custom(format!(
                "Non-delta 'time: {}' must match 'millis: {}'",
                proxy.time, proxy.millis
            )));
        }

        if delta > total {
            return Err(serde::de::Error::custom(format!(
                "Delta 'time: {}' must be less than 'millis: {}'",
                proxy.time, proxy.millis
            )));
        }

        if has_delta {
            Ok(Self::with_delta(total, delta))
        } else {
            Ok(Self::lead(total))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use super::macros::*;
    use super::*;
    use crate::ergast::tests::*;

    const MIN_IN_HOUR: i64 = 60;
    const SEC_IN_MIN: i64 = 60;
    const SEC_IN_HOUR: i64 = SEC_IN_MIN * MIN_IN_HOUR;

    const UNIVERSAL_BAD_DURATION_STRINGS: &[&'static str] = &["40.1111", "", ":", ":2.100", "1::2.100"];

    fn make_bad_duration_strings(case_specific_bad_duration_strings: &[&'static str]) -> Vec<&'static str> {
        let mut a = Vec::from(case_specific_bad_duration_strings);
        let mut b = Vec::from(UNIVERSAL_BAD_DURATION_STRINGS);
        a.append(&mut b);
        a
    }

    #[test]
    fn duration_hms_ms() {
        let dur = super::duration_hms_ms(1, 2, 34, 567);

        assert_eq!(dur.whole_hours(), 1);
        assert_eq!(dur.whole_minutes(), (MIN_IN_HOUR * 1) + 2);
        assert_eq!(dur.whole_seconds(), (SEC_IN_HOUR * 1) + (SEC_IN_MIN * 2) + 34);
        assert_eq!(dur.subsec_milliseconds(), 567);
    }

    #[test]
    fn duration_m_s_ms() {
        let dur = super::duration_m_s_ms(1, 23, 456);

        assert_eq!(dur.whole_hours(), 0);
        assert_eq!(dur.whole_minutes(), 1);
        assert_eq!(dur.whole_seconds(), (SEC_IN_MIN * 1) + 23);
        assert_eq!(dur.subsec_milliseconds(), 456);
    }

    #[test]
    fn duration_s_ms() {
        let dur = super::duration_s_ms(12, 345);

        assert_eq!(dur.whole_hours(), 0);
        assert_eq!(dur.whole_minutes(), 0);
        assert_eq!(dur.whole_seconds(), 12);
        assert_eq!(dur.subsec_milliseconds(), 345);
    }

    #[test]
    fn duration_millis() {
        {
            let dur = super::duration_millis(123);

            assert_eq!(dur.whole_hours(), 0);
            assert_eq!(dur.whole_minutes(), 0);
            assert_eq!(dur.whole_seconds(), 0);
            assert_eq!(dur.subsec_milliseconds(), 123);
        }
        {
            // "1:32:42.436", 2023, R4, P1
            let dur = super::duration_millis(5562436);

            assert_eq!(dur.whole_hours(), 1);
            assert_eq!(dur.whole_minutes(), (MIN_IN_HOUR * 1) + 32);
            assert_eq!(dur.whole_seconds(), (SEC_IN_HOUR * 1) + (SEC_IN_MIN * 32) + 42);
            assert_eq!(dur.subsec_milliseconds(), 436);
        }
    }

    #[test]
    fn parse_subsecond_into_milli() {
        let str_value_pairs = vec![("123", 123), ("023", 23), ("12", 120), ("1", 100), ("0", 0), ("000", 0)];

        for (input, expected) in str_value_pairs {
            assert_eq!(super::parse_subsecond_into_milli(input), expected);
        }
    }

    #[test]
    fn parse_subsecond_into_milli_panic() {
        let bad_strings = vec!["0123", "abc", "1.23", ""];

        for bad_str in bad_strings {
            assert!(catch_unwind(|| super::parse_subsecond_into_milli(bad_str)).is_err());
        }
    }

    #[test]
    fn parse_time() {
        let str_value_pairs = vec![
            ("12:00:00Z", time!(12:00:00)),
            ("11:30:00Z", time!(11:30:00)),
            ("15:00:00Z", time!(15:00:00)),
            ("10:30:00Z", time!(10:30:00)),
        ];

        for (input, expected) in str_value_pairs {
            assert_eq!(super::parse_time(input).unwrap(), expected);
        }
    }

    #[test]
    fn parse_time_err() {
        let bad_strings = vec!["12:00:00", "12:00:0Z", "25:00:00Z", "12:00Z"];

        for bad_str in bad_strings {
            assert!(super::parse_time(bad_str).is_err());
        }
    }

    #[test]
    fn parse_duration() {
        let str_value_pairs = vec![
            ("1:22.327", super::duration_m_s_ms(1, 22, 327)),
            ("1:41.269", super::duration_m_s_ms(1, 41, 269)),
            ("59.037", super::duration_m_s_ms(0, 59, 037)),
            ("2:01.341", super::duration_m_s_ms(2, 1, 341)),
            ("10.1", super::duration_m_s_ms(0, 10, 100)),
            ("1:22.327", super::duration_m_s_ms(1, 22, 327)),
            ("33:17.667", super::duration_m_s_ms(33, 17, 667)),
            ("0.4", super::duration_m_s_ms(0, 0, 400)),
            ("1.882", super::duration_m_s_ms(0, 1, 882)),
            ("1:08.436", super::duration_m_s_ms(1, 8, 436)),
            ("40.111", super::duration_m_s_ms(0, 40, 111)),
            ("3:27.071", super::duration_m_s_ms(3, 27, 071)), // 2021, Spa
            // have [hour]
            ("2:02:53.7", super::duration_hms_ms(2, 2, 53, 700)),
            ("1:28:12.058", super::duration_hms_ms(1, 28, 12, 58)),
            ("2:05:05.152", super::duration_hms_ms(2, 5, 5, 152)), // 2011, R7, red flag lap
        ];

        for (input, expected) in str_value_pairs {
            assert_eq!(super::parse_duration(input).unwrap(), expected);
        }
    }

    #[test]
    fn parse_duration_err() {
        let bad_strings = make_bad_duration_strings(&[
            // have > 59m
            "1:60:30.100",
            "2:74:10.7",
            // have > 59s
            "67.769",
            "1:61.100",
            // have '+' prefix
            "+21.217",
            "+1:14.240",
        ]);

        for bad_time_str in bad_strings {
            assert!(super::parse_time(bad_time_str).is_err());
        }
    }

    #[test]
    fn parse_delta() {
        let str_value_pairs = vec![
            ("+0.4", super::duration_millis(400)),
            ("+1.882", super::duration_s_ms(1, 882)),
            ("+21.217", super::duration_s_ms(21, 217)),
            // have > 59s
            ("+103.588", super::duration_s_ms(103, 588)), // 2006, 16, P8
            ("+103.796", super::duration_s_ms(103, 796)), // 2006, 16, P9
            ("+67.769", super::duration_s_ms(67, 769)),   // 2012, 15, P11
            ("+79.692", super::duration_s_ms(79, 692)),   // 2012, 16, P10
            ("+89.241", super::duration_s_ms(89, 241)),   // 2012, 16, P13
            // have [minute]
            ("+1:14.240", super::duration_m_s_ms(1, 14, 240)),
            ("+18:48.66", super::duration_m_s_ms(18, 48, 660)),
        ];

        for (input, expected) in str_value_pairs {
            assert_eq!(super::parse_delta(input).unwrap(), expected);
        }
    }

    #[test]
    fn parse_delta_err() {
        let bad_strings = make_bad_duration_strings(&[
            // have [hour]
            "1:28:12.058",
            "2:02:53.7",
            // don't have + prefix
            "21.217",
        ]);

        for bad_time_str in bad_strings {
            assert!(super::parse_delta(bad_time_str).is_err());
        }
    }

    #[test]
    fn deserialize_optional_time() {
        #[derive(Deserialize)]
        struct Proxy {
            #[serde(default, deserialize_with = "super::deserialize_optional_time")]
            time: Option<Time>,
        }

        assert_eq!(
            serde_json::from_str::<Proxy>(r#"{"time": "11:30:00Z"}"#)
                .unwrap()
                .time
                .unwrap(),
            time!(11:30:00)
        );

        assert!(serde_json::from_str::<Proxy>(r#"{}"#).unwrap().time.is_none());
    }

    #[test]
    fn deserialize_duration() {
        #[derive(Deserialize)]
        struct Proxy {
            #[serde(deserialize_with = "super::deserialize_duration")]
            duration: Duration,
        }

        assert_eq!(
            serde_json::from_str::<Proxy>(r#"{"duration": "1:23.456"}"#)
                .unwrap()
                .duration,
            super::duration_m_s_ms(1, 23, 456)
        );
    }

    #[test]
    fn date_time_deserialize() {
        let dt: DateTime = serde_json::from_str(
            r#"{
                "date": "2021-08-27"}"#,
        )
        .unwrap();

        assert_eq!(dt.date, date!(2021 - 08 - 27));
        assert!(dt.time.is_none());

        let dt: DateTime = serde_json::from_str(
            r#"{
                "date": "2022-04-22",
                "time": "11:30:00Z"}"#,
        )
        .unwrap();

        assert_eq!(dt.date, date!(2022 - 04 - 22));
        assert!(dt.time.is_some());
        assert_eq!(dt.time.unwrap(), time!(11:30:00));
    }

    #[test]
    fn qualifying_time() {
        let quali = QualifyingTime::Time(super::duration_m_s_ms(1, 23, 456));

        assert!(matches!(quali, QualifyingTime::Time(_)));
        assert!(quali.has_time());
        assert!(!quali.no_time_set());

        let cloned_lap_time = quali.time().clone();

        if let QualifyingTime::Time(lap_time) = quali {
            assert_eq!(lap_time, cloned_lap_time);
            assert_eq!(lap_time, super::duration_m_s_ms(1, 23, 456));
        }
    }

    #[test]
    #[should_panic]
    fn qualifying_time_time_panics() {
        let quali = QualifyingTime::NoTimeSet;

        assert!(matches!(quali, QualifyingTime::NoTimeSet));
        assert!(!quali.has_time());
        assert!(quali.no_time_set());

        let _ = quali.time();
    }

    #[test]
    fn qualifying_time_deserialize() {
        {
            let quali = serde_json::from_str::<QualifyingTime>(r#""1:23.456""#).unwrap();
            assert!(quali.has_time());
            assert!(!quali.no_time_set());
            assert_eq!(quali.time(), &super::duration_m_s_ms(1, 23, 456));
        }

        {
            let quali = serde_json::from_str::<QualifyingTime>(r#""""#).unwrap();
            assert!(!quali.has_time());
            assert!(quali.no_time_set());
            assert!(matches!(quali, QualifyingTime::NoTimeSet));
        }
    }

    #[test]
    fn qualifying_time_deserialize_err() {
        assert!(serde_json::from_str::<QualifyingTime>("1").is_err());
    }

    #[test]
    fn race_time() {
        let p1 = RaceTime::lead(super::duration_millis(5562436));
        assert!(p1.is_lead());
        assert_eq!(p1.total(), &super::duration_hms_ms(1, 32, 42, 436));
        assert_eq!(p1.delta(), &Duration::ZERO);

        let p2 = RaceTime::with_delta(super::duration_millis(5564573), super::duration_m_s_ms(0, 2, 137));
        assert!(!p2.is_lead());
        assert_eq!(p2.total(), &super::duration_hms_ms(1, 32, 42 + 2, 436 + 137));
        assert_eq!(p2.delta(), &super::duration_m_s_ms(0, 2, 137));

        assert_eq!(p2.total().clone() - p1.total().clone(), p2.delta().clone());

        assert_eq!(p1, *RACE_TIME_2023_4_P1);
        assert_eq!(p2, *RACE_TIME_2023_4_P2);
    }

    #[test]
    fn race_time_deserialize() {
        let str_value_pairs = vec![
            (r#"{"millis": "7373700", "time": "2:02:53.7"}"#, RACE_TIME_1950_4_P1.clone()),
            (r#"{"millis": "7374100", "time": "+0.4"}"#, RACE_TIME_1950_4_P2.clone()),
            (r#"{"millis": "5562436", "time": "1:32:42.436"}"#, RACE_TIME_2023_4_P1.clone()),
            (r#"{"millis": "5564573", "time": "+2.137"}"#, RACE_TIME_2023_4_P2.clone()),
        ];

        for (input, expected) in str_value_pairs.iter() {
            assert_eq!(&serde_json::from_str::<RaceTime>(input).unwrap(), expected);
        }
    }

    #[test]
    fn race_time_deserialize_assets() {
        let deserialize_and_assert_eq = |race_time_strings: &[&str], race_times: &[RaceTime]| {
            let deserialized_race_times: Vec<_> = race_time_strings
                .iter()
                .map(|race_time_str| serde_json::from_str::<RaceTime>(race_time_str).unwrap())
                .collect();

            assert!(!deserialized_race_times.is_empty());
            assert_eq!(deserialized_race_times.len(), race_times.len());

            for (des_race_time, ref_race_time) in deserialized_race_times.iter().zip(race_times.iter()) {
                assert_eq!(des_race_time, ref_race_time);
            }
        };

        deserialize_and_assert_eq(&RACE_TIMES_1950_4_STR[..], &RACE_TIMES_1950_4[..]);
        deserialize_and_assert_eq(&RACE_TIMES_2003_4_STR[..], &RACE_TIMES_2003_4[..]);
        deserialize_and_assert_eq(&RACE_TIMES_2021_12_STR[..], &RACE_TIMES_2021_12[..]);
        deserialize_and_assert_eq(&RACE_TIMES_2023_4_STR[..], &RACE_TIMES_2023_4[..]);
    }

    #[test]
    fn race_time_validate_assets() {
        let validate_race_times = |race_times: &[RaceTime]| {
            assert!(race_times.len() >= 2);

            let lead = race_times.first().unwrap();
            let others = &race_times[1..];

            assert!(lead.is_lead());
            assert_eq!(lead.delta(), &Duration::ZERO);

            for other in others.iter() {
                assert!(!other.is_lead());
                assert!(other.delta() > &Duration::ZERO);
                assert!(other.total().clone() > lead.total().clone());
                assert_eq!(other.total().clone() - lead.total().clone(), other.delta().clone());
            }
        };

        validate_race_times(&RACE_TIMES_1950_4[..]);
        validate_race_times(&RACE_TIMES_2003_4[..]);
        validate_race_times(&RACE_TIMES_2021_12[..]);
        validate_race_times(&RACE_TIMES_2023_4[..]);
    }
}
