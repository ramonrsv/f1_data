use serde::de::DeserializeOwned;
use serde_json::{Result, Value};
use ureq;

use crate::ergast::orm::{Driver, Location, Response, Season};

fn format_ergast_url(req: &str) -> String {
    format!("http://ergast.com/api/f1{}.json", req)
}

fn get_into_json<T: DeserializeOwned>(url: &str) -> T {
    ureq::get(url).call().unwrap().into_json().unwrap()
}

fn get_ergast_into_json<T: DeserializeOwned>(req: &str) -> T {
    get_into_json(&format_ergast_url(req))
}

#[cfg(test)]
mod tests {
    use crate::ergast::orm::{Circuit, Constructor};

    use super::*;

    // http://ergast.com/mrd/methods/seasons/
    // --------------------------------------

    #[test]
    fn get_seasons() {
        let resp: Response = get_ergast_into_json("/seasons");

        assert!(resp.mr_data.season_table.is_some());
        assert_eq!(
            resp.mr_data.season_table.as_ref().unwrap().seasons.len(),
            30
        );

        assert_eq!(
            resp.mr_data.season_table.as_ref().unwrap().seasons[0],
            Season {
                season: "1950".to_string(),
                url: "http://en.wikipedia.org/wiki/1950_Formula_One_season".to_string()
            }
        );

        assert_eq!(
            resp.mr_data.season_table.as_ref().unwrap().seasons[29],
            Season {
                season: "1979".to_string(),
                url: "http://en.wikipedia.org/wiki/1979_Formula_One_season".to_string()
            }
        );
    }

    // http://ergast.com/mrd/methods/drivers/
    // --------------------------------------

    #[test]
    fn get_driver_all_fields_present() {
        let resp: Response = get_ergast_into_json("/drivers/alonso");

        assert!(resp.mr_data.driver_table.is_some());
        assert_eq!(resp.mr_data.driver_table.as_ref().unwrap().drivers.len(), 1);

        assert_eq!(
            resp.mr_data.driver_table.as_ref().unwrap().drivers[0],
            Driver {
                driver_id: "alonso".to_string(),
                permanent_number: Some("14".to_string()),
                code: Some("ALO".to_string()),
                url: "http://en.wikipedia.org/wiki/Fernando_Alonso".to_string(),
                given_name: "Fernando".to_string(),
                family_name: "Alonso".to_string(),
                date_of_birth: "1981-07-29".to_string(),
                nationality: "Spanish".to_string()
            }
        );
    }

    #[test]
    fn get_driver_some_fields_missing() {
        let resp: Response = get_ergast_into_json("/drivers/abate");

        assert!(resp.mr_data.driver_table.is_some());
        assert_eq!(resp.mr_data.driver_table.as_ref().unwrap().drivers.len(), 1);

        assert_eq!(
            resp.mr_data.driver_table.as_ref().unwrap().drivers[0],
            Driver {
                driver_id: "abate".to_string(),
                permanent_number: None,
                code: None,
                url: "http://en.wikipedia.org/wiki/Carlo_Mario_Abate".to_string(),
                given_name: "Carlo".to_string(),
                family_name: "Abate".to_string(),
                date_of_birth: "1932-07-10".to_string(),
                nationality: "Italian".to_string()
            }
        );
    }

    // http://ergast.com/mrd/methods/constructors/
    // -------------------------------------------

    #[test]
    fn get_constructor() {
        let resp: Response = get_ergast_into_json("/constructors/mclaren");

        assert!(resp.mr_data.constructor_table.is_some());
        assert_eq!(
            resp.mr_data
                .constructor_table
                .as_ref()
                .unwrap()
                .constructors
                .len(),
            1
        );

        assert_eq!(
            resp.mr_data
                .constructor_table
                .as_ref()
                .unwrap()
                .constructors[0],
            Constructor {
                constructor_id: "mclaren".to_string(),
                url: "http://en.wikipedia.org/wiki/McLaren".to_string(),
                name: "McLaren".to_string(),
                nationality: "British".to_string(),
            }
        );
    }

    // http://ergast.com/mrd/methods/circuits/
    // ---------------------------------------

    #[test]
    fn get_circuit() {
        let resp: Response = get_ergast_into_json("/circuits/spa");

        assert!(resp.mr_data.circuit_table.is_some());
        assert_eq!(
            resp.mr_data.circuit_table.as_ref().unwrap().circuits.len(),
            1
        );

        assert_eq!(
            resp.mr_data.circuit_table.as_ref().unwrap().circuits[0],
            Circuit {
                circuit_id: "spa".to_string(),
                url: "http://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps".to_string(),
                circuit_name: "Circuit de Spa-Francorchamps".to_string(),
                location: Location {
                    lat: "50.4372".to_string(),
                    long: "5.97139".to_string(),
                    locality: "Spa".to_string(),
                    country: "Belgium".to_string()
                }
            }
        );
    }
}
