use const_format::formatcp;
use once_cell::sync::Lazy;
use time::macros::{date, time};
use url::Url;

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
    season: 1950,
    url: Url::parse("http://en.wikipedia.org/wiki/1950_Formula_One_season").unwrap(),
});

pub static SEASON_1979: Lazy<Season> = Lazy::new(|| Season {
    season: 1979,
    url: Url::parse("http://en.wikipedia.org/wiki/1979_Formula_One_season").unwrap(),
});

pub static SEASON_2000: Lazy<Season> = Lazy::new(|| Season {
    season: 2000,
    url: Url::parse("http://en.wikipedia.org/wiki/2000_Formula_One_season").unwrap(),
});

pub static SEASON_2023: Lazy<Season> = Lazy::new(|| Season {
    season: 2023,
    url: Url::parse("https://en.wikipedia.org/wiki/2023_Formula_One_World_Championship").unwrap(),
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

// Optional fields are missing: ["permanentNumber", "code"]
pub const DRIVER_ABATE_STR: &str = r#"{
    "driverId": "abate",
    "url": "http://en.wikipedia.org/wiki/Carlo_Mario_Abate",
    "givenName": "Carlo",
    "familyName": "Abate",
    "dateOfBirth": "1932-07-10",
    "nationality": "Italian"
  }"#;

// Optional fields are missing: ["permanentNumber"]
pub const DRIVER_MICHAEL_STR: &str = r#"{
    "driverId": "michael_schumacher",
    "code": "MSC",
    "url": "http://en.wikipedia.org/wiki/Michael_Schumacher",
    "givenName": "Michael",
    "familyName": "Schumacher",
    "dateOfBirth": "1969-01-03",
    "nationality": "German"
  }"#;

// Optional fields are missing: ["permanentNumber", "code"]
pub const DRIVER_JOS_STR: &str = r#"{
    "driverId": "verstappen",
    "url": "http://en.wikipedia.org/wiki/Jos_Verstappen",
    "givenName": "Jos",
    "familyName": "Verstappen",
    "dateOfBirth": "1972-03-04",
    "nationality": "Dutch"
  }"#;

// Optional fields are missing: ["permanentNumber"]
pub const DRIVER_RALF_STR: &str = r#"{
    "driverId": "ralf_schumacher",
    "code": "SCH",
    "url": "http://en.wikipedia.org/wiki/Ralf_Schumacher",
    "givenName": "Ralf",
    "familyName": "Schumacher",
    "dateOfBirth": "1975-06-30",
    "nationality": "German"
  }"#;

// Optional fields are missing: ["permanentNumber", "code"]
pub const DRIVER_WILSON_STR: &str = r#"{
    "driverId": "wilson",
    "url": "http://en.wikipedia.org/wiki/Justin_Wilson_(racing_driver)",
    "givenName": "Justin",
    "familyName": "Wilson",
    "dateOfBirth": "1978-07-31",
    "nationality": "British"
  }"#;

pub const DRIVER_KIMI_STR: &str = r#"{
    "driverId": "raikkonen",
    "permanentNumber": "7",
    "code": "RAI",
    "url": "http://en.wikipedia.org/wiki/Kimi_R%C3%A4ikk%C3%B6nen",
    "givenName": "Kimi",
    "familyName": "Räikkönen",
    "dateOfBirth": "1979-10-17",
    "nationality": "Finnish"
  }"#;

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

pub const DRIVER_PEREZ_STR: &str = r#"{
    "driverId": "perez",
    "permanentNumber": "11",
    "code": "PER",
    "url": "http://en.wikipedia.org/wiki/Sergio_P%C3%A9rez",
    "givenName": "Sergio",
    "familyName": "Pérez",
    "dateOfBirth": "1990-01-26",
    "nationality": "Mexican"
  }"#;

pub const DRIVER_DE_VRIES_STR: &str = r#"{
    "driverId": "de_vries",
    "permanentNumber": "21",
    "code": "DEV",
    "url": "http://en.wikipedia.org/wiki/Nyck_de_Vries",
    "givenName": "Nyck",
    "familyName": "de Vries",
    "dateOfBirth": "1995-02-06",
    "nationality": "Dutch"
  }"#;

pub const DRIVER_MAX_STR: &str = r#"{
    "driverId": "max_verstappen",
    "permanentNumber": "33",
    "code": "VER",
    "url": "http://en.wikipedia.org/wiki/Max_Verstappen",
    "givenName": "Max",
    "familyName": "Verstappen",
    "dateOfBirth": "1997-09-30",
    "nationality": "Dutch"
  }"#;

pub const DRIVER_LECLERC_STR: &str = r#"{
    "driverId": "leclerc",
    "permanentNumber": "16",
    "code": "LEC",
    "url": "http://en.wikipedia.org/wiki/Charles_Leclerc",
    "givenName": "Charles",
    "familyName": "Leclerc",
    "dateOfBirth": "1997-10-16",
    "nationality": "Monegasque"
  }"#;

pub static DRIVER_ABATE: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "abate".to_string(),
    permanent_number: None,
    code: None,
    url: Url::parse("http://en.wikipedia.org/wiki/Carlo_Mario_Abate").unwrap(),
    given_name: "Carlo".to_string(),
    family_name: "Abate".to_string(),
    date_of_birth: "1932-07-10".to_string(),
    nationality: "Italian".to_string(),
});

