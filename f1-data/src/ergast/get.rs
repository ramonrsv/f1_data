use serde::de::DeserializeOwned;
use ureq;

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
    use crate::ergast::response::*;

    use super::*;
    use crate::ergast::tests::*;

    // http://ergast.com/mrd/methods/seasons/
    // --------------------------------------

    #[test]
    #[ignore]
    fn get_seasons() {
        let resp: Response = get_ergast_into_json("/seasons");

        assert!(resp.mr_data.season_table.is_some());
        assert_eq!(resp.mr_data.season_table.as_ref().unwrap().seasons.len(), 30);

        assert_eq!(resp.mr_data.season_table.as_ref().unwrap().seasons[0], *SEASON_1950);
        assert_eq!(resp.mr_data.season_table.as_ref().unwrap().seasons[29], *SEASON_1979);
    }

    // http://ergast.com/mrd/methods/drivers/
    // --------------------------------------

    fn verify_single_driver(driver_id: &str, driver: &Driver) {
        let resp: Response = get_ergast_into_json(&format!("/drivers/{driver_id}"));

        assert!(resp.mr_data.driver_table.is_some());
        assert_eq!(resp.mr_data.driver_table.as_ref().unwrap().drivers.len(), 1);

        assert_eq!(&resp.mr_data.driver_table.as_ref().unwrap().drivers[0], driver);
    }

    #[test]
    #[ignore]
    fn get_driver_all_fields_present() {
        verify_single_driver("alonso", &DRIVER_ALONSO);
    }

    #[test]
    #[ignore]
    fn get_driver_some_fields_missing() {
        verify_single_driver("abate", &DRIVER_ABATE);
    }

    // http://ergast.com/mrd/methods/constructors/
    // -------------------------------------------

    fn verify_single_constructor(constructor_id: &str, constructor: &Constructor) {
        let resp: Response = get_ergast_into_json(&format!("/constructors/{constructor_id}"));

        assert!(resp.mr_data.constructor_table.is_some());
        assert_eq!(resp.mr_data.constructor_table.as_ref().unwrap().constructors.len(), 1);

        assert_eq!(
            &resp.mr_data.constructor_table.as_ref().unwrap().constructors[0],
            constructor
        );
    }

    #[test]
    #[ignore]
    fn get_constructor() {
        verify_single_constructor("mclaren", &CONSTRUCTOR_MCLAREN);
        verify_single_constructor("ferrari", &CONSTRUCTOR_FERRARI);
    }

    // http://ergast.com/mrd/methods/circuits/
    // ---------------------------------------

    fn verify_single_circuit(circuit_id: &str, circuit: &Circuit) {
        let resp: Response = get_ergast_into_json(&format!("/circuits/{circuit_id}"));

        assert!(resp.mr_data.circuit_table.is_some());
        assert_eq!(resp.mr_data.circuit_table.as_ref().unwrap().circuits.len(), 1);

        assert_eq!(&resp.mr_data.circuit_table.as_ref().unwrap().circuits[0], circuit);
    }

    #[test]
    #[ignore]
    fn get_circuit() {
        verify_single_circuit("spa", &CIRCUIT_SPA);
        verify_single_circuit("silverstone", &CIRCUIT_SILVERSTONE);
    }
}
