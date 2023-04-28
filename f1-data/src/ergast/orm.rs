use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(rename = "MRData")]
    pub mr_data: MrData,
}

#[derive(Deserialize, Debug)]
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
}

#[derive(Deserialize, Debug)]
pub struct SeasonTable {
    #[serde(rename = "Seasons")]
    pub seasons: Vec<Season>,
}

#[derive(Deserialize, Debug)]
pub struct DriverTable {
    #[serde(rename = "Drivers")]
    pub drivers: Vec<Driver>,
}

#[derive(Deserialize, Debug)]
pub struct ConstructorTable {
    #[serde(rename = "Constructors")]
    pub constructors: Vec<Constructor>,
}

#[derive(Deserialize, Debug)]
pub struct CircuitTable {
    #[serde(rename = "Circuits")]
    pub circuits: Vec<Circuit>,
}

#[derive(Deserialize, Debug)]
pub struct RaceTable {
    #[serde(rename = "Races")]
    pub races: Vec<Race>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Season {
    pub season: String,
    pub url: String,
}

#[derive(Deserialize, PartialEq, Debug)]
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

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    pub constructor_id: String,
    pub url: String,
    pub name: String,
    pub nationality: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub circuit_id: String,
    pub url: String,
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

#[derive(Deserialize, PartialEq, Debug)]
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
    pub qualifying_results: Vec<QualifyingResult>,
    #[serde(rename = "SprintResults")]
    pub sprint_results: Vec<SprintResult>,
    #[serde(rename = "Results")]
    pub results: Vec<Result>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QualifyingResult {
    pub number: String,
    pub position: String,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    pub q1: Option<String>,
    pub q2: Option<String>,
    pub q3: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
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

#[derive(Deserialize, PartialEq, Debug)]
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

#[derive(Deserialize, PartialEq, Debug)]
pub struct Location {
    pub lat: String,
    pub long: String,
    pub locality: String,
    pub country: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DateTime {
    pub date: String,
    pub time: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Time {
    pub millis: Option<String>,
    pub time: String,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct FastestLap {
    pub rank: String,
    pub lap: String,
    #[serde(rename = "Time")]
    pub time: Time,
    #[serde(rename = "AverageSpeed")]
    pub average_speed: AverageSpeed,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct AverageSpeed {
    pub units: String,
    pub speed: String,
}
