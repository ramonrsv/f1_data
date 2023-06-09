use ureq;

use crate::ergast::resource::{Page, Resource};
use crate::ergast::response::Response;

/// An error that may occur while processing a [`Resource`](crate::ergast::resource::Resource)
/// HTTP request from the Ergast API, via the provided family of `get_*` methods. These may be
/// underlying HTTP errors, represented by [`Error::Http`], errors parsing the JSON response,
/// represented by [`Error::Parse`], or errors due to unmet restrictions imposed on the response,
/// e.g. a request by a method supporting only single-page responses resulted in a multi-page
/// response, represented by [`Error::MultiPage`].
#[derive(Debug)]
pub enum Error {
    /// Underlying HTTP error, passing through the [`ureq::Error`] returned by
    /// [`ureq::Request::call`].
    Http(Box<ureq::Error>),

    /// Error parsing the JSON response into a serializable type from
    /// [`response`](crate::ergast::response), presumably an error from [`serde_json`] but passing
    /// through the [`std::io::Error`] returned by [`ureq::Response::into_json`].
    Parse(std::io::Error),

    /// A request by a method supporting only single-page responses resulted in a multi-page one.
    MultiPage,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Self::Http(Box::new(error))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Parse(error)
    }
}

/// Performs a GET request to the Ergast API for a specific page of the argument specified
/// [`Resource`] and returns a [`Response`] with a single page, parsed from the JSON response, of a
/// possibly multi-page response. `Response::mr_data::pagination` can be used to check for
/// [`response::Pagination::is_last_page`](crate::ergast::response::Pagination::is_last_page) and
/// get [`response::Pagination::next_page`](crate::ergast::response::Pagination::next_page) to
/// request the following page of the response, via another call to this method.
///
/// This method performs no additional processing; it returns the top-level [`Response`] type that
/// is a direct representation of the full JSON response. It is expected that users will use one of
/// the other convenience `get_*` methods, e.g. `get_driver_info`, in almost all cases, but this
/// method is provided for maximum flexibility.
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_response_page, resource::{Filters, Page, Resource}};
///
/// let resp = get_response_page(Resource::SeasonList(Filters::none()), Page::with_limit(50)).unwrap();
///
/// let seasons = resp.mr_data.table.as_seasons().unwrap();
/// assert_eq!(seasons.len(), 50);
/// assert_eq!(seasons.first().unwrap().season, 1950);
/// assert_eq!(seasons.last().unwrap().season, 1999);
/// assert!(!resp.mr_data.pagination.is_last_page());
///
/// let resp = get_response_page(
///     Resource::SeasonList(Filters::none()),
///     resp.mr_data.pagination.next_page().unwrap().into(),
/// )
/// .unwrap();
///
/// let seasons = resp.mr_data.table.as_seasons().unwrap();
/// assert!(seasons.len() <= 50);
/// assert_eq!(seasons.first().unwrap().season, 2000);
/// assert!(resp.mr_data.pagination.is_last_page());
/// ```
pub fn get_response_page(resource: Resource, page: Page) -> Result<Response, Error> {
    Ok(ureq::request_url("GET", &resource.to_url_with(page))
        .call()?
        .into_json::<Response>()?)
}

/// Performs a GET request to the Ergast API for the argument specified [`Resource`] and returns a
/// single-page [`Response`], parsed from the JSON response. An [`Error::MultiPage`] is returned if
/// the requested [`Resource`] results in a multi-page response.
///
/// This method performs no additional processing, it returns the top-level [`Response`] type that
/// is a direct representation of the full JSON response. It is expected that users will use one of
/// the other convenience `get_*` methods, e.g. `get_driver_info`, in almost all cases, but this
/// method is provided for maximum flexibility.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::DriverID;
/// use f1_data::ergast::{get::get_response, resource::{Filters, Resource}};
///
/// let resp = get_response(Resource::DriverInfo(Filters {
///     driver_id: Some(DriverID::from("leclerc")),
///     ..Filters::none()
/// }))
/// .unwrap();
///
/// assert_eq!(resp.mr_data.table.as_drivers().unwrap()[0].given_name, "Charles".to_string());
/// ```
pub fn get_response(resource: Resource) -> Result<Response, Error> {
    single_page_or_err(get_response_page(resource, Page::default())?)
}

