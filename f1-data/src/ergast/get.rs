use ureq;

use crate::{
    ergast::{
        resource::{Filters, Page, Resource},
        response::{Circuit, Constructor, Driver, Payload, Response, Season, Status, Table},
    },
    id::{CircuitID, ConstructorID, DriverID, SeasonID},
};

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
    /// A request resulted in a response that did not contain the expected [`Table`] variant.
    BadTableVariant,
    /// A request resulted in a response that did not contain the expected [`Payload`] variant.
    BadPayloadVariant,
    /// A request resulted in a response that did not contain any of the expected elements.
    NotFound,
    /// A request resulted in a response that contained more than the expected number of elements.
    TooMany,
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

impl From<Table> for Error {
    fn from(_: Table) -> Self {
        Self::BadTableVariant
    }
}

impl From<Payload> for Error {
    fn from(_: Payload) -> Self {
        Self::BadPayloadVariant
    }
}

pub type Result<T> = std::result::Result<T, Error>;

/// Performs a GET request to the Ergast API for a specific page of the argument specified
/// [`Resource`] and returns a [`Response`] with a single page, parsed from the JSON response, of a
/// possibly multi-page response. `Response::mr_data::pagination` can be used to check for
/// [`response::Pagination::is_last_page`](crate::ergast::response::Pagination::is_last_page) and
/// get [`response::Pagination::next_page`](crate::ergast::response::Pagination::next_page) to
/// request the following page of the response, via another call to this method.
///
/// This method performs no additional processing; it returns the top-level [`Response`] type that
/// is a direct representation of the full JSON response. It is expected that users will use one of
/// the other convenience `get_*` methods, e.g. [`get_seasons`], in almost all cases, but this
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
pub fn get_response_page(resource: Resource, page: Page) -> Result<Response> {
    ureq::request_url("GET", &resource.to_url_with(page))
        .call()?
        .into_json::<Response>()
        .map_err(|e| e.into())
}