pub static DRIVER_MICHAEL: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "michael_schumacher".to_string(),
    permanent_number: None,
    code: Some("MSC".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Michael_Schumacher").unwrap(),
    given_name: "Michael".to_string(),
    family_name: "Schumacher".to_string(),
    date_of_birth: "1969-01-03".to_string(),
    nationality: "German".to_string(),
});

pub static DRIVER_JOS: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "verstappen".to_string(),
    permanent_number: None,
    code: None,
    url: Url::parse("http://en.wikipedia.org/wiki/Jos_Verstappen").unwrap(),
    given_name: "Jos".to_string(),
    family_name: "Verstappen".to_string(),
    date_of_birth: "1972-03-04".to_string(),
    nationality: "Dutch".to_string(),
});

pub static DRIVER_RALF: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "ralf_schumacher".to_string(),
    permanent_number: None,
    code: Some("SCH".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Ralf_Schumacher").unwrap(),
    given_name: "Ralf".to_string(),
    family_name: "Schumacher".to_string(),
    date_of_birth: "1975-06-30".to_string(),
    nationality: "German".to_string(),
});

pub static DRIVER_WILSON: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "wilson".to_string(),
    permanent_number: None,
    code: None,
    url: Url::parse("http://en.wikipedia.org/wiki/Justin_Wilson_(racing_driver)").unwrap(),
    given_name: "Justin".to_string(),
    family_name: "Wilson".to_string(),
    date_of_birth: "1978-07-31".to_string(),
    nationality: "British".to_string(),
});

pub static DRIVER_KIMI: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "raikkonen".to_string(),
    permanent_number: Some(7),
    code: Some("RAI".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Kimi_R%C3%A4ikk%C3%B6nen").unwrap(),
    given_name: "Kimi".to_string(),
    family_name: "Räikkönen".to_string(),
    date_of_birth: "1979-10-17".to_string(),
    nationality: "Finnish".to_string(),
});

pub static DRIVER_ALONSO: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "alonso".to_string(),
    permanent_number: Some(14),
    code: Some("ALO".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Fernando_Alonso").unwrap(),
    given_name: "Fernando".to_string(),
    family_name: "Alonso".to_string(),
    date_of_birth: "1981-07-29".to_string(),
    nationality: "Spanish".to_string(),
});

pub static DRIVER_PEREZ: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "perez".to_string(),
    permanent_number: Some(11),
    code: Some("PER".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Sergio_P%C3%A9rez").unwrap(),
    given_name: "Sergio".to_string(),
    family_name: "Pérez".to_string(),
    date_of_birth: "1990-01-26".to_string(),
    nationality: "Mexican".to_string(),
});

pub static DRIVER_DE_VRIES: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "de_vries".to_string(),
    permanent_number: Some(21),
    code: Some("DEV".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Nyck_de_Vries").unwrap(),
    given_name: "Nyck".to_string(),
    family_name: "de Vries".to_string(),
    date_of_birth: "1995-02-06".to_string(),
    nationality: "Dutch".to_string(),
});

pub static DRIVER_MAX: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "max_verstappen".to_string(),
    permanent_number: Some(33),
    code: Some("VER".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Max_Verstappen").unwrap(),
    given_name: "Max".to_string(),
    family_name: "Verstappen".to_string(),
    date_of_birth: "1997-09-30".to_string(),
    nationality: "Dutch".to_string(),
});

pub static DRIVER_LECLERC: Lazy<Driver> = Lazy::new(|| Driver {
    driver_id: "leclerc".to_string(),
    permanent_number: Some(16),
    code: Some("LEC".to_string()),
    url: Url::parse("http://en.wikipedia.org/wiki/Charles_Leclerc").unwrap(),
    given_name: "Charles".to_string(),
    family_name: "Leclerc".to_string(),
    date_of_birth: "1997-10-16".to_string(),
    nationality: "Monegasque".to_string(),
});

pub const DRIVER_TABLE_STR: &str = formatcp!(
    r#"{{
    "Drivers": [
        {DRIVER_ABATE_STR},
        {DRIVER_MICHAEL_STR},
        {DRIVER_JOS_STR},
        {DRIVER_RALF_STR},
        {DRIVER_WILSON_STR},
        {DRIVER_KIMI_STR},
        {DRIVER_ALONSO_STR},
        {DRIVER_PEREZ_STR},
        {DRIVER_DE_VRIES_STR},
        {DRIVER_MAX_STR},
        {DRIVER_LECLERC_STR}
    ]}}"#
);

pub static DRIVER_TABLE: Lazy<DriverTable> = Lazy::new(|| DriverTable {
    drivers: vec![
        DRIVER_ABATE.clone(),
        DRIVER_MICHAEL.clone(),
        DRIVER_JOS.clone(),
        DRIVER_RALF.clone(),
        DRIVER_WILSON.clone(),
        DRIVER_KIMI.clone(),
        DRIVER_ALONSO.clone(),
        DRIVER_PEREZ.clone(),
        DRIVER_DE_VRIES.clone(),
        DRIVER_MAX.clone(),
        DRIVER_LECLERC.clone(),
    ],
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

pub const CONSTRUCTOR_WILLIAMS_STR: &str = r#"{
    "constructorId": "williams",
    "url": "http://en.wikipedia.org/wiki/Williams_Grand_Prix_Engineering",
    "name": "Williams",
    "nationality": "British"
  }"#;

pub const CONSTRUCTOR_MINARDI_STR: &str = r#"{
    "constructorId": "minardi",
    "url": "http://en.wikipedia.org/wiki/Minardi",
    "name": "Minardi",
    "nationality": "Italian"
  }"#;