/// Performs a GET request to the Ergast API for the argument specified [`Resource`] and returns a
/// single-page [`Response`], parsed from the JSON response, with the maximum allowed pagination
/// limit. An [`Error::MultiPage`] is returned if the requested [`Resource`] results in a multi-page
/// response. This method is similar to [`get_response`] but allows for larger requests to be
/// accommodated in a single page.
///
/// This method performs no additional processing, it returns the top-level [`Response`] type that
/// is a direct representation of the full JSON response. It is expected that users will use one of
/// the other convenience `get_*` methods, e.g. `get_driver_info`, in almost all cases, but this
/// method is provided for maximum flexibility.
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_response_max_limit, resource::{Filters, Resource}};
///
/// let resp = get_response_max_limit(Resource::SeasonList(Filters::none())).unwrap();
///
/// let seasons = resp.mr_data.table.as_seasons().unwrap();
/// assert!(seasons.len() >= 74);
/// assert_eq!(seasons[0].season, 1950);
/// assert_eq!(seasons[73].season, 2023);
/// ```
pub fn get_response_max_limit(resource: Resource) -> Result<Response, Error> {
    single_page_or_err(get_response_page(resource, Page::with_max_limit())?)
}

/// Convert [`Response`] to `Result<Response, Error>`, enforcing that [`Response`] is single-page
fn single_page_or_err(response: Response) -> Result<Response, Error> {
    if response.mr_data.pagination.is_single_page() {
        Ok(response)
    } else {
        Err(Error::MultiPage)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ergast::{
            resource::{Filters, LapTimeFilters, PitStopFilters, Resource},
            response::*,
        },
        id::{RoundID, SeasonID},
    };

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
        let resp = get_response_max_limit(Resource::SeasonList(Filters::none())).unwrap();

        let seasons = resp.mr_data.table.as_seasons().unwrap();
        assert!(seasons.len() >= 74);

        assert_eq!(seasons[0], *SEASON_1950);
        assert_eq!(seasons[29], *SEASON_1979);
    }

    // Resource::DriverInfo
    // --------------------

    fn verify_single_driver(driver_id: &str, driver: &Driver) {
        let resp = get_response(Resource::DriverInfo(Filters {
            driver_id: Some(driver_id.into()),
            ..Filters::none()
        }))
        .unwrap();

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
        let resp = get_response(Resource::ConstructorInfo(Filters {
            constructor_id: Some(constructor_id.into()),
            ..Filters::none()
        }))
        .unwrap();

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
        let resp = get_response(Resource::CircuitInfo(Filters {
            circuit_id: Some(circuit_id.into()),
            ..Filters::none()
        }))
        .unwrap();

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

    fn verify_single_race_schedule(season: SeasonID, round: RoundID, race_schedule: &Race) {
        let resp = get_response(Resource::RaceSchedule(Filters {
            season: Some(season),
            round: Some(round),
            ..Filters::none()
        }))
        .unwrap();

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
        let resp = get_response(Resource::QualifyingResults(Filters {
            season: Some(2003),
            round: Some(4),
            ..Filters::none()
        }))
        .unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2003_4_QUALIFYING_RESULTS;

        assert_eq_race(actual, expected);

        let actual_results = actual.payload.as_qualifying_results().unwrap();
        let expected_results = expected.payload.as_qualifying_results().unwrap();

        assert_eq!(actual_results.len(), 20);

        assert_eq!(actual_results[0..1], expected_results[0..1]);
        assert_eq!(actual_results[19], expected_results[2]);
    }

    #[test]
    #[ignore]
    fn get_qualifying_results_2023_4() {
        let resp = get_response(Resource::QualifyingResults(Filters {
            season: Some(2023),
            round: Some(4),
            ..Filters::none()
        }))
        .unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_QUALIFYING_RESULTS;

        assert_eq_race(actual, expected);

        let actual_results = actual.payload.as_qualifying_results().unwrap();
        let expected_results = expected.payload.as_qualifying_results().unwrap();

        assert_eq!(actual_results.len(), 20);
        assert_eq!(actual_results[0..2], expected_results[0..2]);
    }

    // Resource::SprintResults
    // -----------------------

    #[test]
    #[ignore]
    fn get_sprint_results_2023_4() {
        let resp = get_response(Resource::SprintResults(Filters {
            season: Some(2023),
            round: Some(4),
            ..Filters::none()
        }))
        .unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_SPRINT_RESULTS;

        assert_eq_race(actual, expected);

        let actual_results = actual.payload.as_sprint_results().unwrap();
        let expected_results = expected.payload.as_sprint_results().unwrap();

        assert_eq!(actual_results.len(), 20);
        assert_eq!(actual_results[0], expected_results[0]);
    }

    #[test]
    #[ignore]
    fn get_sprint_results_no_sprint() {
        let resp = get_response(Resource::SprintResults(Filters {
            season: Some(2023),
            round: Some(1),
            ..Filters::none()
        }))
        .unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert!(races.is_empty());
    }

    // Resource::RaceResults
    // ---------------------

    #[test]
    #[ignore]
    fn get_race_results_2003_4() {
        let resp = get_response(Resource::RaceResults(Filters {
            season: Some(2003),
            round: Some(4),
            ..Filters::none()
        }))
        .unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2003_4_RACE_RESULTS;

        assert_eq_race(actual, expected);

        let actual_results = actual.payload.as_race_results().unwrap();
        let expected_results = expected.payload.as_race_results().unwrap();

        assert_eq!(actual_results.len(), 20);

        assert_eq!(actual_results[0..1], expected_results[0..1]);
        assert_eq!(actual_results[18], expected_results[2]);
    }

    #[test]
    #[ignore]
    fn get_race_results_2023_4() {
        let resp = get_response(Resource::RaceResults(Filters {
            season: Some(2023),
            round: Some(4),
            ..Filters::none()
        }))
        .unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_RACE_RESULTS;

        assert_eq_race(actual, expected);

        let actual_results = actual.payload.as_race_results().unwrap();
        let expected_results = expected.payload.as_race_results().unwrap();

        assert_eq!(actual_results.len(), 20);

        assert_eq!(actual_results[0..1], expected_results[0..1]);
        assert_eq!(actual_results[19], expected_results[2]);
    }

    // Resource::FinishingStatus
    // -------------------------

    #[test]
    #[ignore]
    fn get_finishing_status_2022() {
        let resp = get_response(Resource::FinishingStatus(Filters {
            season: Some(2022),
            ..Filters::none()
        }))
        .unwrap();

        let actual = resp.mr_data.table.as_status().unwrap();
        let expected = STATUS_TABLE_2022.as_status().unwrap();

        assert!(!actual.is_empty());
        assert_eq!(actual[0..expected.len()], expected[..]);
    }

    // Resource::LapTimes
    // ------------------

    #[test]
    #[ignore]
    fn get_lap_times_2023_4() {
        let resp = get_response_page(Resource::LapTimes(LapTimeFilters::new(2023, 4)), Page::default()).unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_LAPS;

        assert_eq_race(actual, expected);

        let actual_laps = actual.payload.as_laps().unwrap();
        let expected_laps = expected.payload.as_laps().unwrap();

        assert!(actual_laps.len() >= 2);
        assert_eq!(expected_laps.len(), 2);

        assert_eq!(actual_laps[0].timings[..2], expected_laps[0].timings[..]);
        assert_eq!(actual_laps[1].timings[..2], expected_laps[1].timings[..]);
    }

    // Resource::PitStops
    // ------------------

    #[test]
    #[ignore]
    fn get_pit_stops_2023_4() {
        let resp = get_response(Resource::PitStops(PitStopFilters::new(2023, 4))).unwrap();

        let races = resp.mr_data.table.as_races().unwrap();
        assert_eq!(races.len(), 1);

        let actual = &races[0];
        let expected = &RACE_2023_4_PIT_STOPS;

        assert_eq_race(actual, expected);

        let actual_pit_stops = actual.payload.as_pit_stops().unwrap();
        let expected_pit_stops = expected.payload.as_pit_stops().unwrap();

        assert!(actual_pit_stops.len() >= 2);
        assert_eq!(expected_pit_stops.len(), 2);

        assert_eq!(actual_pit_stops[8], expected_pit_stops[0]);
        assert_eq!(actual_pit_stops[11], expected_pit_stops[1]);
    }

    // Pagination
    // ----------

    #[test]
    #[ignore]
    fn pagination() {
        let req = Resource::RaceResults(Filters {
            season: Some(2023),
            round: Some(4),
            ..Filters::none()
        });

        let expected_results = RACE_2023_4_RACE_RESULTS.payload.as_race_results().unwrap();

        let get_actual_results = |resp: &Response| {
            let races = resp.mr_data.table.as_races().unwrap();
            let race_results = races[0].payload.as_race_results().unwrap();
            race_results.clone()
        };

        {
            let resp = get_response(req.clone()).unwrap();
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
            let mut resp = get_response_page(req.clone(), Page::with_limit(5)).unwrap();
            assert!(!resp.mr_data.pagination.is_last_page());

            let actual_results = get_actual_results(&resp);
            assert_eq!(actual_results[0..1], expected_results[0..1]);

            let mut current_offset: u32 = 0;

            while !resp.mr_data.pagination.is_last_page() {
                assert_eq!(resp.mr_data.pagination.limit, 5);
                assert_eq!(resp.mr_data.pagination.offset, current_offset);
                assert_eq!(resp.mr_data.pagination.total, 20);

                assert_eq!(get_actual_results(&resp).len(), 5);

                resp = get_response_page(req.clone(), resp.mr_data.pagination.next_page().unwrap().into()).unwrap();

                current_offset += 5;
            }

            let actual_results = get_actual_results(&resp);
            assert_eq!(actual_results[4], expected_results[2]);

            assert!(resp.mr_data.pagination.next_page().is_none());
        }
    }

    // Error::MultiPage
    // ----------------

    #[test]
    #[ignore]
    fn pagination_multi_page_error() {
        assert!(matches!(get_response(Resource::SeasonList(Filters::none())), Err(Error::MultiPage)));
    }
}
