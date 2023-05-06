use const_format::formatcp;
use once_cell::sync::Lazy;

use super::response::*;

// http://ergast.com/mrd/methods/seasons/
// --------------------------------------

pub const SEASON_1950_STR: &str = r#"{
    "season": "1950",
    "url": "http://en.wikipedia.org/wiki/1950_Formula_One_season"
  }"#;

pub const SEASON_1979_STR: &str = r#"{
    "season": "1979",
    "url": "http://en.wikipedia.org/wiki/1979_Formula_One_season"
  }"#;

pub const SEASON_2000_STR: &str = r#"{
    "season": "2000",
    "url": "http://en.wikipedia.org/wiki/2000_Formula_One_season"
  }"#;

pub const SEASON_2023_STR: &str = r#"{
    "season": "2023",
    "url": "https://en.wikipedia.org/wiki/2023_Formula_One_World_Championship"
  }"#;

pub static SEASON_1950: Lazy<Season> = Lazy::new(|| Season {
    season: "1950".to_string(),
    url: "http://en.wikipedia.org/wiki/1950_Formula_One_season".to_string(),
});

pub static SEASON_1979: Lazy<Season> = Lazy::new(|| Season {
    season: "1979".to_string(),
    url: "http://en.wikipedia.org/wiki/1979_Formula_One_season".to_string(),
});

pub static SEASON_2000: Lazy<Season> = Lazy::new(|| Season {
    season: "2000".to_string(),
    url: "http://en.wikipedia.org/wiki/2000_Formula_One_season".to_string(),
});

pub static SEASON_2023: Lazy<Season> = Lazy::new(|| Season {
    season: "2023".to_string(),
    url: "https://en.wikipedia.org/wiki/2023_Formula_One_World_Championship".to_string(),
});

pub const SEASON_TABLE_STR: &str = formatcp!(
    r#"{{
    "Seasons": [
        {SEASON_1950_STR},
        {SEASON_1979_STR},
        {SEASON_2000_STR},
        {SEASON_2023_STR}
    ]}}"#
);

pub static SEASON_TABLE: Lazy<SeasonTable> = Lazy::new(|| SeasonTable {
    seasons: vec![
        SEASON_1950.clone(),
        SEASON_1979.clone(),
        SEASON_2000.clone(),
        SEASON_2023.clone(),
    ],
});

// http://ergast.com/mrd/methods/drivers/
// --------------------------------------

// All possible fields are present
pub const DRIVER_ALONSO_STR: &str = r#"{
    "driverId": "alonso",
    "permanentNumber": "14",
    "code": "ALO",
    "url": "http://en.wikipedia.org/wiki/Fernando_Alonso",
    "givenName": "Fernando",
    "familyName": "Alonso",
    "dateOfBirth": "1981-07-29",
    "nationality": "Spanish"
  }"#;

// Optional fields are missing: ["permanentNumber", "code"]
pub const DRIVER_ABATE_STR: &str = r#"{
    "driverId": "abate",
    "url": "http://en.wikipedia.org/wiki/Carlo_Mario_Abate",
    "givenName": "Carlo",
    "familyName": "Abate",
    "dateOfBirth": "1932-07-10",
    "nationality": "Italian"
  }"#;

pub static DRIVER_ALONSO: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "alonso".to_string(),
    permanent_number: Some("14".to_string()),
    code: Some("ALO".to_string()),
    url: "http://en.wikipedia.org/wiki/Fernando_Alonso".to_string(),
    given_name: "Fernando".to_string(),
    family_name: "Alonso".to_string(),
    date_of_birth: "1981-07-29".to_string(),
    nationality: "Spanish".to_string(),
});

pub static DRIVER_ABATE: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "abate".to_string(),
    permanent_number: None,
    code: None,
    url: "http://en.wikipedia.org/wiki/Carlo_Mario_Abate".to_string(),
    given_name: "Carlo".to_string(),
    family_name: "Abate".to_string(),
    date_of_birth: "1932-07-10".to_string(),
    nationality: "Italian".to_string(),
});

pub const DRIVER_TABLE_STR: &str = formatcp!(
    r#"{{
    "Drivers": [
        {DRIVER_ALONSO_STR},
        {DRIVER_ABATE_STR}
    ]}}"#
);

pub static DRIVER_TABLE: Lazy<DriverTable> = Lazy::new(|| DriverTable {
    drivers: vec![DRIVER_ALONSO.clone(), DRIVER_ABATE.clone()],
});

// http://ergast.com/mrd/methods/constructors/
// -------------------------------------------