pub const CONSTRUCTOR_ALPHA_TAURI_STR: &str = r#"{
    "constructorId": "alphatauri",
    "url": "http://en.wikipedia.org/wiki/Scuderia_AlphaTauri",
    "name": "AlphaTauri",
    "nationality": "Italian"
  }"#;

pub const CONSTRUCTOR_RED_BULL_STR: &str = r#"{
    "constructorId": "red_bull",
    "url": "http://en.wikipedia.org/wiki/Red_Bull_Racing",
    "name": "Red Bull",
    "nationality": "Austrian"
  }"#;

pub static CONSTRUCTOR_MCLAREN: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "mclaren".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/McLaren").unwrap(),
    name: "McLaren".to_string(),
    nationality: "British".to_string(),
});

pub static CONSTRUCTOR_FERRARI: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "ferrari".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Scuderia_Ferrari").unwrap(),
    name: "Ferrari".to_string(),
    nationality: "Italian".to_string(),
});

pub static CONSTRUCTOR_WILLIAMS: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "williams".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Williams_Grand_Prix_Engineering").unwrap(),
    name: "Williams".to_string(),
    nationality: "British".to_string(),
});

pub static CONSTRUCTOR_MINARDI: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "minardi".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Minardi").unwrap(),
    name: "Minardi".to_string(),
    nationality: "Italian".to_string(),
});

pub static CONSTRUCTOR_ALPHA_TAURI: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "alphatauri".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Scuderia_AlphaTauri").unwrap(),
    name: "AlphaTauri".to_string(),
    nationality: "Italian".to_string(),
});

pub static CONSTRUCTOR_RED_BULL: Lazy<Constructor> = Lazy::new(|| Constructor {
    constructor_id: "red_bull".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Red_Bull_Racing").unwrap(),
    name: "Red Bull".to_string(),
    nationality: "Austrian".to_string(),
});

pub const CONSTRUCTOR_TABLE_STR: &str = formatcp!(
    r#"{{
    "Constructors": [
        {CONSTRUCTOR_MCLAREN_STR},
        {CONSTRUCTOR_FERRARI_STR},
        {CONSTRUCTOR_WILLIAMS_STR},
        {CONSTRUCTOR_MINARDI_STR},
        {CONSTRUCTOR_ALPHA_TAURI_STR},
        {CONSTRUCTOR_RED_BULL_STR}
    ]}}"#
);

pub static CONSTRUCTOR_TABLE: Lazy<ConstructorTable> = Lazy::new(|| ConstructorTable {
    constructors: vec![
        CONSTRUCTOR_MCLAREN.clone(),
        CONSTRUCTOR_FERRARI.clone(),
        CONSTRUCTOR_WILLIAMS.clone(),
        CONSTRUCTOR_MINARDI.clone(),
        CONSTRUCTOR_ALPHA_TAURI.clone(),
        CONSTRUCTOR_RED_BULL.clone(),
    ],
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
    url: Url::parse("http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps").unwrap(),
    circuit_name: "Circuit de Spa-Francorchamps".to_string(),
    location: Location {
        lat: 50.4372,
        long: 5.97139,
        locality: "Spa".to_string(),
        country: "Belgium".to_string(),
    },
});

pub static CIRCUIT_SILVERSTONE: Lazy<Circuit> = Lazy::new(|| Circuit {
    circuit_id: "silverstone".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Silverstone_Circuit").unwrap(),
    circuit_name: "Silverstone Circuit".to_string(),
    location: Location {
        lat: 52.0786,
        long: -1.01694,
        locality: "Silverstone".to_string(),
        country: "UK".to_string(),
    },
});

pub static CIRCUIT_IMOLA: Lazy<Circuit> = Lazy::new(|| Circuit {
    circuit_id: "imola".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Autodromo_Enzo_e_Dino_Ferrari").unwrap(),
    circuit_name: "Autodromo Enzo e Dino Ferrari".to_string(),
    location: Location {
        lat: 44.3439,
        long: 11.7167,
        locality: "Imola".to_string(),
        country: "Italy".to_string(),
    },
});

