use std::str::FromStr;

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

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum Table {
    SeasonTable(SeasonTable),
    DriverTable(DriverTable),
    ConstructorTable(ConstructorTable),
    CircuitTable(CircuitTable),
    RaceTable(RaceTable),
    StatusTable(StatusTable),
}
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct SeasonTable {
    #[serde(rename = "Seasons")]
    pub seasons: Vec<Season>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct DriverTable {
    #[serde(rename = "Drivers")]
    pub drivers: Vec<Driver>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct ConstructorTable {
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructor>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct CircuitTable {
    #[serde(rename = "Circuits")]
    pub circuits: Vec<Circuit>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct RaceTable {
    #[serde(rename = "Races")]
    pub races: Vec<Race>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct StatusTable {
    #[serde(rename = "Status")]
    pub status: Vec<Status>,
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
    #[serde(rename = "QualifyingResults")]
    pub qualifying_results: Option<Vec<QualifyingResult>>,
    #[serde(rename = "SprintResults")]
    pub sprint_results: Option<Vec<SprintResult>>,
    #[serde(rename = "Results")]
    pub results: Option<Vec<RaceResult>>,
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
    pub position_text: String,
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
    pub position_text: String,
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
        let season_table: SeasonTable = serde_json::from_str(SEASON_TABLE_STR).unwrap();

        assert!(!season_table.seasons.is_empty());
        assert_eq!(&season_table.seasons, &SEASON_TABLE.seasons);
    }

    #[test]
    fn driver_table() {
        let driver_table: DriverTable = serde_json::from_str(DRIVER_TABLE_STR).unwrap();

        assert!(!driver_table.drivers.is_empty());
        assert_eq!(&driver_table.drivers, &DRIVER_TABLE.drivers);
    }

    #[test]
    fn constructor_table() {
        let constructor_table: ConstructorTable = serde_json::from_str(CONSTRUCTOR_TABLE_STR).unwrap();

        assert!(!constructor_table.constructors.is_empty());
        assert_eq!(&constructor_table.constructors, &CONSTRUCTOR_TABLE.constructors);
    }

    #[test]
    fn circuit_table() {
        let circuit_table: CircuitTable = serde_json::from_str(CIRCUIT_TABLE_STR).unwrap();

        assert!(!circuit_table.circuits.is_empty());
        assert_eq!(&circuit_table.circuits, &CIRCUIT_TABLE.circuits);
    }

    #[test]
    fn race_table_schedule() {
        let race_table: RaceTable = serde_json::from_str(RACE_TABLE_SCHEDULE_STR).unwrap();

        assert!(!race_table.races.is_empty());
        assert_eq!(&race_table.races, &RACE_TABLE_SCHEDULE.races);
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

            assert!(race.qualifying_results.is_some());
            assert!(!race.qualifying_results.as_ref().unwrap().is_empty());
            assert_eq!(race, *RACE_2003_4_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_4_QUALIFYING_RESULTS_STR).unwrap();

            assert!(race.qualifying_results.is_some());
            assert!(!race.qualifying_results.as_ref().unwrap().is_empty());
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

        assert!(race.sprint_results.is_some());
        assert!(!race.sprint_results.as_ref().unwrap().is_empty());
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

            assert!(race.results.is_some());
            assert!(!race.results.as_ref().unwrap().is_empty());
            assert_eq!(race, *RACE_2003_4_RACE_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_4_RACE_RESULTS_STR).unwrap();

            assert!(race.results.is_some());
            assert!(!race.results.as_ref().unwrap().is_empty());
            assert_eq!(race, *RACE_2023_4_RACE_RESULTS);
        }
    }

    #[test]
    fn finishing_status() {
        let status_table: StatusTable = serde_json::from_str(STATUS_TABLE_2022_STR).unwrap();

        assert!(!status_table.status.is_empty());
        assert_eq!(&status_table.status, &STATUS_TABLE_2022.status);
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
