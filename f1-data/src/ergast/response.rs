use std::str::FromStr;

use serde::{Deserialize, Deserializer};
use serde_with::{serde_as, DisplayFromStr};
use url::Url;
use void::Void;

use crate::ergast::time::{Duration, Time};

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
    #[serde_as(as = "DisplayFromStr")]
    pub limit: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub offset: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub total: u32,
    #[serde(rename = "SeasonTable")]
    pub season_table: Option<SeasonTable>,
    #[serde(rename = "DriverTable")]
    pub driver_table: Option<DriverTable>,
    #[serde(rename = "ConstructorTable")]
    pub constructor_table: Option<ConstructorTable>,
    #[serde(rename = "CircuitTable")]
    pub circuit_table: Option<CircuitTable>,
    #[serde(rename = "RaceTable")]
    pub race_table: Option<RaceTable>,
    #[serde(rename = "StatusTAble")]
    pub status_table: Option<StatusTable>,
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
    pub date_of_birth: time::Date,
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
    pub date: time::Date,
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
    pub date: time::Date,
    pub time: Option<Time>,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct RaceTime {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub millis: Option<u32>,
    pub time: String,
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

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub enum SpeedUnits {
    #[serde(rename = "kph")]
    Kph,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AverageSpeed {
    pub units: SpeedUnits,
    #[serde_as(as = "DisplayFromStr")]
    pub speed: f32,
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

    pub fn parse(time: &str) -> Self {
        if !time.is_empty() {
            <Self as From<LapTime>>::from(LapTime::parse(time).unwrap())
        } else {
            QualifyingTime::NoTimeSet
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
    fn from(lap: LapTime) -> QualifyingTime {
        QualifyingTime::Time(lap)
    }
}

impl FromStr for QualifyingTime {
    type Err = Void;

    fn from_str(time: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse(time))
    }
}

#[cfg(test)]
mod tests {
    use time::macros::date;

    use crate::ergast::time::macros::time;

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
    fn qualifying_time_from_lap_time() {
        let quali = QualifyingTime::from(LapTime::from_m_s_ms(1, 23, 456));

        assert!(matches!(quali, QualifyingTime::Time(_)));
        assert!(quali.has_time());
        assert!(!quali.no_time_set());

        let cloned_lap = quali.time().clone();

        if let QualifyingTime::Time(lap) = quali {
            assert_eq!(lap, cloned_lap);
            assert_eq!(lap, LapTime::from_m_s_ms(1, 23, 456));
        }
    }

    #[test]
    fn qualifying_time_from_str() {
        {
            let quali = QualifyingTime::from_str("1:23.456").unwrap();
            assert!(quali.has_time());
            assert!(!quali.no_time_set());
            assert_eq!(quali.time(), &LapTime::from_m_s_ms(1, 23, 456));
        }

        {
            let quali = QualifyingTime::from_str("").unwrap();
            assert!(!quali.has_time());
            assert!(quali.no_time_set());
            assert!(matches!(quali, QualifyingTime::NoTimeSet));
        }
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
}