pub static CIRCUIT_BAKU: Lazy<Circuit> = Lazy::new(|| Circuit {
    circuit_id: "baku".to_string(),
    url: Url::parse("http://en.wikipedia.org/wiki/Baku_City_Circuit").unwrap(),
    circuit_name: "Baku City Circuit".to_string(),
    location: Location {
        lat: 40.3725,
        long: 49.8533,
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

// Races, used in schedule, qualifying, sprint, results
// ----------------------------------------------------

pub const RACE_1950_1_STR: &str = formatcp!(
    r#"
    "season": "1950",
    "round": "1",
    "url": "http://en.wikipedia.org/wiki/1950_British_Grand_Prix",
    "raceName": "British Grand Prix",
    "Circuit": {CIRCUIT_SILVERSTONE_STR},
    "date": "1950-05-13"
  "#
);

pub const RACE_2003_4_STR: &str = formatcp!(
    r#"
    "season": "2003",
    "round": "4",
    "url": "http://en.wikipedia.org/wiki/2003_San_Marino_Grand_Prix",
    "raceName": "San Marino Grand Prix",
    "Circuit": {CIRCUIT_IMOLA_STR},
    "date": "2003-04-20"
  "#
);

pub const RACE_2015_11_STR: &str = formatcp!(
    r#"
    "season": "2015",
    "round": "11",
    "url": "http://en.wikipedia.org/wiki/2015_Belgian_Grand_Prix",
    "raceName": "Belgian Grand Prix",
    "Circuit": {CIRCUIT_SPA_STR},
    "date": "2015-08-23",
    "time": "12:00:00Z"
  "#
);

pub const RACE_2021_12_STR: &str = formatcp!(
    r#"
    "season": "2021",
    "round": "12",
    "url": "http://en.wikipedia.org/wiki/2021_Belgian_Grand_Prix",
    "raceName": "Belgian Grand Prix",
    "Circuit": {CIRCUIT_SPA_STR},
    "date": "2021-08-29",
    "time": "13:00:00Z"
  "#
);

pub const RACE_2022_4_STR: &str = formatcp!(
    r#"
    "season": "2022",
    "round": "4",
    "url": "http://en.wikipedia.org/wiki/2022_Emilia_Romagna_Grand_Prix",
    "raceName": "Emilia Romagna Grand Prix",
    "Circuit": {CIRCUIT_IMOLA_STR},
    "date": "2022-04-24",
    "time": "13:00:00Z"
  "#
);

pub const RACE_2023_4_STR: &str = formatcp!(
    r#"
    "season": "2023",
    "round": "4",
    "url": "https://en.wikipedia.org/wiki/2023_Azerbaijan_Grand_Prix",
    "raceName": "Azerbaijan Grand Prix",
    "Circuit": {CIRCUIT_BAKU_STR},
    "date": "2023-04-30",
    "time": "11:00:00Z"
  "#
);

// Can be used to fill all unspecified fields for a given race
pub const RACE_NONE: Lazy<Race> = Lazy::new(|| Race {
    season: 0,
    round: 0,
    url: Url::parse("http://empty.org").unwrap(),
    race_name: "".to_string(),
    circuit: Circuit {
        circuit_id: "".to_string(),
        url: Url::parse("http://empty.org").unwrap(),
        circuit_name: "".to_string(),
        location: Location {
            lat: f64::NAN,
            long: f64::NAN,
            locality: "".to_string(),
            country: "".to_string(),
        },
    },
    date: time::Date::MIN,
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

pub const RACE_1950_1: Lazy<Race> = Lazy::new(|| Race {
    season: 1950,
    round: 1,
    url: Url::parse("http://en.wikipedia.org/wiki/1950_British_Grand_Prix").unwrap(),
    race_name: "British Grand Prix".to_string(),
    circuit: CIRCUIT_SILVERSTONE.clone(),
    date: date!(1950 - 05 - 13),
    ..RACE_NONE.clone()
});

pub const RACE_2003_4: Lazy<Race> = Lazy::new(|| Race {
    season: 2003,
    round: 4,
    url: Url::parse("http://en.wikipedia.org/wiki/2003_San_Marino_Grand_Prix").unwrap(),
    race_name: "San Marino Grand Prix".to_string(),
    circuit: CIRCUIT_IMOLA.clone(),
    date: date!(2003 - 04 - 20),
    ..RACE_NONE.clone()
});

pub const RACE_2015_11: Lazy<Race> = Lazy::new(|| Race {
    season: 2015,
    round: 11,
    url: Url::parse("http://en.wikipedia.org/wiki/2015_Belgian_Grand_Prix").unwrap(),
    race_name: "Belgian Grand Prix".to_string(),
    circuit: CIRCUIT_SPA.clone(),
    date: date!(2015 - 08 - 23),
    time: Some(time!(12:00:00)),
    ..RACE_NONE.clone()
});

pub const RACE_2021_12: Lazy<Race> = Lazy::new(|| Race {
    season: 2021,
    round: 12,
    url: Url::parse("http://en.wikipedia.org/wiki/2021_Belgian_Grand_Prix").unwrap(),
    race_name: "Belgian Grand Prix".to_string(),
    circuit: CIRCUIT_SPA.clone(),
    date: date!(2021 - 08 - 29),
    time: Some(time!(13:00:00)),
    ..RACE_NONE.clone()
});

pub const RACE_2022_4: Lazy<Race> = Lazy::new(|| Race {
    season: 2022,
    round: 4,
    url: Url::parse("http://en.wikipedia.org/wiki/2022_Emilia_Romagna_Grand_Prix").unwrap(),
    race_name: "Emilia Romagna Grand Prix".to_string(),
    circuit: CIRCUIT_IMOLA.clone(),
    date: date!(2022 - 04 - 24),
    time: Some(time!(13:00:00)),
    ..RACE_NONE.clone()
});

pub const RACE_2023_4: Lazy<Race> = Lazy::new(|| Race {
    season: 2023,
    round: 4,
    url: Url::parse("https://en.wikipedia.org/wiki/2023_Azerbaijan_Grand_Prix").unwrap(),
    race_name: "Azerbaijan Grand Prix".to_string(),
    circuit: CIRCUIT_BAKU.clone(),
    date: date!(2023 - 04 - 30),
    time: Some(time!(11:00:00)),
    ..RACE_NONE.clone()
});

// http://ergast.com/mrd/methods/schedule/
// ---------------------------------------

// Has "date" only
pub const RACE_1950_1_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    {RACE_1950_1_STR}
  }}"#
);

// Has "date" only
pub const RACE_2003_4_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    {RACE_2003_4_STR}
  }}"#
);