/// Performs a GET request to the Ergast API for the argument specified [`Resource`] and returns a
/// single-page [`Response`], parsed from the JSON response. An [`Error::MultiPage`] is returned if
/// the requested [`Resource`] results in a multi-page response.
///
/// This method performs no additional processing, it returns the top-level [`Response`] type that
/// is a direct representation of the full JSON response. It is expected that users will use one of
/// the other convenience `get_*` methods, e.g. [`get_seasons`], in almost all cases, but this
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
pub fn get_response(resource: Resource) -> Result<Response> {
    get_response_page(resource, Page::default()).and_then(verify_is_single_page)
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
pub fn get_response_max_limit(resource: Resource) -> Result<Response> {
    get_response_page(resource, Page::with_max_limit()).and_then(verify_is_single_page)
}

/// Convert a [`Response`] to [`Result<Response>`], enforcing that [`Response`] is single-page, via
/// `response::Pagination::is_single_page`, and returning [`Error::MultiPage`] if it's not.
fn verify_is_single_page(response: Response) -> Result<Response> {
    if response.mr_data.pagination.is_single_page() {
        Ok(response)
    } else {
        Err(Error::MultiPage)
    }
}

/// Extract single element from [`Iterator`] into [`Result<T::Item>`], enforcing that there is only
/// one element in the [`Iterator`], returning [`Error::NotFound`] if the iterator contained no
/// elements, or [`Error::TooMany`] if it contained more than one.
fn verify_has_one_element_and_extract<T: Iterator>(mut sequence: T) -> Result<T::Item> {
    if let Some(val) = sequence.next() {
        if sequence.next().is_none() {
            Ok(val)
        } else {
            Err(Error::TooMany)
        }
    } else {
        Err(Error::NotFound)
    }
}

/// Performs a GET request to the Ergast API for [`Resource::SeasonList`], with the passed argument
/// [`Filters`], and return the resulting inner [`Season`]s from [`Table`] in `resp.mr_data.table`.
/// An [`Error::MultiPage`] is returned if `seasons` would not fit in a [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_seasons, resource::Filters};
///
/// let seasons = get_seasons(Filters::none()).unwrap();
/// assert!(!seasons.is_empty());
/// assert_eq!(seasons[0].season, 1950);
/// ```
pub fn get_seasons(filters: Filters) -> Result<Vec<Season>> {
    get_response_max_limit(Resource::SeasonList(filters))?
        .mr_data
        .table
        .into_seasons()
        .map_err(|e| e.into())
}

/// Performs a GET request to the Ergast API for a single [`Season`], identified by a [`SeasonID`],
/// from [`Resource::SeasonList`]. An [`Error::NotFound`] is returned if the season is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::get::{Error, get_season};
///
/// assert_eq!(get_season(1950).unwrap().season, 1950);
/// assert!(matches!(get_season(1940), Err(Error::NotFound)));
/// ```
pub fn get_season(season: SeasonID) -> Result<Season> {
    get_seasons(Filters::new().season(season))
        .map(|v| v.into_iter())
        .and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::DriverInfo`], with the passed argument
/// [`Filters`], and return the resulting inner [`Driver`]s from [`Table`] in `resp.mr_data.table`.
/// An [`Error::MultiPage`] is returned if `drivers` would not fit in a [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_drivers, resource::Filters};
///
/// let drivers = get_drivers(Filters::new().season(2022)).unwrap();
/// assert!(!drivers.is_empty());
/// assert_eq!(
///     drivers
///         .iter()
///         .find(|driver| driver.driver_id == "alonso".to_string())
///         .unwrap()
///         .given_name,
///     "Fernando".to_string()
/// );
/// ```
pub fn get_drivers(filters: Filters) -> Result<Vec<Driver>> {
    get_response_max_limit(Resource::DriverInfo(filters))?
        .mr_data
        .table
        .into_drivers()
        .map_err(|e| e.into())
}

/// Performs a GET request to the Ergast API for a single [`Driver`], identified by a [`DriverID`],
/// from [`Resource::DriverInfo`]. An [`Error::NotFound`] is returned if the driver is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::DriverID;
/// use f1_data::ergast::get::{Error, get_driver};
///
/// assert_eq!(get_driver(DriverID::from("alonso")).unwrap().given_name, "Fernando".to_string());
/// assert!(matches!(get_driver(DriverID::from("unknown")), Err(Error::NotFound)));
/// ```
pub fn get_driver(driver_id: DriverID) -> Result<Driver> {
    get_drivers(Filters::new().driver_id(driver_id))
        .map(|v| v.into_iter())
        .and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::ConstructorInfo`], with the passed
/// argument [`Filters`], and return the resulting inner [`Constructor`]s from [`Table`] in
/// `resp.mr_data.table`. An [`Error::MultiPage`] is returned if `constructors` would not fit in a
/// [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_constructors, resource::Filters};
///
/// let constructors = get_constructors(Filters::new().season(2022)).unwrap();
/// assert!(!constructors.is_empty());
/// assert_eq!(
///     constructors
///         .iter()
///         .find(|constructor| constructor.constructor_id == "ferrari".to_string())
///         .unwrap()
///         .name,
///     "Ferrari".to_string()
/// );
/// ```
pub fn get_constructors(filters: Filters) -> Result<Vec<Constructor>> {
    get_response_max_limit(Resource::ConstructorInfo(filters))?
        .mr_data
        .table
        .into_constructors()
        .map_err(|e| e.into())
}

/// Performs a GET request to the Ergast API for a single [`Constructor`], identified by a
/// [`ConstructorID`], from [`Resource::ConstructorInfo`]. An [`Error::NotFound`] is returned if the
/// constructor is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::ConstructorID;
/// use f1_data::ergast::get::{Error, get_constructor};
///
/// assert_eq!(get_constructor(ConstructorID::from("ferrari")).unwrap().name, "Ferrari".to_string());
/// assert!(matches!(get_constructor(ConstructorID::from("unknown")), Err(Error::NotFound)));
/// ```
pub fn get_constructor(constructor_id: ConstructorID) -> Result<Constructor> {
    get_constructors(Filters::new().constructor_id(constructor_id))
        .map(|v| v.into_iter())
        .and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::CircuitInfo`], with the passed argument
/// [`Filters`], and return the resulting inner [`Circuit`]s from [`Table`] in `resp.mr_data.table`.
/// An [`Error::MultiPage`] is returned if `circuits` would not fit in a [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_circuits, resource::Filters};
///
/// let circuits = get_circuits(Filters::new().season(2022)).unwrap();
/// assert!(!circuits.is_empty());
/// assert_eq!(
///     circuits
///         .iter()
///         .find(|circuit| circuit.circuit_id == "spa".to_string())
///         .unwrap()
///         .circuit_name,
///     "Circuit de Spa-Francorchamps".to_string()
/// );
/// ```
pub fn get_circuits(filters: Filters) -> Result<Vec<Circuit>> {
    get_response_max_limit(Resource::CircuitInfo(filters))?
        .mr_data
        .table
        .into_circuits()
        .map_err(|e| e.into())
}

/// Performs a GET request to the Ergast API for a single [`Circuit`], identified by a [`CircuitID`]
/// from [`Resource::CircuitInfo`]. An [`Error::NotFound`] is returned if the circuit is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::CircuitID;
/// use f1_data::ergast::get::{Error, get_circuit};
///
/// assert_eq!(
///     get_circuit(CircuitID::from("spa")).unwrap().circuit_name,
///     "Circuit de Spa-Francorchamps".to_string()
/// );
/// assert!(matches!(get_circuit(CircuitID::from("unknown")), Err(Error::NotFound)));
/// ```
pub fn get_circuit(circuit_id: CircuitID) -> Result<Circuit> {
    get_circuits(Filters::new().circuit_id(circuit_id))
        .map(|v| v.into_iter())
        .and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::FinishingStatus`], with the argument
/// [`Filters`], and return the resulting inner [`Status`]s from [`Table`] in `resp.mr_data.table`.
/// An [`Error::MultiPage`] is returned if `status` would not fit in a [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_statuses, resource::Filters};
///
/// let statuses = get_statuses(Filters::none()).unwrap();
/// assert!(!statuses.is_empty());
/// assert_eq!(
///     statuses
///         .iter()
///         .find(|status| status.status_id == 1)
///         .unwrap()
///         .status,
///     "Finished".to_string()
/// );
/// ```
pub fn get_statuses(filters: Filters) -> Result<Vec<Status>> {
    get_response_max_limit(Resource::FinishingStatus(filters))?
        .mr_data
        .table
        .into_status()
        .map_err(|e| e.into())
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

    fn assert_each_expected_in_actual<T: PartialEq + core::fmt::Debug>(
        actual_list: &[T],
        expected_list: &[T],
        min_actual_list_len: usize,
    ) {
        assert!(actual_list.len() >= min_actual_list_len);
        assert!(!expected_list.is_empty());

        for expected in expected_list {
            assert!(actual_list.iter().find(|actual| actual == &expected).is_some());
        }
    }

    fn assert_each_get_eq_expected<G, T>(get: G, expected_list: &[T])
    where
        G: Fn(&T) -> Result<T>,
        T: PartialEq + core::fmt::Debug,
    {
        assert!(!expected_list.is_empty());

        for expected in expected_list {
            assert_eq!(&get(expected).unwrap(), expected);
        }
    }

    fn assert_not_found<T>(result: Result<T>) {
        assert!(matches!(result, Err(Error::NotFound)));
    }

    // Resource::SeasonList
    // --------------------

    #[test]
    #[ignore]
    fn get_seasons() {
        assert_each_expected_in_actual(
            &super::get_seasons(Filters::none()).unwrap(),
            &SEASON_TABLE.as_seasons().unwrap(),
            74,
        );
    }

    #[test]
    #[ignore]
    fn get_season() {
        assert_each_get_eq_expected(|season| super::get_season(season.season), SEASON_TABLE.as_seasons().unwrap());
    }

    #[test]
    #[ignore]
    fn get_seasons_empty() {
        assert!(super::get_seasons(Filters::new().season(1949)).unwrap().is_empty());
    }

    #[test]
    #[ignore]
    fn get_season_error_not_found() {
        assert_not_found(super::get_season(1949));
    }

    // Resource::DriverInfo
    // --------------------

    #[test]
    #[ignore]
    fn get_drivers() {
        assert_each_expected_in_actual(
            &super::get_drivers(Filters::none()).unwrap(),
            &DRIVER_TABLE.as_drivers().unwrap(),
            857,
        );
    }

    #[test]
    #[ignore]
    fn get_driver() {
        assert_each_get_eq_expected(
            |driver| super::get_driver(driver.driver_id.clone()),
            DRIVER_TABLE.as_drivers().unwrap(),
        );
    }

    #[test]
    #[ignore]
    fn get_drivers_empty() {
        assert!(super::get_drivers(Filters::new().season(1949)).unwrap().is_empty());
    }

    #[test]
    #[ignore]
    fn get_driver_error_not_found() {
        assert_not_found(super::get_driver(DriverID::from("unknown")));
    }

    // Resource::ConstructorInfo
    // -------------------------

    #[test]
    #[ignore]
    fn get_constructors() {
        assert_each_expected_in_actual(
            &super::get_constructors(Filters::none()).unwrap(),
            &CONSTRUCTOR_TABLE.as_constructors().unwrap(),
            211,
        );
    }

    #[test]
    #[ignore]
    fn get_constructor() {
        assert_each_get_eq_expected(
            |constructor| super::get_constructor(constructor.constructor_id.clone()),
            CONSTRUCTOR_TABLE.as_constructors().unwrap(),
        );
    }

    #[test]
    #[ignore]
    fn get_constructors_empty() {
        assert!(super::get_constructors(Filters::new().season(1949)).unwrap().is_empty());
    }

    #[test]
    #[ignore]
    fn get_constructor_error_not_found() {
        assert_not_found(super::get_constructor(ConstructorID::from("unknown")));
    }

    // Resource::CircuitInfo
    // ---------------------

    #[test]
    #[ignore]
    fn get_circuits() {
        assert_each_expected_in_actual(
            &super::get_circuits(Filters::none()).unwrap(),
            &CIRCUIT_TABLE.as_circuits().unwrap(),
            77,
        );
    }

    #[test]
    #[ignore]
    fn get_circuit() {
        assert_each_get_eq_expected(
            |circuit| super::get_circuit(circuit.circuit_id.clone()),
            CIRCUIT_TABLE.as_circuits().unwrap(),
        );
    }

    #[test]
    #[ignore]
    fn get_circuits_empty() {
        assert!(super::get_circuits(Filters::new().season(1949)).unwrap().is_empty());
    }

    #[test]
    #[ignore]
    fn get_circuit_error_not_found() {
        assert_not_found(super::get_circuit(CircuitID::from("unknown")));
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
    fn get_statuses() {
        assert_each_expected_in_actual(
            &super::get_statuses(Filters::new().season(2022)).unwrap(),
            &STATUS_TABLE_2022.as_status().unwrap(),
            29,
        );
    }

    #[test]
    #[ignore]
    fn get_statuses_empty() {
        assert!(super::get_statuses(Filters::new().season(1949)).unwrap().is_empty());
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

    // Pagination, get_response_page, get_response, get_response_max_limit
    // -------------------------------------------------------------------

    #[test]
    #[ignore]
    fn get_response_single_page() {
        let resp = get_response(Resource::SeasonList(Filters::new().season(1950))).unwrap();

        let pagination = resp.mr_data.pagination;
        assert!(pagination.is_single_page());
        assert!(pagination.is_last_page());
        assert_eq!(pagination.limit, 30);
        assert_eq!(pagination.offset, 0);
        assert_eq!(pagination.total, 1);

        let seasons = resp.mr_data.table.as_seasons().unwrap();
        assert_eq!(seasons.len(), 1);
        assert_eq!(seasons[0], *SEASON_1950);
    }

    #[test]
    #[ignore]
    fn get_response_multi_page_error() {
        let resp = get_response(Resource::SeasonList(Filters::none()));
        assert!(matches!(resp, Err(Error::MultiPage)));
    }

    #[test]
    #[ignore]
    fn get_response_max_limit_single_page() {
        let resp = get_response_max_limit(Resource::SeasonList(Filters::none())).unwrap();

        let pagination = resp.mr_data.pagination;
        assert!(pagination.is_single_page());
        assert!(pagination.is_last_page());
        assert_eq!(pagination.limit, 1000);
        assert_eq!(pagination.offset, 0);
        assert!(pagination.total >= 74);

        let seasons = resp.mr_data.table.as_seasons().unwrap();
        assert_eq!(seasons[0], *SEASON_1950);
        assert_eq!(seasons[29], *SEASON_1979);
        assert_eq!(seasons[50], *SEASON_2000);
        assert_eq!(seasons[73], *SEASON_2023);
    }

    #[test]
    #[ignore]
    fn get_response_max_limit_multi_page_error() {
        let resp = get_response_max_limit(Resource::LapTimes(LapTimeFilters::new(2023, 1)));
        assert!(matches!(resp, Err(Error::MultiPage)));
    }

    #[test]
    #[ignore]
    fn get_response_page_multi_page() {
        let req = Resource::SeasonList(Filters::none());
        let page = Page::with_limit(5);

        let mut resp = get_response_page(req.clone(), page.clone()).unwrap();
        assert!(!resp.mr_data.pagination.is_last_page());

        let mut current_offset: u32 = 0;

        while !resp.mr_data.pagination.is_last_page() {
            let pagination = resp.mr_data.pagination;
            assert!(!pagination.is_single_page());
            assert_eq!(pagination.limit, page.limit());
            assert_eq!(pagination.offset, current_offset);
            assert!(pagination.total >= 74);

            let seasons = resp.mr_data.table.as_seasons().unwrap();
            assert_eq!(seasons.len(), page.limit() as usize);

            match current_offset {
                0 => assert_eq!(seasons[0], *SEASON_1950),
                25 => assert_eq!(seasons[4], *SEASON_1979),
                50 => assert_eq!(seasons[0], *SEASON_2000),
                70 => assert_eq!(seasons[3], *SEASON_2023),
                _ => (),
            }

            resp = get_response_page(req.clone(), pagination.next_page().unwrap().into()).unwrap();

            current_offset += page.limit();
        }

        let pagination = resp.mr_data.pagination;
        assert!(!pagination.is_single_page());
        assert!(pagination.is_last_page());
        assert_eq!(pagination.limit, page.limit());
        assert_eq!(pagination.offset, current_offset);
        assert!(pagination.total >= 74);

        let seasons = resp.mr_data.table.as_seasons().unwrap();
        assert_eq!(seasons.last().unwrap().season, 1950 + current_offset + (seasons.len() as u32) - 1);
    }
}