pub const CONSTRUCTOR_MCLAREN_STR: &str = r#"{
    "constructorId": "mclaren",
    "url": "http://en.wikipedia.org/wiki/McLaren",
    "name": "McLaren",
    "nationality": "British"
  }"#;

pub const CONSTRUCTOR_FERRARI_STR: &str = r#"{
    "constructorId": "ferrari",
    "url": "http://en.wikipedia.org/wiki/Scuderia_Ferrari",
    "name": "Ferrari",
    "nationality": "Italian"
  }"#;

pub static CONSTRUCTOR_MCLAREN: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "mclaren".to_string(),
    url: "http://en.wikipedia.org/wiki/McLaren".to_string(),
    name: "McLaren".to_string(),
    nationality: "British".to_string(),
});

pub static CONSTRUCTOR_FERRARI: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "ferrari".to_string(),
    url: "http://en.wikipedia.org/wiki/Scuderia_Ferrari".to_string(),
    name: "Ferrari".to_string(),
    nationality: "Italian".to_string(),
});

pub const CONSTRUCTOR_TABLE_STR: &str = formatcp!(
    r#"{{
    "Constructors": [
        {CONSTRUCTOR_MCLAREN_STR},
        {CONSTRUCTOR_FERRARI_STR}
    ]}}"#
);

pub static CONSTRUCTOR_TABLE: Lazy<ConstructorTable> = Lazy::new(|| ConstructorTable {
    constructors: vec![CONSTRUCTOR_MCLAREN.clone(), CONSTRUCTOR_FERRARI.clone()],
});

// http://ergast.com/mrd/methods/circuits/
// ---------------------------------------

pub const CIRCUIT_SPA_STR: &str = r#"{
    "circuitId": "spa",
    "url": "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps",
    "circuitName": "Circuit de Spa-Francorchamps",
    "Location": {
      "lat": "50.4372",
      "long": "5.97139",
      "locality": "Spa",
      "country": "Belgium"
    }
  }"#;

pub const CIRCUIT_SILVERSTONE_STR: &str = r#"{
    "circuitId": "silverstone",
    "url": "http://en.wikipedia.org/wiki/Silverstone_Circuit",
    "circuitName": "Silverstone Circuit",
    "Location": {
      "lat": "52.0786",
      "long": "-1.01694",
      "locality": "Silverstone",
      "country": "UK"
    }
  }"#;

pub const CIRCUIT_IMOLA_STR: &str = r#"{
    "circuitId": "imola",
    "url": "http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari",
    "circuitName": "Autodromo Enzo e Dino Ferrari",
    "Location": {
      "lat": "44.3439",
      "long": "11.7167",
      "locality": "Imola",
      "country": "Italy"
    }
  }"#;

pub const CIRCUIT_BAKU_STR: &str = r#"{
    "circuitId": "baku",
    "url": "http://en.wikipedia.org/wiki/Baku_City_Circuit",
    "circuitName": "Baku City Circuit",
    "Location": {
      "lat": "40.3725",
      "long": "49.8533",
      "locality": "Baku",
      "country": "Azerbaijan"
    }
  }"#;

pub static CIRCUIT_SPA: Lazy<Circuit> = Lazy::new(|| Circuit {
    circuit_id: "spa".to_string(),
    url: "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps".to_string(),
    circuit_name: "Circuit de Spa-Francorchamps".to_string(),
    location: Location {
        lat: "50.4372".to_string(),
        long: "5.97139".to_string(),
        locality: "Spa".to_string(),
        country: "Belgium".to_string(),
    },
});

pub static CIRCUIT_SILVERSTONE: Lazy<Circuit> = Lazy::new(|| Circuit {
    circuit_id: "silverstone".to_string(),
    url: "http://en.wikipedia.org/wiki/Silverstone_Circuit".to_string(),
    circuit_name: "Silverstone Circuit".to_string(),
    location: Location {
        lat: "52.0786".to_string(),
        long: "-1.01694".to_string(),
        locality: "Silverstone".to_string(),
        country: "UK".to_string(),
    },
});

pub static CIRCUIT_IMOLA: Lazy<Circuit> = Lazy::new(|| Circuit {
    circuit_id: "imola".to_string(),
    url: "http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari".to_string(),
    circuit_name: "Autodromo Enzo e Dino Ferrari".to_string(),
    location: Location {
        lat: "44.3439".to_string(),
        long: "11.7167".to_string(),
        locality: "Imola".to_string(),
        country: "Italy".to_string(),
    },
});

pub static CIRCUIT_BAKU: Lazy<Circuit> = Lazy::new(|| Circuit {
    circuit_id: "baku".to_string(),
    url: "http://en.wikipedia.org/wiki/Baku_City_Circuit".to_string(),
    circuit_name: "Baku City Circuit".to_string(),
    location: Location {
        lat: "40.3725".to_string(),
        long: "49.8533".to_string(),
        locality: "Baku".to_string(),
        country: "Azerbaijan".to_string(),
    },
});