// Has "date" and "time"
pub const RACE_2015_11_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    {RACE_2015_11_STR}
  }}"#
);

// Has "FirstPractice", "SecondPractice", "ThirdPractice", "Qualifying"
// Sessions have only "date"
pub const RACE_2021_12_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    {RACE_2021_12_STR},
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
    {RACE_2022_4_STR},
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
    {RACE_2023_4_STR},
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

pub const RACE_1950_1_SCHEDULE: Lazy<Race> = Lazy::new(|| Race { ..RACE_1950_1.clone() });
pub const RACE_2003_4_SCHEDULE: Lazy<Race> = Lazy::new(|| Race { ..RACE_2003_4.clone() });
pub const RACE_2015_11_SCHEDULE: Lazy<Race> = Lazy::new(|| Race { ..RACE_2015_11.clone() });

pub const RACE_2021_12_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    first_practice: Some(DateTime {
        date: date!(2021 - 08 - 27),
        time: None,
    }),
    second_practice: Some(DateTime {
        date: date!(2021 - 08 - 27),
        time: None,
    }),
    third_practice: Some(DateTime {
        date: date!(2021 - 08 - 28),
        time: None,
    }),
    qualifying: Some(DateTime {
        date: date!(2021 - 08 - 28),
        time: None,
    }),
    ..RACE_2021_12.clone()
});

pub const RACE_2022_4_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    first_practice: Some(DateTime {
        date: date!(2022 - 04 - 22),
        time: Some(time!(11:30:00)),
    }),
    qualifying: Some(DateTime {
        date: date!(2022 - 04 - 22),
        time: Some(time!(15:00:00)),
    }),
    second_practice: Some(DateTime {
        date: date!(2022 - 04 - 23),
        time: Some(time!(10:30:00)),
    }),
    sprint: Some(DateTime {
        date: date!(2022 - 04 - 23),
        time: Some(time!(14:30:00)),
    }),
    ..RACE_2022_4.clone()
});

pub const RACE_2023_4_SCHEDULE: Lazy<Race> = Lazy::new(|| Race {
    first_practice: Some(DateTime {
        date: date!(2023 - 04 - 28),
        time: Some(time!(09:30:00)),
    }),
    qualifying: Some(DateTime {
        date: date!(2023 - 04 - 28),
        time: Some(time!(13:00:00)),
    }),
    second_practice: Some(DateTime {
        date: date!(2023 - 04 - 29),
        time: Some(time!(09:30:00)),
    }),
    sprint: Some(DateTime {
        date: date!(2023 - 04 - 29),
        time: Some(time!(13:30:00)),
    }),
    ..RACE_2023_4.clone()
});

pub const RACE_TABLE_SCHEDULE_STR: &str = formatcp!(
    r#"{{
    "Races": [
        {RACE_1950_1_SCHEDULE_STR},
        {RACE_2003_4_SCHEDULE_STR},
        {RACE_2015_11_SCHEDULE_STR},
        {RACE_2021_12_SCHEDULE_STR},
        {RACE_2022_4_SCHEDULE_STR},
        {RACE_2023_4_SCHEDULE_STR}
    ]}}"#
);

pub static RACE_TABLE_SCHEDULE: Lazy<RaceTable> = Lazy::new(|| RaceTable {
    races: vec![
        RACE_1950_1_SCHEDULE.clone(),
        RACE_2003_4_SCHEDULE.clone(),
        RACE_2015_11_SCHEDULE.clone(),
        RACE_2021_12_SCHEDULE.clone(),
        RACE_2022_4_SCHEDULE.clone(),
        RACE_2023_4_SCHEDULE.clone(),
    ],
});

// http://ergast.com/mrd/methods/qualifying/
// -----------------------------------------

pub const QUALIFYING_RESULT_2003_4_P1_STR: &str = formatcp!(
    r#"{{
    "number": "1",
    "position": "1",
    "Driver": {DRIVER_MICHAEL_STR},
    "Constructor": {CONSTRUCTOR_FERRARI_STR},
    "Q1": "1:22.327"
  }}"#
);

pub const QUALIFYING_RESULT_2003_4_P2_STR: &str = formatcp!(
    r#"{{
    "number": "4",
    "position": "2",
    "Driver": {DRIVER_RALF_STR},
    "Constructor": {CONSTRUCTOR_WILLIAMS_STR},
    "Q1": "1:22.341"
  }}"#
);

pub const QUALIFYING_RESULT_2003_4_P20_STR: &str = formatcp!(
    r#"{{
    "number": "19",
    "position": "20",
    "Driver": {DRIVER_JOS_STR},
    "Constructor": {CONSTRUCTOR_MINARDI_STR},
    "Q1": ""
  }}"#
);

pub const QUALIFYING_RESULT_2023_4_P1_STR: &str = formatcp!(
    r#"{{
    "number": "16",
    "position": "1",
    "Driver": {DRIVER_LECLERC_STR},
    "Constructor": {CONSTRUCTOR_FERRARI_STR},
    "Q1": "1:41.269",
    "Q2": "1:41.037",
    "Q3": "1:40.203"
  }}"#
);

