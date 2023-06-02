use std::str::FromStr;

use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr};
use url::Url;

use crate::ergast::time::{Date, Duration, ParseError, Time};

pub const GRID_PIT_LANE: u32 = 0;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Response {
    #[serde(rename = "MRData")]
    pub mr_data: MrData,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct MrData {
    pub xmlns: String,
    pub series: String,
    pub url: Url,
    #[serde(flatten)]
    pub pagination: Pagination,
    #[serde(flatten)]
    pub table: Table,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Pagination {
    #[serde_as(as = "DisplayFromStr")]
    pub limit: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub offset: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub total: u32,
}

impl Pagination {
    pub fn is_last_page(&self) -> bool {
        (self.offset + self.limit) >= self.total
    }

    pub fn next_page(&self) -> Option<Self> {
        if self.is_last_page() {
            None
        } else {
            Some(Self {
                offset: self.offset + self.limit,
                ..*self
            })
        }
    }
}

/// [`Table`] represents all the possible different lists of data that may be returned in a
/// [`Response`] from the Ergast API, e.g. [`Table::Seasons`] corresponds to the the `"SeasonTable"`
/// property key in the JSON response, containing a list of [`Season`]s which corresponds to the
/// `"Seasons"` property key. One and only of these tables may be returned in a given response,
/// which is represented by the different variants of this enum.
///
/// The variants and inner fields may be matched and accessed via the usual pattern matching, or
/// via accessor functions provided by [`enum-as-inner`](https://crates.io/crates/enum-as-inner).
///
/// # Examples:
///
/// ```
/// # use url::Url;
/// use f1_data::ergast::response::{Season, Table};
///
/// let table = Table::Seasons {
///     seasons: vec![Season {
///         season: 2022,
///         url: Url::parse("http://empty.org").unwrap(),
///     }],
/// };
///
/// let Table::Seasons { ref seasons } = table else { panic!("Expected Seasons variant") };
/// assert_eq!(seasons[0].season, 2022);
///
/// assert_eq!(table.as_seasons().unwrap()[0].season, 2022);
/// ```
#[derive(Deserialize, EnumAsInner, PartialEq, Clone, Debug)]
pub enum Table {
    /// Contains a list of [`Season`]s, and corresponds to the `"SeasonTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "SeasonTable")]
    Seasons {
        /// List of [`Season`]s, corresponding to the `"Seasons"` property key in the JSON response.
        #[serde(rename = "Seasons")]
        seasons: Vec<Season>,
    },
    /// Contains a list of [`Driver`]s, and corresponds to the `"DriverTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "DriverTable")]
    Drivers {
        /// List of [`Driver`]s, corresponding to the `"Drivers"` property key in the JSON response.
        #[serde(rename = "Drivers")]
        drivers: Vec<Driver>,
    },
    /// Contains a list of [`Constructor`]s, and corresponds to the `"ConstructorTable"` property
    /// key in the JSON response from the Ergast API.
    #[serde(rename = "ConstructorTable")]
    Constructors {
        /// List of [`Constructor`]s, corresponding to the `"Constructors"` property key in the JSON
        /// response.
        #[serde(rename = "Constructors")]
        constructors: Vec<Constructor>,
    },
    /// Contains a list of [`Circuit`]s, and corresponds to the `"CircuitTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "CircuitTable")]
    Circuits {
        /// List of [`Circuit`]s, corresponding to the `"Circuits"` property key in the JSON
        /// response.
        #[serde(rename = "Circuits")]
        circuits: Vec<Circuit>,
    },
    /// Contains a list of [`Race`]s, and corresponds to the `"RaceTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "RaceTable")]
    Races {
        /// List of [`Race`]s, corresponding to the `"Races"` property key in the JSON response.
        #[serde(rename = "Races")]
        races: Vec<Race>,
    },
    /// Contains a list of [`Status`]es, and corresponds to the `"StatusTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "StatusTable")]
    Status {
        /// List of [`Status`]es, corresponding to the `"Status"` property key in the JSON response.
        #[serde(rename = "Status")]
        status: Vec<Status>,
    },
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Season {
    #[serde_as(as = "DisplayFromStr")]
    pub season: u32,
    pub url: Url,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub driver_id: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub permanent_number: Option<u32>,
    pub code: Option<String>,
    pub url: Url,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: Date,
    pub nationality: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    pub constructor_id: String,
    pub url: Url,
    pub name: String,
    pub nationality: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub circuit_id: String,
    pub url: Url,
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    #[serde_as(as = "DisplayFromStr")]
    pub season: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub round: u32,
    pub url: Url,
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: Date,
    pub time: Option<Time>,
    #[serde(flatten)]
    pub schedule: Schedule,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Schedule {
    #[serde(rename = "FirstPractice")]
    pub first_practice: Option<DateTime>,
    #[serde(rename = "SecondPractice")]
    pub second_practice: Option<DateTime>,
    #[serde(rename = "ThirdPractice")]
    pub third_practice: Option<DateTime>,
    #[serde(rename = "Qualifying")]
    pub qualifying: Option<DateTime>,
    #[serde(rename = "Sprint")]
    pub sprint: Option<DateTime>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Payload {
    QualifyingResults(Vec<QualifyingResult>),
    SprintResults(Vec<SprintResult>),
    RaceResults(Vec<RaceResult>),
    /// Only one kind of payload may be returned in a response, but it may also contain no payload,
    /// e.g. when [`Resource::RaceSchedule`](crate::ergast::resource::Resource::RaceSchedule) is
    /// requested. While that could be handled via `Option<Payload>`, it's more ergonomic to handle
    /// it with this [`Payload::NoPayload`] variant. The [`Payload::has_payload`] method is provided
    /// to query the presence of a payload, i.e. any variant that is not [`Payload::NoPayload`].
    NoPayload,
}

impl Payload {
    /// Returns true if any of the payload variants is held, i.e. any variant that in't
    /// [`Payload::NoPayload`].
    pub fn has_payload(&self) -> bool {
        !matches!(self, Self::NoPayload)
    }
}

impl From<PayloadProxy> for Payload {
    fn from(proxy: PayloadProxy) -> Self {
        type Proxy = PayloadProxy;

        match proxy {
            Proxy::QualifyingResults(qr) => Self::QualifyingResults(qr),
            Proxy::SprintResults(sr) => Self::SprintResults(sr),
            Proxy::RaceResults(rr) => Self::RaceResults(rr),
        }
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Deserialize, PartialEq, Clone, Debug)]
enum PayloadProxy {
    QualifyingResults(Vec<QualifyingResult>),
    SprintResults(Vec<SprintResult>),
    #[serde(rename = "Results")]
    RaceResults(Vec<RaceResult>),
}

impl<'de> Deserialize<'de> for Payload {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if let Some(proxy) = Option::<PayloadProxy>::deserialize(deserializer)? {
            Ok(Payload::from(proxy))
        } else {
            Ok(Payload::NoPayload)
        }
    }
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct QualifyingResult {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde(rename = "Q1")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub q1: Option<QualifyingTime>,
    #[serde(rename = "Q2")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub q2: Option<QualifyingTime>,
    #[serde(rename = "Q3")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub q3: Option<QualifyingTime>,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SprintResult {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    pub position_text: Position,
    #[serde_as(as = "DisplayFromStr")]
    pub points: u32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde_as(as = "DisplayFromStr")]
    pub grid: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub laps: u32,
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<RaceTime>,
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RaceResult {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    pub position_text: Position,
    #[serde_as(as = "DisplayFromStr")]
    pub points: u32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde_as(as = "DisplayFromStr")]
    pub grid: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub laps: u32,
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<RaceTime>,
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Position {
    Finished(u32),
    Retired,
    Disqualified,
    Excluded,
    Withdrawn,
    FailedToQualify,
    NotClassified,
}

impl Position {
    pub const R: Position = Position::Retired;
    pub const D: Position = Position::Disqualified;
    pub const E: Position = Position::Excluded;
    pub const W: Position = Position::Withdrawn;
    pub const F: Position = Position::FailedToQualify;
    pub const N: Position = Position::NotClassified;
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            "R" => Ok(Self::R),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            "F" => Ok(Self::F),
            "N" => Ok(Self::N),
            num => Ok(Self::Finished(
                num.parse::<u32>()
                    .map_err(|err| serde::de::Error::custom(err.to_string()))?,
            )),
        }
    }
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde_as(as = "DisplayFromStr")]
    pub status_id: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub count: u32,
    pub status: String,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Location {
    #[serde_as(as = "DisplayFromStr")]
    pub lat: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub long: f64,
    pub locality: String,
    pub country: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct DateTime {
    pub date: Date,
    pub time: Option<Time>,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct FastestLap {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub rank: Option<u32>,
    #[serde_as(as = "DisplayFromStr")]
    pub lap: u32,
    #[serde(rename = "Time", deserialize_with = "extract_nested_lap_time")]
    pub time: LapTime,
    #[serde(rename = "AverageSpeed")]
    pub average_speed: Option<AverageSpeed>,
}

fn extract_nested_lap_time<'de, D: Deserializer<'de>>(deserializer: D) -> Result<LapTime, D::Error> {
    #[derive(Deserialize)]
    struct Time {
        time: LapTime,
    }
    Ok(Time::deserialize(deserializer)?.time)
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AverageSpeed {
    pub units: SpeedUnits,
    #[serde_as(as = "DisplayFromStr")]
    pub speed: f32,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum SpeedUnits {
    #[serde(rename = "kph")]
    Kph,
}

pub type LapTime = Duration;

#[derive(PartialEq, Clone, Debug)]
pub enum QualifyingTime {
    Time(LapTime),
    NoTimeSet,
}

impl QualifyingTime {
    pub fn from_m_s_ms(minutes: i64, seconds: i64, milliseconds: i64) -> Self {
        Self::Time(LapTime::from_m_s_ms(minutes, seconds, milliseconds))
    }

    pub fn parse(t_str: &str) -> Result<Self, ParseError> {
        if !t_str.is_empty() {
            LapTime::parse(t_str).map(QualifyingTime::Time)
        } else {
            Ok(QualifyingTime::NoTimeSet)
        }
    }

    pub fn has_time(&self) -> bool {
        matches!(self, Self::Time(_))
    }

    pub fn no_time_set(&self) -> bool {
        matches!(self, Self::NoTimeSet)
    }

    pub fn time(&self) -> &LapTime {
        match &self {
            Self::Time(time) => time,
            _ => panic!("Cannot get time of NoTimeSet"),
        }
    }
}

impl From<LapTime> for QualifyingTime {
    fn from(lap_time: LapTime) -> QualifyingTime {
        QualifyingTime::Time(lap_time)
    }
}

impl FromStr for QualifyingTime {
    type Err = ParseError;

    fn from_str(t_str: &str) -> Result<Self, Self::Err> {
        Self::parse(t_str)
    }
}

#[serde_as]
#[derive(Deserialize, Debug)]
struct RaceTimeProxy {
    #[serde_as(as = "DisplayFromStr")]
    pub millis: u32,
    pub time: String,
}

#[derive(PartialEq, Clone, Debug)]
pub struct RaceTime {
    total: Duration,
    delta: Duration,
}

impl RaceTime {
    pub fn lead(total: Duration) -> RaceTime {
        RaceTime {
            total,
            delta: Duration::ZERO,
        }
    }

    pub fn with_delta(total: Duration, delta: Duration) -> RaceTime {
        assert!(delta < total);

        RaceTime { total, delta }
    }

    pub fn is_lead(&self) -> bool {
        self.delta == Duration::ZERO
    }

    pub fn total(&self) -> &Duration {
        &self.total
    }

    pub fn delta(&self) -> &Duration {
        &self.delta
    }

    fn parse_from_proxy(proxy: &RaceTimeProxy) -> Result<Self, ParseError> {
        if proxy.time.is_empty() {
            return Err(ParseError::InvalidRaceTime("Unexpected empty 'time'".to_string()));
        }

        let has_delta = proxy.time.starts_with('+');

        let total = Duration::milliseconds(proxy.millis as i64);
        let delta = Duration::parse(if has_delta { &proxy.time[1..] } else { &proxy.time })?;

        if !has_delta && (total != delta) {
            return Err(ParseError::InvalidRaceTime(format!("Non-delta 'time' must match 'millis': {:?}", proxy)));
        }

        if delta > total {
            return Err(ParseError::InvalidRaceTime(format!("Delta 'time' must be less than 'millis': {:?}", proxy)));
        }

        Ok(if has_delta {
            Self::with_delta(total, delta)
        } else {
            Self::lead(total)
        })
    }
}

impl TryFrom<RaceTimeProxy> for RaceTime {
    type Error = ParseError;

    fn try_from(proxy: RaceTimeProxy) -> Result<Self, Self::Error> {
        RaceTime::parse_from_proxy(&proxy)
    }
}

impl<'de> Deserialize<'de> for RaceTime {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RaceTime::parse_from_proxy(&RaceTimeProxy::deserialize(deserializer)?)
            .map_err(|err| serde::de::Error::custom(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::ergast::time::macros::{date, time};

    use super::*;
    use crate::ergast::tests::*;

    #[test]
    fn season_table() {
        let table: Table = serde_json::from_str(SEASON_TABLE_STR).unwrap();
        assert!(!table.as_seasons().unwrap().is_empty());
        assert_eq!(table, *SEASON_TABLE);
    }

    #[test]
    fn driver_table() {
        let table: Table = serde_json::from_str(DRIVER_TABLE_STR).unwrap();
        assert!(!table.as_drivers().unwrap().is_empty());
        assert_eq!(table, *DRIVER_TABLE);
    }

    #[test]
    fn constructor_table() {
        let table: Table = serde_json::from_str(CONSTRUCTOR_TABLE_STR).unwrap();
        assert!(!table.as_constructors().unwrap().is_empty());
        assert_eq!(table, *CONSTRUCTOR_TABLE);
    }

    #[test]
    fn circuit_table() {
        let table: Table = serde_json::from_str(CIRCUIT_TABLE_STR).unwrap();
        assert!(!table.as_circuits().unwrap().is_empty());
        assert_eq!(table, *CIRCUIT_TABLE);
    }

    #[test]
    fn race_table_schedule() {
        let table: Table = serde_json::from_str(RACE_TABLE_SCHEDULE_STR).unwrap();
        assert!(!table.as_races().unwrap().is_empty());
        assert_eq!(table, *RACE_TABLE_SCHEDULE);
    }

    #[test]
    fn qualifying_result() {
        let from_str = |result_str| serde_json::from_str::<QualifyingResult>(result_str).unwrap();

        assert_eq!(from_str(QUALIFYING_RESULT_2003_4_P1_STR), *QUALIFYING_RESULT_2003_4_P1);
        assert_eq!(from_str(QUALIFYING_RESULT_2003_4_P2_STR), *QUALIFYING_RESULT_2003_4_P2);
        assert_eq!(from_str(QUALIFYING_RESULT_2003_4_P20_STR), *QUALIFYING_RESULT_2003_4_P20);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_4_P1_STR), *QUALIFYING_RESULT_2023_4_P1);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_4_P2_STR), *QUALIFYING_RESULT_2023_4_P2);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_4_P3_STR), *QUALIFYING_RESULT_2023_4_P3);
    }

    #[test]
    fn qualifying_results() {
        {
            let race: Race = serde_json::from_str(RACE_2003_4_QUALIFYING_RESULTS_STR).unwrap();

            let Payload::QualifyingResults(qualifying_results) = &race.payload else {
                panic!("Expected QualifyingResults variant")
            };

            assert!(!qualifying_results.is_empty());

            assert_eq!(race, *RACE_2003_4_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_4_QUALIFYING_RESULTS_STR).unwrap();

            let Payload::QualifyingResults(qualifying_results) = &race.payload else {
                panic!("Expected QualifyingResults variant")
            };

            assert!(!qualifying_results.is_empty());

            assert_eq!(race, *RACE_2023_4_QUALIFYING_RESULTS);
        }
    }

    #[test]
    fn sprint_result() {
        let from_str = |result_str| serde_json::from_str::<SprintResult>(result_str).unwrap();

        assert_eq!(from_str(SPRINT_RESULT_2023_4_P1_STR), *SPRINT_RESULT_2023_4_P1);
    }

    #[test]
    fn sprint_results() {
        let race: Race = serde_json::from_str(RACE_2023_4_SPRINT_RESULTS_STR).unwrap();

        let Payload::SprintResults(sprint_results) = &race.payload else {
            panic!("Expected SprintResults variant")
        };

        assert!(!sprint_results.is_empty());

        assert_eq!(race, *RACE_2023_4_SPRINT_RESULTS);
    }

    #[test]
    fn race_result() {
        let from_str = |result_str| serde_json::from_str::<RaceResult>(result_str).unwrap();

        assert_eq!(from_str(RACE_RESULT_2003_4_P1_STR), *RACE_RESULT_2003_4_P1);
        assert_eq!(from_str(RACE_RESULT_2003_4_P2_STR), *RACE_RESULT_2003_4_P2);
        assert_eq!(from_str(RACE_RESULT_2003_4_P19_STR), *RACE_RESULT_2003_4_P19);

        assert_eq!(from_str(RACE_RESULT_2023_4_P1_STR), *RACE_RESULT_2023_4_P1);
        assert_eq!(from_str(RACE_RESULT_2023_4_P2_STR), *RACE_RESULT_2023_4_P2);
        assert_eq!(from_str(RACE_RESULT_2023_4_P20_STR), *RACE_RESULT_2023_4_P20);
    }

    #[test]
    fn race_results() {
        {
            let race: Race = serde_json::from_str(RACE_2003_4_RACE_RESULTS_STR).unwrap();

            let Payload::RaceResults(race_results) = &race.payload else {
                panic!("Expected RaceResults variant")
            };

            assert!(!race_results.is_empty());

            assert_eq!(race, *RACE_2003_4_RACE_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_4_RACE_RESULTS_STR).unwrap();

            let Payload::RaceResults(race_results) = &race.payload else {
                panic!("Expected RaceResults variant")
            };

            assert!(!race_results.is_empty());

            assert_eq!(race, *RACE_2023_4_RACE_RESULTS);
        }
    }

    #[test]
    fn finishing_status() {
        let table: Table = serde_json::from_str(STATUS_TABLE_2022_STR).unwrap();
        assert!(!table.as_status().unwrap().is_empty());
        assert_eq!(table, *STATUS_TABLE_2022);
    }

    #[test]
    fn pagination_is_last_page() {
        assert!(Pagination {
            limit: 30,
            offset: 0,
            total: 16
        }
        .is_last_page());

        assert!(Pagination {
            limit: 10,
            offset: 5,
            total: 15
        }
        .is_last_page());

        assert!(!Pagination {
            limit: 10,
            offset: 4,
            total: 15
        }
        .is_last_page());
    }

    #[test]
    fn pagination_next_page() {
        assert_eq!(
            Pagination {
                limit: 10,
                offset: 0,
                total: 15
            }
            .next_page()
            .unwrap(),
            Pagination {
                limit: 10,
                offset: 10,
                total: 15
            }
        );

        assert!(Pagination {
            limit: 10,
            offset: 10,
            total: 15
        }
        .next_page()
        .is_none());
    }

    #[test]
    fn pagination_deserialize() {
        const REF_PAGINATION: Pagination = Pagination {
            limit: 30,
            offset: 0,
            total: 16,
        };

        assert_eq!(
            serde_json::from_str::<Pagination>(
                r#"{
                "limit": "30",
                "offset": "0",
                "total": "16"
              }"#
            )
            .unwrap(),
            REF_PAGINATION
        );

        assert_eq!(
            serde_json::from_str::<MrData>(
                r#"{
                "xmlns": "http://ergast.com/mrd/1.5",
                "series": "f1",
                "url": "http://ergast.com/api/f1/races.json",
                "limit": "30",
                "offset": "0",
                "total": "16",
                "RaceTable": { "Races": [] }
              }"#
            )
            .unwrap()
            .pagination,
            REF_PAGINATION
        );
    }

    #[test]
    fn position() {
        assert_eq!(Position::Retired, Position::R);
        assert_ne!(Position::Retired, Position::E);

        let pos = Position::Finished(10);
        assert!(matches!(pos, Position::Finished(_)));
        assert!(!matches!(pos, Position::E));

        match pos {
            Position::Finished(pos) => assert_eq!(pos, 10),
            _ => panic!("Expected Finished variant"),
        };
    }

    #[test]
    fn position_deserialize() {
        assert!(matches!(serde_json::from_str::<Position>("\"R\"").unwrap(), Position::R));
        assert!(matches!(serde_json::from_str::<Position>("\"D\"").unwrap(), Position::D));
        assert!(matches!(serde_json::from_str::<Position>("\"E\"").unwrap(), Position::E));
        assert!(matches!(serde_json::from_str::<Position>("\"W\"").unwrap(), Position::W));
        assert!(matches!(serde_json::from_str::<Position>("\"F\"").unwrap(), Position::F));
        assert!(matches!(serde_json::from_str::<Position>("\"N\"").unwrap(), Position::N));

        let Position::Finished(pos) = serde_json::from_str("\"10\"").unwrap() else {
            panic!("Expected Finished variant")
        };
        assert_eq!(pos, 10);

        assert!(serde_json::from_str::<Position>("\"unknown\"").is_err());
    }

    #[test]
    fn date_time() {
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
    fn qualifying_time_from_lap_time() {
        let quali = QualifyingTime::from(LapTime::from_m_s_ms(1, 23, 456));

        assert!(matches!(quali, QualifyingTime::Time(_)));
        assert!(quali.has_time());
        assert!(!quali.no_time_set());

        let cloned_lap_time = quali.time().clone();

        if let QualifyingTime::Time(lap_time) = quali {
            assert_eq!(lap_time, cloned_lap_time);
            assert_eq!(lap_time, LapTime::from_m_s_ms(1, 23, 456));
        }
    }

    #[test]
    fn qualifying_time_parse() {
        {
            let quali = QualifyingTime::parse("1:23.456").unwrap();
            assert!(quali.has_time());
            assert!(!quali.no_time_set());
            assert_eq!(quali.time(), &LapTime::from_m_s_ms(1, 23, 456));
        }

        {
            let quali = QualifyingTime::parse("").unwrap();
            assert!(!quali.has_time());
            assert!(quali.no_time_set());
            assert!(matches!(quali, QualifyingTime::NoTimeSet));
        }
    }

    #[test]
    fn qualifying_time_parse_err() {
        assert!(QualifyingTime::parse("1").is_err());
    }

    #[test]
    #[should_panic]
    fn qualifying_time_time_panics() {
        let quali = QualifyingTime::NoTimeSet;

        assert!(matches!(quali, QualifyingTime::NoTimeSet));
        assert!(!quali.has_time());
        assert!(quali.no_time_set());

        quali.time();
    }

    #[test]
    fn race_time_construction() {
        let p1 = RaceTime::lead(Duration::milliseconds(5562436));
        assert!(p1.is_lead());
        assert_eq!(p1.total(), &Duration::from_hms_ms(1, 32, 42, 436));
        assert_eq!(p1.delta(), &Duration::ZERO);

        let p2 = RaceTime::with_delta(Duration::milliseconds(5564573), Duration::from_m_s_ms(0, 2, 137));
        assert!(!p2.is_lead());
        assert_eq!(p2.total(), &Duration::from_hms_ms(1, 32, 42 + 2, 436 + 137));
        assert_eq!(p2.delta(), &Duration::from_m_s_ms(0, 2, 137));

        assert_eq!(p2.total().clone() - p1.total().clone(), p2.delta().clone());

        assert_eq!(p1, *RACE_TIME_2023_4_P1);
        assert_eq!(p2, *RACE_TIME_2023_4_P2);
    }

    #[test]
    fn race_time_parse_from_proxy() {
        let proxy_race_time_pairs = vec![
            (
                RaceTimeProxy {
                    millis: 7373700,
                    time: "2:02:53.7".to_string(),
                },
                RACE_TIME_1950_4_P1.clone(),
            ),
            (
                RaceTimeProxy {
                    millis: 7374100,
                    time: "+0.4".to_string(),
                },
                RACE_TIME_1950_4_P2.clone(),
            ),
            (
                RaceTimeProxy {
                    millis: 5562436,
                    time: "1:32:42.436".to_string(),
                },
                RACE_TIME_2023_4_P1.clone(),
            ),
            (
                RaceTimeProxy {
                    millis: 5564573,
                    time: "+2.137".to_string(),
                },
                RACE_TIME_2023_4_P2.clone(),
            ),
        ];

        for (proxy, race_time) in proxy_race_time_pairs.iter() {
            assert_eq!(RaceTime::parse_from_proxy(&proxy).unwrap(), race_time.clone());
        }
    }

    #[test]
    fn race_time_deserialize() {
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
        validate_race_times(&RACE_TIMES_2023_4[..]);
    }
}
