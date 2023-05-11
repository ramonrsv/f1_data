use std::str::FromStr;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use time::Duration;
use url::Url;
use void::Void;

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
    pub date_of_birth: String,
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
    pub date: String,
    pub time: Option<String>,
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
    pub q1: Option<LapTime>,
    #[serde(rename = "Q2")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub q2: Option<LapTime>,
    #[serde(rename = "Q3")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub q3: Option<LapTime>,
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
    pub time: Option<Time>,
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
    pub time: Option<Time>,
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

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Location {
    pub lat: String,
    pub long: String,
    pub locality: String,
    pub country: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct DateTime {
    pub date: String,
    pub time: Option<String>,
}

#[derive(PartialEq, Clone, Debug)]
pub enum LapTime {
    LapTime(Duration),
    NoTimeSet,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Time {
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
    #[serde(rename = "Time")]
    pub time: Time,
    #[serde(rename = "AverageSpeed")]
    pub average_speed: Option<AverageSpeed>,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AverageSpeed {
    pub units: String,
    #[serde_as(as = "DisplayFromStr")]
    pub speed: f32,
}

impl LapTime {
    pub fn from(minutes: i64, seconds: i64, milliseconds: i64) -> Self {
        LapTime::LapTime(Duration::minutes(minutes) + Duration::seconds(seconds) + Duration::milliseconds(milliseconds))
    }

    pub fn has_time(&self) -> bool {
        matches!(self, LapTime::LapTime(_))
    }

    pub fn no_time_set(&self) -> bool {
        matches!(self, LapTime::NoTimeSet)
    }

    pub fn time(&self) -> &time::Duration {
        match &self {
            LapTime::LapTime(time) => time,
            _ => panic!("Cannot get time of NoTimeSet"),
        }
    }
}

impl FromStr for LapTime {
    type Err = Void;

    // @todo Implement a proper Err for parsing failures, instead of panics
    fn from_str(time: &str) -> Result<Self, Self::Err> {
        let re = Lazy::new(|| Regex::new(r"^((\d):)?([0-5]?\d)\.(\d{3})$").unwrap());

        if time.is_empty() {
            Ok(LapTime::NoTimeSet)
        } else {
            assert!(re.is_match(time));

            let matches = re.captures(time).unwrap();

            let minutes = if matches.get(2).is_some() {
                matches[2].parse::<i64>().unwrap()
            } else {
                0
            };

            let seconds = matches[3].parse::<i64>().unwrap();
            let milliseconds = matches[4].parse::<i64>().unwrap();

            Ok(LapTime::from(minutes, seconds, milliseconds))
        }
    }
}

#[cfg(test)]
mod tests {
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
    fn lap_time_from() {
        let lap = LapTime::from(1, 23, 456);

        assert!(matches!(lap, LapTime::LapTime(_)));
        assert!(lap.has_time());
        assert!(!lap.no_time_set());

        let ctime = lap.time().clone();

        if let LapTime::LapTime(time) = lap {
            assert_eq!(time, ctime);

            assert_eq!(time.whole_minutes(), 1);
            assert_eq!(time.whole_seconds() - 60, 23);
            assert_eq!(time.subsec_milliseconds(), 456);
        }
    }

    #[test]
    #[should_panic]
    fn lap_time_time_panics() {
        let lap = LapTime::NoTimeSet;

        assert!(matches!(lap, LapTime::NoTimeSet));
        assert!(!lap.has_time());
        assert!(lap.no_time_set());

        lap.time();
    }

    #[test]
    fn lap_time_from_str() {
        assert_eq!(LapTime::from_str("1:22.327").unwrap(), LapTime::from(1, 22, 327));
        assert_eq!(LapTime::from_str("1:41.269").unwrap(), LapTime::from(1, 41, 269));

        assert_eq!(LapTime::from_str("59.037").unwrap(), LapTime::from(0, 59, 037));

        assert_eq!(LapTime::from_str("2:01.341").unwrap(), LapTime::from(2, 1, 341));

        assert!(matches!(LapTime::from_str("").unwrap(), LapTime::NoTimeSet));
    }

    #[test]
    #[should_panic]
    fn lap_time_from_str_panics() {
        assert!(std::panic::catch_unwind(|| LapTime::from_str("90.203").unwrap()).is_err());
        assert!(std::panic::catch_unwind(|| LapTime::from_str("10.1").unwrap()).is_err());
        assert!(std::panic::catch_unwind(|| LapTime::from_str("10.1").unwrap()).is_err());

        // To satisfy should_panic, itself to indicate that this test checks panics
        LapTime::from_str("1").unwrap();
    }
}