pub const QUALIFYING_RESULT_2023_4_P2_STR: &str = formatcp!(
    r#"{{
    "number": "1",
    "position": "2",
    "Driver": {DRIVER_MAX_STR},
    "Constructor": {CONSTRUCTOR_RED_BULL_STR},
    "Q1": "1:41.398",
    "Q2": "1:40.822",
    "Q3": "1:40.391"
  }}"#
);

pub const QUALIFYING_RESULT_2023_4_P3_STR: &str = formatcp!(
    r#"{{
    "number": "11",
    "position": "3",
    "Driver": {DRIVER_PEREZ_STR},
    "Constructor": {CONSTRUCTOR_RED_BULL_STR},
    "Q1": "1:41.756",
    "Q2": "1:41.131",
    "Q3": "1:40.495"
  }}"#
);

pub const QUALIFYING_RESULT_2003_4_P1: Lazy<QualifyingResult> = Lazy::new(|| QualifyingResult {
    number: 1,
    position: 1,
    driver: DRIVER_MICHAEL.clone(),
    constructor: CONSTRUCTOR_FERRARI.clone(),
    q1: Some(QualifyingTime::from_m_s_ms(1, 22, 327)),
    q2: None,
    q3: None,
});

pub const QUALIFYING_RESULT_2003_4_P2: Lazy<QualifyingResult> = Lazy::new(|| QualifyingResult {
    number: 4,
    position: 2,
    driver: DRIVER_RALF.clone(),
    constructor: CONSTRUCTOR_WILLIAMS.clone(),
    q1: Some(QualifyingTime::from_m_s_ms(1, 22, 341)),
    q2: None,
    q3: None,
});

pub const QUALIFYING_RESULT_2003_4_P20: Lazy<QualifyingResult> = Lazy::new(|| QualifyingResult {
    number: 19,
    position: 20,
    driver: DRIVER_JOS.clone(),
    constructor: CONSTRUCTOR_MINARDI.clone(),
    q1: Some(QualifyingTime::NoTimeSet),
    q2: None,
    q3: None,
});

pub const QUALIFYING_RESULT_2023_4_P1: Lazy<QualifyingResult> = Lazy::new(|| QualifyingResult {
    number: 16,
    position: 1,
    driver: DRIVER_LECLERC.clone(),
    constructor: CONSTRUCTOR_FERRARI.clone(),
    q1: Some(QualifyingTime::from_m_s_ms(1, 41, 269)),
    q2: Some(QualifyingTime::from_m_s_ms(1, 41, 037)),
    q3: Some(QualifyingTime::from_m_s_ms(1, 40, 203)),
});

pub const QUALIFYING_RESULT_2023_4_P2: Lazy<QualifyingResult> = Lazy::new(|| QualifyingResult {
    number: 1,
    position: 2,
    driver: DRIVER_MAX.clone(),
    constructor: CONSTRUCTOR_RED_BULL.clone(),
    q1: Some(QualifyingTime::from_m_s_ms(1, 41, 398)),
    q2: Some(QualifyingTime::from_m_s_ms(1, 40, 822)),
    q3: Some(QualifyingTime::from_m_s_ms(1, 40, 391)),
});

pub const QUALIFYING_RESULT_2023_4_P3: Lazy<QualifyingResult> = Lazy::new(|| QualifyingResult {
    number: 11,
    position: 3,
    driver: DRIVER_PEREZ.clone(),
    constructor: CONSTRUCTOR_RED_BULL.clone(),
    q1: Some(QualifyingTime::from_m_s_ms(1, 41, 756)),
    q2: Some(QualifyingTime::from_m_s_ms(1, 41, 131)),
    q3: Some(QualifyingTime::from_m_s_ms(1, 40, 495)),
});

pub const RACE_2003_4_QUALIFYING_RESULTS_STR: &str = formatcp!(
    r#"{{
    {RACE_2003_4_STR},
    "QualifyingResults": [
        {QUALIFYING_RESULT_2003_4_P1_STR},
        {QUALIFYING_RESULT_2003_4_P2_STR},
        {QUALIFYING_RESULT_2003_4_P20_STR}
    ]
  }}"#
);

pub static RACE_2003_4_QUALIFYING_RESULTS: Lazy<Race> = Lazy::new(|| Race {
    qualifying_results: Some(vec![
        QUALIFYING_RESULT_2003_4_P1.clone(),
        QUALIFYING_RESULT_2003_4_P2.clone(),
        QUALIFYING_RESULT_2003_4_P20.clone(),
    ]),
    ..RACE_2003_4.clone()
});

pub const RACE_2023_4_QUALIFYING_RESULTS_STR: &str = formatcp!(
    r#"{{
    {RACE_2023_4_STR},
    "QualifyingResults": [
        {QUALIFYING_RESULT_2023_4_P1_STR},
        {QUALIFYING_RESULT_2023_4_P2_STR},
        {QUALIFYING_RESULT_2023_4_P3_STR}
    ]
  }}"#
);

pub static RACE_2023_4_QUALIFYING_RESULTS: Lazy<Race> = Lazy::new(|| Race {
    qualifying_results: Some(vec![
        QUALIFYING_RESULT_2023_4_P1.clone(),
        QUALIFYING_RESULT_2023_4_P2.clone(),
        QUALIFYING_RESULT_2023_4_P3.clone(),
    ]),
    ..RACE_2023_4.clone()
});

// http://ergast.com/mrd/methods/sprint/
// -------------------------------------