pub const CIRCUIT_TABLE_STR: &str = formatcp!(
    r#"{{
    "Circuits": [
        {CIRCUIT_SPA_STR},
        {CIRCUIT_SILVERSTONE_STR},
        {CIRCUIT_IMOLA_STR},
        {CIRCUIT_BAKU_STR}
    ]}}"#
);

pub static CIRCUIT_TABLE: Lazy<CircuitTable> = Lazy::new(|| CircuitTable {
    circuits: vec![
        CIRCUIT_SPA.clone(),
        CIRCUIT_SILVERSTONE.clone(),
        CIRCUIT_IMOLA.clone(),
        CIRCUIT_BAKU.clone(),
    ],
});

// http://ergast.com/mrd/methods/schedule/
// ---------------------------------------

// Has "date" only
pub const RACE_1950_1_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    "season": "1950",
    "round": "1",
    "url": "http://en.wikipedia.org/wiki/1950_British_Grand_Prix",
    "raceName": "British Grand Prix",
    "Circuit": {CIRCUIT_SILVERSTONE_STR},
    "date": "1950-05-13"
  }}"#
);

// Has "date" and "time"
pub const RACE_2015_11_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    "season": "2015",
    "round": "11",
    "url": "http://en.wikipedia.org/wiki/2015_Belgian_Grand_Prix",
    "raceName": "Belgian Grand Prix",
    "Circuit": {CIRCUIT_SPA_STR},
    "date": "2015-08-23",
    "time": "12:00:00Z"
  }}"#
);

// Has "FirstPractice", "SecondPractice", "ThirdPractice", "Qualifying"
// Sessions have only "data"
pub const RACE_2021_12_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    "season": "2021",
    "round": "12",
    "url": "http://en.wikipedia.org/wiki/2021_Belgian_Grand_Prix",
    "raceName": "Belgian Grand Prix",
    "Circuit": {CIRCUIT_SPA_STR},
    "date": "2021-08-29",
    "time": "13:00:00Z",
    "FirstPractice": {{
      "date": "2021-08-27"
    }},
    "SecondPractice": {{
      "date": "2021-08-27"
    }},
    "ThirdPractice": {{
      "date": "2021-08-28"
    }},
    "Qualifying": {{
      "date": "2021-08-28"
    }}
}}"#
);

// Has "Sprint"
// Sessions have "date" and "time"
pub const RACE_2022_4_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    "season": "2022",
    "round": "4",
    "url": "http://en.wikipedia.org/wiki/2022_Emilia_Romagna_Grand_Prix",
    "raceName": "Emilia Romagna Grand Prix",
    "Circuit": {CIRCUIT_IMOLA_STR},
    "date": "2022-04-24",
    "time": "13:00:00Z",
    "FirstPractice":  {{
      "date": "2022-04-22",
      "time": "11:30:00Z"
    }},
    "Qualifying":  {{
      "date": "2022-04-22",
      "time": "15:00:00Z"
    }},
    "SecondPractice":  {{
      "date": "2022-04-23",
      "time": "10:30:00Z"
    }},
    "Sprint":  {{
      "date": "2022-04-23",
      "time": "14:30:00Z"
    }}
}}"#
);

// @todo Should have sprint shootout session, but Ergast has not updated
pub const RACE_2023_4_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    "season": "2023",
    "round": "4",
    "url": "https://en.wikipedia.org/wiki/2023_Azerbaijan_Grand_Prix",
    "raceName": "Azerbaijan Grand Prix",
    "Circuit": {CIRCUIT_BAKU_STR},
    "date": "2023-04-30",
    "time": "11:00:00Z",
    "FirstPractice":  {{
      "date": "2023-04-28",
      "time": "09:30:00Z"
    }},
    "Qualifying":  {{
      "date": "2023-04-28",
      "time": "13:00:00Z"
    }},
    "SecondPractice":  {{
      "date": "2023-04-29",
      "time": "09:30:00Z"
    }},
    "Sprint": {{
      "date": "2023-04-29",
      "time": "13:30:00Z"
    }}
}}"#
);

pub const RACE_1950_1_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    season: "1950".to_string(),
    round: "1".to_string(),
    url: "http://en.wikipedia.org/wiki/1950_British_Grand_Prix".to_string(),
    race_name: "British Grand Prix".to_string(),
    circuit: CIRCUIT_SILVERSTONE.clone(),
    date: "1950-05-13".to_string(),
    time: None,
    first_practice: None,
    second_practice: None,
    third_practice: None,
    qualifying: None,
    sprint: None,
    qualifying_results: None,
    sprint_results: None,
    results: None,
});

