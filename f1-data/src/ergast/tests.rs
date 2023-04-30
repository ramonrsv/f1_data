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

pub const CIRCUIT_TABLE_STR: &str = formatcp!(
    r#"{{
    "Circuits": [
        {CIRCUIT_SPA_STR},
        {CIRCUIT_SILVERSTONE_STR}
    ]}}"#
);

pub static CIRCUIT_TABLE: Lazy<CircuitTable> = Lazy::new(|| CircuitTable {
    circuits: vec![CIRCUIT_SPA.clone(), CIRCUIT_SILVERSTONE.clone()],
});