pub const SPRINT_RESULT_2023_4_P1_STR: &str = formatcp!(
    r#"{{
    "number": "11",
    "position": "1",
    "positionText": "1",
    "points": "8",
    "Driver": {DRIVER_PEREZ_STR},
    "Constructor": {CONSTRUCTOR_RED_BULL_STR},
    "grid": "2",
    "laps": "17",
    "status": "Finished",
    "Time": {{
        "millis": "1997667",
        "time": "33:17.667"
    }},
    "FastestLap": {{
        "lap": "11",
        "Time": {{
            "time": "1:43.616"
        }}
    }}
  }}"#
);

pub const SPRINT_RESULT_2023_4_P1: Lazy<SprintResult> = Lazy::new(|| SprintResult {
    number: 11,
    position: 1,
    position_text: "1".to_string(),
    points: 8,
    driver: DRIVER_PEREZ.clone(),
    constructor: CONSTRUCTOR_RED_BULL.clone(),
    grid: 2,
    laps: 17,
    status: "Finished".to_string(),
    time: Some(Time {
        millis: Some(1997667),
        time: "33:17.667".to_string(),
    }),
    fastest_lap: Some(FastestLap {
        rank: None,
        lap: 11,
        time: LapTime::from_m_s_ms(1, 43, 616),
        average_speed: None,
    }),
});

pub const RACE_2023_4_SPRINT_RESULTS_STR: &str = formatcp!(
    r#"{{
    {RACE_2023_4_STR},
    "SprintResults": [
        {SPRINT_RESULT_2023_4_P1_STR}
    ]
  }}"#
);

pub static RACE_2023_4_SPRINT_RESULTS: Lazy<Race> = Lazy::new(|| Race {
    sprint_results: Some(vec![SPRINT_RESULT_2023_4_P1.clone()]),
    ..RACE_2023_4.clone()
});

// http://ergast.com/mrd/methods/results/
// --------------------------------------

pub const RACE_RESULT_2003_4_P1_STR: &str = formatcp!(
    r#"{{
    "number": "1",
    "position": "1",
    "positionText": "1",
    "points": "10",
    "Driver": {DRIVER_MICHAEL_STR},
    "Constructor": {CONSTRUCTOR_FERRARI_STR},
    "grid": "1",
    "laps": "62",
    "status": "Finished",
    "Time": {{
        "millis": "5292058",
        "time": "1:28:12.058"
    }}
  }}"#
);

pub const RACE_RESULT_2003_4_P2_STR: &str = formatcp!(
    r#"{{
    "number": "6",
    "position": "2",
    "positionText": "2",
    "points": "8",
    "Driver": {DRIVER_KIMI_STR},
    "Constructor": {CONSTRUCTOR_MCLAREN_STR},
    "grid": "6",
    "laps": "62",
    "status": "Finished",
    "Time": {{
        "millis": "5293940",
        "time": "+1.882"
    }}
  }}"#
);

pub const RACE_RESULT_2003_4_P19_STR: &str = formatcp!(
    r#"{{
    "number": "18",
    "position": "19",
    "positionText": "R",
    "points": "0",
    "Driver": {DRIVER_WILSON_STR},
    "Constructor": {CONSTRUCTOR_MINARDI_STR},
    "grid": "18",
    "laps": "23",
    "status": "Fuel rig"
  }}"#
);

pub const RACE_RESULT_2023_4_P1_STR: &str = formatcp!(
    r#"{{
    "number": "11",
    "position": "1",
    "positionText": "1",
    "points": "25",
    "Driver": {DRIVER_PEREZ_STR},
    "Constructor": {CONSTRUCTOR_RED_BULL_STR},
    "grid": "3",
    "laps": "51",
    "status": "Finished",
    "Time": {{
        "millis": "5562436",
        "time": "1:32:42.436"
    }},
    "FastestLap": {{
        "rank": "5",
        "lap": "50",
        "Time": {{
            "time": "1:44.589"
        }},
        "AverageSpeed": {{
            "units": "kph",
            "speed": "206.625"
        }}
    }}
  }}"#
);

pub const RACE_RESULT_2023_4_P2_STR: &str = formatcp!(
    r#"{{
    "number": "1",
    "position": "2",
    "positionText": "2",
    "points": "18",
    "Driver": {DRIVER_MAX_STR},
    "Constructor": {CONSTRUCTOR_RED_BULL_STR},
    "grid": "2",
    "laps": "51",
    "status": "Finished",
    "Time": {{
        "millis": "5564573",
        "time": "+2.137"
    }},
    "FastestLap": {{
        "rank": "2",
        "lap": "51",
        "Time": {{
            "time": "1:44.232"
        }},
        "AverageSpeed": {{
            "units": "kph",
            "speed": "207.333"
        }}
    }}
  }}"#
);

pub const RACE_RESULT_2023_4_P20_STR: &str = formatcp!(
    r#"{{
    "number": "21",
    "position": "20",
    "positionText": "R",
    "points": "0",
    "Driver": {DRIVER_DE_VRIES_STR},
    "Constructor": {CONSTRUCTOR_ALPHA_TAURI_STR},
    "grid": "18",
    "laps": "9",
    "status": "Accident",
    "FastestLap": {{
        "rank": "20",
        "lap": "4",
        "Time": {{
            "time": "1:48.781"
        }},
        "AverageSpeed": {{
            "units": "kph",
            "speed": "198.663"
        }}
    }}
  }}"#
);