pub const RACE_2015_11_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    season: "2015".to_string(),
    round: "11".to_string(),
    url: "http://en.wikipedia.org/wiki/2015_Belgian_Grand_Prix".to_string(),
    race_name: "Belgian Grand Prix".to_string(),
    circuit: CIRCUIT_SPA.clone(),
    date: "2015-08-23".to_string(),
    time: Some("12:00:00Z".to_string()),
    first_practice: None,
    second_practice: None,
    third_practice: None,
    qualifying: None,
    sprint: None,
    qualifying_results: None,
    sprint_results: None,
    results: None,
});

pub const RACE_2021_12_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    season: "2021".to_string(),
    round: "12".to_string(),
    url: "http://en.wikipedia.org/wiki/2021_Belgian_Grand_Prix".to_string(),
    race_name: "Belgian Grand Prix".to_string(),
    circuit: CIRCUIT_SPA.clone(),
    date: "2021-08-29".to_string(),
    time: Some("13:00:00Z".to_string()),
    first_practice: Some(DateTime {
        date: "2021-08-27".to_string(),
        time: None,
    }),
    second_practice: Some(DateTime {
        date: "2021-08-27".to_string(),
        time: None,
    }),
    third_practice: Some(DateTime {
        date: "2021-08-28".to_string(),
        time: None,
    }),
    qualifying: Some(DateTime {
        date: "2021-08-28".to_string(),
        time: None,
    }),
    sprint: None,
    qualifying_results: None,
    sprint_results: None,
    results: None,
});

pub const RACE_2022_4_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    season: "2022".to_string(),
    round: "4".to_string(),
    url: "http://en.wikipedia.org/wiki/2022_Emilia_Romagna_Grand_Prix".to_string(),
    race_name: "Emilia Romagna Grand Prix".to_string(),
    circuit: CIRCUIT_IMOLA.clone(),
    date: "2022-04-24".to_string(),
    time: Some("13:00:00Z".to_string()),
    first_practice: Some(DateTime {
        date: "2022-04-22".to_string(),
        time: Some("11:30:00Z".to_string()),
    }),
    qualifying: Some(DateTime {
        date: "2022-04-22".to_string(),
        time: Some("15:00:00Z".to_string()),
    }),
    second_practice: Some(DateTime {
        date: "2022-04-23".to_string(),
        time: Some("10:30:00Z".to_string()),
    }),
    sprint: Some(DateTime {
        date: "2022-04-23".to_string(),
        time: Some("14:30:00Z".to_string()),
    }),
    third_practice: None,
    qualifying_results: None,
    sprint_results: None,
    results: None,
});

pub const RACE_2023_4_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    season: "2023".to_string(),
    round: "4".to_string(),
    url: "https://en.wikipedia.org/wiki/2023_Azerbaijan_Grand_Prix".to_string(),
    race_name: "Azerbaijan Grand Prix".to_string(),
    circuit: CIRCUIT_BAKU.clone(),
    date: "2023-04-30".to_string(),
    time: Some("11:00:00Z".to_string()),
    first_practice: Some(DateTime {
        date: "2023-04-28".to_string(),
        time: Some("09:30:00Z".to_string()),
    }),
    qualifying: Some(DateTime {
        date: "2023-04-28".to_string(),
        time: Some("13:00:00Z".to_string()),
    }),
    second_practice: Some(DateTime {
        date: "2023-04-29".to_string(),
        time: Some("09:30:00Z".to_string()),
    }),
    sprint: Some(DateTime {
        date: "2023-04-29".to_string(),
        time: Some("13:30:00Z".to_string()),
    }),
    third_practice: None,
    qualifying_results: None,
    sprint_results: None,
    results: None,
});

pub const RACE_TABLE_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    "Races": [
        {RACE_1950_1_SCHEDULE_STR},
        {RACE_2015_11_SCHEDULE_STR},
        {RACE_2021_12_SCHEDULE_STR},
        {RACE_2022_4_SCHEDULE_STR},
        {RACE_2023_4_SCHEDULE_STR}
    ]}}"#
);

pub static RACE_TABLE_SCHEDULE: Lazy<RaceTable> = Lazy::new(|| RaceTable {
    races: vec![
        RACE_1950_1_SCHEDULE.clone(),
        RACE_2015_11_SCHEDULE.clone(),
        RACE_2021_12_SCHEDULE.clone(),
        RACE_2022_4_SCHEDULE.clone(),
        RACE_2023_4_SCHEDULE.clone(),
    ],
});
