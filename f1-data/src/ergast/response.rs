use serde::Deserialize;

pub const GRID_PIT_LANE: u32 = 0;

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Response {
    #[serde(rename = "MRData")]
    pub mr_data: MrData,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct MrData {
    pub xmlns: String,
    pub series: String,
    pub url: String,
    pub limit: String,
    pub offset: String,
    pub total: String,
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

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Season {
    pub season: String,
    pub url: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub driver_id: String,
    pub permanent_number: Option<String>,
    pub code: Option<String>,
    pub url: String,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: String,
    pub nationality: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    pub constructor_id: String,
    pub url: String,
    pub name: String,
    pub nationality: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub circuit_id: String,
    pub url: String,
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Race {
    pub season: String,
    pub round: String,
    pub url: String,
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
    pub results: Option<Vec<Result>>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct QualifyingResult {
    pub number: String,
    pub position: String,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde(rename = "Q1")]
    pub q1: Option<String>,
    #[serde(rename = "Q2")]
    pub q2: Option<String>,
    #[serde(rename = "Q3")]
    pub q3: Option<String>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SprintResult {
    pub number: String,
    pub position: String,
    pub position_text: String,
    pub points: String,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    pub grid: String,
    pub laps: String,
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<Time>,
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub number: String,
    pub position: String,
    pub position_text: String,
    pub points: String,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    pub grid: String,
    pub laps: String,
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<Time>,
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub status_id: String,
    pub count: String,
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

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Time {
    pub millis: Option<String>,
    pub time: String,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct FastestLap {
    pub rank: Option<String>,
    pub lap: String,
    #[serde(rename = "Time")]
    pub time: Time,
    #[serde(rename = "AverageSpeed")]
    pub average_speed: Option<AverageSpeed>,
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct AverageSpeed {
    pub units: String,
    pub speed: String,
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
}