pub const RACE_RESULT_2003_4_P1: Lazy<RaceResult> = Lazy::new(|| RaceResult {
    number: 1,
    position: 1,
    position_text: "1".to_string(),
    points: 10,
    driver: DRIVER_MICHAEL.clone(),
    constructor: CONSTRUCTOR_FERRARI.clone(),
    grid: 1,
    laps: 62,
    status: "Finished".to_string(),
    time: Some(Time {
        millis: Some(5292058),
        time: "1:28:12.058".to_string(),
    }),
    fastest_lap: None,
});

pub const RACE_RESULT_2003_4_P2: Lazy<RaceResult> = Lazy::new(|| RaceResult {
    number: 6,
    position: 2,
    position_text: "2".to_string(),
    points: 8,
    driver: DRIVER_KIMI.clone(),
    constructor: CONSTRUCTOR_MCLAREN.clone(),
    grid: 6,
    laps: 62,
    status: "Finished".to_string(),
    time: Some(Time {
        millis: Some(5293940),
        time: "+1.882".to_string(),
    }),
    fastest_lap: None,
});

pub const RACE_RESULT_2003_4_P19: Lazy<RaceResult> = Lazy::new(|| RaceResult {
    number: 18,
    position: 19,
    position_text: "R".to_string(),
    points: 0,
    driver: DRIVER_WILSON.clone(),
    constructor: CONSTRUCTOR_MINARDI.clone(),
    grid: 18,
    laps: 23,
    status: "Fuel rig".to_string(),
    time: None,
    fastest_lap: None,
});

pub const RACE_RESULT_2023_4_P1: Lazy<RaceResult> = Lazy::new(|| RaceResult {
    number: 11,
    position: 1,
    position_text: "1".to_string(),
    points: 25,
    driver: DRIVER_PEREZ.clone(),
    constructor: CONSTRUCTOR_RED_BULL.clone(),
    grid: 3,
    laps: 51,
    status: "Finished".to_string(),
    time: Some(Time {
        millis: Some(5562436),
        time: "1:32:42.436".to_string(),
    }),
    fastest_lap: Some(FastestLap {
        rank: Some(5),
        lap: 50,
        time: LapTime::from_m_s_ms(1, 44, 589),
        average_speed: Some(AverageSpeed {
            units: SpeedUnits::Kph,
            speed: 206.625,
        }),
    }),
});

pub const RACE_RESULT_2023_4_P2: Lazy<RaceResult> = Lazy::new(|| RaceResult {
    number: 1,
    position: 2,
    position_text: "2".to_string(),
    points: 18,
    driver: DRIVER_MAX.clone(),
    constructor: CONSTRUCTOR_RED_BULL.clone(),
    grid: 2,
    laps: 51,
    status: "Finished".to_string(),
    time: Some(Time {
        millis: Some(5564573),
        time: "+2.137".to_string(),
    }),
    fastest_lap: Some(FastestLap {
        rank: Some(2),
        lap: 51,
        time: LapTime::from_m_s_ms(1, 44, 232),
        average_speed: Some(AverageSpeed {
            units: SpeedUnits::Kph,
            speed: 207.333,
        }),
    }),
});

pub const RACE_RESULT_2023_4_P20: Lazy<RaceResult> = Lazy::new(|| RaceResult {
    number: 21,
    position: 20,
    position_text: "R".to_string(),
    points: 0,
    driver: DRIVER_DE_VRIES.clone(),
    constructor: CONSTRUCTOR_ALPHA_TAURI.clone(),
    grid: 18,
    laps: 9,
    status: "Accident".to_string(),
    time: None,
    fastest_lap: Some(FastestLap {
        rank: Some(20),
        lap: 4,
        time: LapTime::from_m_s_ms(1, 48, 781),
        average_speed: Some(AverageSpeed {
            units: SpeedUnits::Kph,
            speed: 198.663,
        }),
    }),
});

pub const RACE_2003_4_RACE_RESULTS_STR: &str = formatcp!(
    r#"{{
    {RACE_2003_4_STR},
    "Results": [
        {RACE_RESULT_2003_4_P1_STR},
        {RACE_RESULT_2003_4_P2_STR},
        {RACE_RESULT_2003_4_P19_STR}
    ]
  }}"#
);

pub static RACE_2003_4_RACE_RESULTS: Lazy<Race> = Lazy::new(|| Race {
    results: Some(vec![
        RACE_RESULT_2003_4_P1.clone(),
        RACE_RESULT_2003_4_P2.clone(),
        RACE_RESULT_2003_4_P19.clone(),
    ]),
    ..RACE_2003_4.clone()
});

pub const RACE_2023_4_RACE_RESULTS_STR: &str = formatcp!(
    r#"{{
    {RACE_2023_4_STR},
    "Results": [
        {RACE_RESULT_2023_4_P1_STR},
        {RACE_RESULT_2023_4_P2_STR},
        {RACE_RESULT_2023_4_P20_STR}
    ]
  }}"#
);

pub static RACE_2023_4_RACE_RESULTS: Lazy<Race> = Lazy::new(|| Race {
    results: Some(vec![
        RACE_RESULT_2023_4_P1.clone(),
        RACE_RESULT_2023_4_P2.clone(),
        RACE_RESULT_2023_4_P20.clone(),
    ]),
    ..RACE_2023_4.clone()
});
