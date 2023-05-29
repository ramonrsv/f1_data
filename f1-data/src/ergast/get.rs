use serde::de::DeserializeOwned;
use ureq;

use crate::ergast::resource::{Page, Resource};

fn get_into_json<T: DeserializeOwned>(request: Resource) -> T {
    ureq::request_url("GET", &request.to_url())
        .call()
        .unwrap()
        .into_json()
        .unwrap()
}

fn get_into_json_with<T: DeserializeOwned>(request: Resource, page: Page) -> T {
    ureq::request_url("GET", &request.to_url_with(page))
        .call()
        .unwrap()
        .into_json()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::ergast::resource::{Filters, Resource};
    use crate::ergast::response::*;

    use super::*;
    use crate::ergast::tests::*;

    fn assert_eq_race(left: &Race, right: &Race) {
        assert_eq!(left.season, right.season);
        assert_eq!(left.round, right.round);
        assert_eq!(left.url, right.url);
        assert_eq!(left.race_name, right.race_name);
        assert_eq!(left.circuit, right.circuit);
        assert_eq!(left.date, right.date);
        assert_eq!(left.time, right.time);
    }

    // Resource::SeasonList
    // --------------------

    #[test]
    #[ignore]
    fn get_seasons() {
        let resp: Response = get_into_json(Resource::SeasonList(Filters::none()));

        let seasons = resp.mr_data.table.as_seasons().unwrap();
        assert_eq!(seasons.len(), 30);

        assert_eq!(seasons[0], *SEASON_1950);
        assert_eq!(seasons[29], *SEASON_1979);
    }

    // Resource::DriverInfo
    // --------------------

    fn verify_single_driver(driver_id: &str, driver: &Driver) {
        let resp: Response = get_into_json(Resource::DriverInfo(Filters {
            driver_id: Some(driver_id.to_string()),
            ..Filters::none()
        }));

        let drivers = resp.mr_data.table.as_drivers().unwrap();
        assert_eq!(drivers.len(), 1);
        assert_eq!(&drivers[0], driver);
    }

    #[test]
    #[ignore]
    fn get_driver_some_fields_missing() {
        verify_single_driver("abate", &DRIVER_ABATE);
        verify_single_driver("michael_schumacher", &DRIVER_MICHAEL);
        verify_single_driver("verstappen", &DRIVER_JOS);
        verify_single_driver("ralf_schumacher", &DRIVER_RALF);
        verify_single_driver("wilson", &DRIVER_WILSON);
    }

    #[test]
    #[ignore]
    fn get_driver_all_fields_present() {
        verify_single_driver("raikkonen", &DRIVER_KIMI);
        verify_single_driver("alonso", &DRIVER_ALONSO);
        verify_single_driver("perez", &DRIVER_PEREZ);
        verify_single_driver("de_vries", &DRIVER_DE_VRIES);
        verify_single_driver("max_verstappen", &DRIVER_MAX);
        verify_single_driver("leclerc", &DRIVER_LECLERC);
    }

    // Resource::ConstructorInfo
    // -------------------------

    fn verify_single_constructor(constructor_id: &str, constructor: &Constructor) {
        let resp: Response = get_into_json(Resource::ConstructorInfo(Filters {
            constructor_id: Some(constructor_id.to_string()),
            ..Filters::none()
        }));

        let constructors = resp.mr_data.table.as_constructors().unwrap();
        assert_eq!(constructors.len(), 1);
        assert_eq!(&constructors[0], constructor);
    }

    #[test]
    #[ignore]
    fn get_constructor() {
        verify_single_constructor("mclaren", &CONSTRUCTOR_MCLAREN);
        verify_single_constructor("ferrari", &CONSTRUCTOR_FERRARI);
        verify_single_constructor("williams", &CONSTRUCTOR_WILLIAMS);
        verify_single_constructor("minardi", &CONSTRUCTOR_MINARDI);
        verify_single_constructor("alphatauri", &CONSTRUCTOR_ALPHA_TAURI);
        verify_single_constructor("red_bull", &CONSTRUCTOR_RED_BULL);
    }

    // Resource::CircuitInfo
    // ---------------------

    fn verify_single_circuit(circuit_id: &str, circuit: &Circuit) {
        let resp: Response = get_into_json(Resource::CircuitInfo(Filters {
            circuit_id: Some(circuit_id.to_string()),
            ..Filters::none()
        }));

        let circuits = resp.mr_data.table.as_circuits().unwrap();
        assert_eq!(circuits.len(), 1);
        assert_eq!(&circuits[0], circuit);
    }

    #[test]
    #[ignore]
    fn get_circuit() {
        verify_single_circuit("spa", &CIRCUIT_SPA);
        verify_single_circuit("silverstone", &CIRCUIT_SILVERSTONE);
        verify_single_circuit("imola", &CIRCUIT_IMOLA);
        verify_single_circuit("baku", &CIRCUIT_BAKU);
    }

    // Resource::RaceSchedule
    // ----------------------

    fn verify_single_race_schedule(year: u32, round: u32, race_schedule: &Race) {
        let resp: Response = get_into_json(Resource::RaceSchedule(Filters {
            year: Some(year),
            round: Some(round),
            ..Filters::none()
        }));

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);
        assert_eq!(&races[0], race_schedule);
    }

    #[test]
    #[ignore]
    fn get_race_schedule() {
        verify_single_race_schedule(1950, 1, &RACE_1950_1_SCHEDULE);
        verify_single_race_schedule(2003, 4, &RACE_2003_4_SCHEDULE);
        verify_single_race_schedule(2015, 11, &RACE_2015_11_SCHEDULE);
        verify_single_race_schedule(2021, 12, &RACE_2021_12_SCHEDULE);
        verify_single_race_schedule(2022, 4, &RACE_2022_4_SCHEDULE);
        verify_single_race_schedule(2023, 4, &RACE_2023_4_SCHEDULE);
    }

    // Resource::QualifyingResults
    // ---------------------------

    #[test]
    #[ignore]
    fn get_qualifying_results_2003_4() {
        let resp: Response = get_into_json(Resource::QualifyingResults(Filters {
            year: Some(2003),
            round: Some(4),
            ..Filters::none()
        }));

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2003_4_QUALIFYING_RESULTS;

        assert_eq_race(actual, expected);

        let SessionResults::QualifyingResults(actual_results) = &actual.results else {
            panic!("Expected QualifyingResults variant")
        };

        let SessionResults::QualifyingResults(expected_results) = &expected.results else {
            panic!("Expected QualifyingResults variant")
        };

        assert_eq!(actual_results.len(), 20);

        assert_eq!(actual_results[0..1], expected_results[0..1]);
        assert_eq!(actual_results[19], expected_results[2]);
    }

    #[test]
    #[ignore]
    fn get_qualifying_results_2023_4() {
        let resp: Response = get_into_json(Resource::QualifyingResults(Filters {
            year: Some(2023),
            round: Some(4),
            ..Filters::none()
        }));

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_QUALIFYING_RESULTS;

        assert_eq_race(actual, expected);

        let SessionResults::QualifyingResults(actual_results) = &actual.results else {
            panic!("Expected QualifyingResults variant")
        };

        let SessionResults::QualifyingResults(expected_results) = &expected.results else {
            panic!("Expected QualifyingResults variant")
        };

        assert_eq!(actual_results.len(), 20);
        assert_eq!(actual_results[0..2], expected_results[0..2]);
    }

    // Resource::SprintResults
    // -----------------------

    #[test]
    #[ignore]
    fn get_sprint_results_2023_4() {
        let resp: Response = get_into_json(Resource::SprintResults(Filters {
            year: Some(2023),
            round: Some(4),
            ..Filters::none()
        }));

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_SPRINT_RESULTS;

        assert_eq_race(actual, expected);

        let SessionResults::SprintResults(actual_results) = &actual.results else {
            panic!("Expected SprintResults variant")
        };

        let SessionResults::SprintResults(expected_results) = &expected.results else {
            panic!("Expected SprintResults variant")
        };

        assert_eq!(actual_results.len(), 20);
        assert_eq!(actual_results[0], expected_results[0]);
    }

    #[test]
    #[ignore]
    fn get_sprint_results_no_sprint() {
        let resp: Response = get_into_json(Resource::SprintResults(Filters {
            year: Some(2023),
            round: Some(1),
            ..Filters::none()
        }));

        let races = resp.mr_data.table.as_races().unwrap();
        assert!(races.is_empty());
    }

    // Resource::RaceResults
    // ---------------------

    #[test]
    #[ignore]
    fn get_race_results_2003_4() {
        let resp: Response = get_into_json(Resource::RaceResults(Filters {
            year: Some(2003),
            round: Some(4),
            ..Filters::none()
        }));

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2003_4_RACE_RESULTS;

        assert_eq_race(actual, expected);

        let SessionResults::RaceResults(actual_results) = &actual.results else {
            panic!("Expected RaceResults variant")
        };

        let SessionResults::RaceResults(expected_results) = &expected.results else {
            panic!("Expected RaceResults variant")
        };

        assert_eq!(actual_results.len(), 20);

        assert_eq!(actual_results[0..1], expected_results[0..1]);
        assert_eq!(actual_results[18], expected_results[2]);
    }

    #[test]
    #[ignore]
    fn get_race_results_2023_4() {
        let resp: Response = get_into_json(Resource::RaceResults(Filters {
            year: Some(2023),
            round: Some(4),
            ..Filters::none()
        }));

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_RACE_RESULTS;

        assert_eq_race(actual, expected);

        let SessionResults::RaceResults(actual_results) = &actual.results else {
            panic!("Expected RaceResults variant")
        };

        let SessionResults::RaceResults(expected_results) = &expected.results else {
            panic!("Expected RaceResults variant")
        };

        assert_eq!(actual_results.len(), 20);

        assert_eq!(actual_results[0..1], expected_results[0..1]);
        assert_eq!(actual_results[19], expected_results[2]);
    }

    // Resource::FinishingStatus
    // -------------------------

    #[test]
    fn get_finishing_status_2022() {
        let resp: Response = get_into_json(Resource::FinishingStatus(Filters {
            year: Some(2022),
            ..Filters::none()
        }));

        let actual = resp.mr_data.table.as_status().unwrap();
        let expected = STATUS_TABLE_2022.as_status().unwrap();

        assert!(!actual.is_empty());
        assert_eq!(actual[0..expected.len()], expected[..]);
    }

    // Pagination
    // ----------

    #[test]
    #[ignore]
    fn pagination() {
        let req = Resource::RaceResults(Filters {
            year: Some(2023),
            round: Some(4),
            ..Filters::none()
        });

        let SessionResults::RaceResults(expected_results) = &RACE_2023_4_RACE_RESULTS.results else {
            panic!("Expected RaceResults variant")
        };

        let get_actual_results = |resp: &Response| {
            let races = resp.mr_data.table.as_races().unwrap();

            let SessionResults::RaceResults(race_results) = &races[0].results else {
                panic!("Expected RaceResults variant")
            };

            race_results.clone()
        };

        {
            let resp: Response = get_into_json(req.clone());
            assert!(resp.mr_data.pagination.is_last_page());
            assert_eq!(resp.mr_data.pagination.limit, 30);
            assert_eq!(resp.mr_data.pagination.offset, 0);
            assert_eq!(resp.mr_data.pagination.total, 20);

            let actual_results = get_actual_results(&resp);

            assert_eq!(actual_results.len(), 20);
            assert_eq!(actual_results[0..1], expected_results[0..1]);
            assert_eq!(actual_results[19], expected_results[2]);
        }

        {
            let mut resp: Response = get_into_json_with(req.clone(), Page::with_limit(5));
            assert!(!resp.mr_data.pagination.is_last_page());

            let actual_results = get_actual_results(&resp);
            assert_eq!(actual_results[0..1], expected_results[0..1]);

            let mut current_offset: u32 = 0;

            while !resp.mr_data.pagination.is_last_page() {
                assert_eq!(resp.mr_data.pagination.limit, 5);
                assert_eq!(resp.mr_data.pagination.offset, current_offset);
                assert_eq!(resp.mr_data.pagination.total, 20);

                assert_eq!(get_actual_results(&resp).len(), 5);

                resp = get_into_json_with(req.clone(), resp.mr_data.pagination.next_page().unwrap().into());

                current_offset += 5;
            }

            let actual_results = get_actual_results(&resp);
            assert_eq!(actual_results[4], expected_results[2]);

            assert!(resp.mr_data.pagination.next_page().is_none());
        }
    }
}
