use serde_json;
use ureq;

use crate::{
    ergast::{
        error::{Error, Result},
        resource::{Filters, LapTimeFilters, Page, PitStopFilters, Resource},
        response::{
            Circuit, Constructor, Driver, Lap, Payload, PitStop, QualifyingResult, Race, RaceResult, Response,
            Schedule, Season, SprintResult, Status, Timing,
        },
        time::Duration,
    },
    id::{CircuitID, ConstructorID, DriverID, RaceID, SeasonID},
};

#[cfg(doc)]
use crate::ergast::response::{Pagination, Table};

/// Performs a GET request to the Ergast API for a specific page of the argument specified
/// [`Resource`] and returns a [`Response`] with a single page, parsed from the JSON response, of a
/// possibly multi-page response. `Response::mr_data::pagination` can be used to check for
/// [`Pagination::is_last_page`] and get [`Pagination::next_page`] to request the following page of
/// the response, via another call to this method.
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
/// let resp = get_response_page(&Resource::SeasonList(Filters::none()), Page::with_limit(50)).unwrap();
///
/// let seasons = resp.mr_data.table.as_seasons().unwrap();
/// assert_eq!(seasons.len(), 50);
/// assert_eq!(seasons.first().unwrap().season, 1950);
/// assert_eq!(seasons.last().unwrap().season, 1999);
/// assert!(!resp.mr_data.pagination.is_last_page());
///
/// let resp = get_response_page(
///     &Resource::SeasonList(Filters::none()),
///     resp.mr_data.pagination.next_page().unwrap().into(),
/// )
/// .unwrap();
///
/// let seasons = resp.mr_data.table.as_seasons().unwrap();
/// assert!(seasons.len() <= 50);
/// assert_eq!(seasons.first().unwrap().season, 2000);
/// assert!(resp.mr_data.pagination.is_last_page());
/// ```
pub fn get_response_page(resource: &Resource, page: Page) -> Result<Response> {
    ureq::request_url("GET", &resource.to_url_with(page))
        .call()
        .map_err(into)
        .map(ureq::Response::into_reader)
        .and_then(|reader| serde_json::from_reader(reader).map_err(into))
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
/// let resp = get_response(&Resource::DriverInfo(Filters {
///     driver_id: Some(DriverID::from("leclerc")),
///     ..Filters::none()
/// }))
/// .unwrap();
///
/// assert_eq!(resp.mr_data.table.as_drivers().unwrap()[0].given_name, "Charles".to_string());
/// ```
pub fn get_response(resource: &Resource) -> Result<Response> {
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
/// let resp = get_response_max_limit(&Resource::SeasonList(Filters::none())).unwrap();
///
/// let seasons = resp.mr_data.table.as_seasons().unwrap();
/// assert!(seasons.len() >= 74);
/// assert_eq!(seasons[0].season, 1950);
/// assert_eq!(seasons[73].season, 2023);
/// ```
pub fn get_response_max_limit(resource: &Resource) -> Result<Response> {
    get_response_page(resource, Page::with_max_limit()).and_then(verify_is_single_page)
}

/// Performs a GET request to the Ergast API for [`Resource::SeasonList`], with the argument
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
    get_response_max_limit(&Resource::SeasonList(filters))?
        .mr_data
        .table
        .into_seasons()
        .map_err(into)
}

/// Performs a GET request to the Ergast API for a single [`Season`], identified by a [`SeasonID`],
/// from [`Resource::SeasonList`]. An [`Error::NotFound`] is returned if the season is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{error::Error, get::get_season};
///
/// assert_eq!(get_season(1950).unwrap().season, 1950);
/// assert!(matches!(get_season(1940), Err(Error::NotFound)));
/// ```
pub fn get_season(season: SeasonID) -> Result<Season> {
    get_seasons(Filters::new().season(season)).and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::DriverInfo`], with the argument
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
    get_response_max_limit(&Resource::DriverInfo(filters))?
        .mr_data
        .table
        .into_drivers()
        .map_err(into)
}

/// Performs a GET request to the Ergast API for a single [`Driver`], identified by a [`DriverID`],
/// from [`Resource::DriverInfo`]. An [`Error::NotFound`] is returned if the driver is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::DriverID;
/// use f1_data::ergast::{error::Error, get::get_driver};
///
/// assert_eq!(get_driver(DriverID::from("alonso")).unwrap().given_name, "Fernando".to_string());
/// assert!(matches!(get_driver(DriverID::from("unknown")), Err(Error::NotFound)));
/// ```
pub fn get_driver(driver_id: DriverID) -> Result<Driver> {
    get_drivers(Filters::new().driver_id(driver_id)).and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::ConstructorInfo`], with the argument
/// [`Filters`], and return the resulting inner [`Constructor`]s from [`Table`] in
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
    get_response_max_limit(&Resource::ConstructorInfo(filters))?
        .mr_data
        .table
        .into_constructors()
        .map_err(into)
}

/// Performs a GET request to the Ergast API for a single [`Constructor`], identified by a
/// [`ConstructorID`], from [`Resource::ConstructorInfo`]. An [`Error::NotFound`] is returned if the
/// constructor is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::ConstructorID;
/// use f1_data::ergast::{error::Error, get::get_constructor};
///
/// assert_eq!(get_constructor(ConstructorID::from("ferrari")).unwrap().name, "Ferrari".to_string());
/// assert!(matches!(get_constructor(ConstructorID::from("unknown")), Err(Error::NotFound)));
/// ```
pub fn get_constructor(constructor_id: ConstructorID) -> Result<Constructor> {
    get_constructors(Filters::new().constructor_id(constructor_id)).and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::CircuitInfo`], with the argument
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
    get_response_max_limit(&Resource::CircuitInfo(filters))?
        .mr_data
        .table
        .into_circuits()
        .map_err(into)
}

/// Performs a GET request to the Ergast API for a single [`Circuit`], identified by a [`CircuitID`]
/// from [`Resource::CircuitInfo`]. An [`Error::NotFound`] is returned if the circuit is not found.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::CircuitID;
/// use f1_data::ergast::{error::Error, get::get_circuit};
///
/// assert_eq!(
///     get_circuit(CircuitID::from("spa")).unwrap().circuit_name,
///     "Circuit de Spa-Francorchamps".to_string()
/// );
/// assert!(matches!(get_circuit(CircuitID::from("unknown")), Err(Error::NotFound)));
/// ```
pub fn get_circuit(circuit_id: CircuitID) -> Result<Circuit> {
    get_circuits(Filters::new().circuit_id(circuit_id)).and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for [`Resource::RaceSchedule`], with the argument
/// [`Filters`], and returns a sequence of [`Race<Schedule>`]s processed from the inner [`Race`]s
/// from [`Table`]. An [`Error::MultiPage`] is returned if the results would not fit in a
/// [`Page::with_max_limit`].
///
/// **Note:** The returned [`Race<Schedule>`]s contain all the common fields in a [`Race`], e.g.
/// [`Race::season`], [`Race::round`], [`Race::race_name`], etc., so this function can be used to
/// obtain general information about race weekend events, e.g. a list of rounds for a season.
///
/// **Note:** Since more than [`Page::MAX_LIMIT`] races have taken place in the history of F1,
/// calling this function without any filters will return [`Error::MultiPage`]. As such, it is
/// necessary to pass some filters, e.g. [`Filters::season`], [`Filters::driver_id`], etc.
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_race_schedules, resource::Filters, time::macros::{date, time}};
///
/// let races = get_race_schedules(Filters::new().season(2022)).unwrap();
/// assert_eq!(races.len(), 22);
///
/// let sprint_count = races.iter().filter(|race| race.schedule().sprint.is_some()).count();
/// assert_eq!(sprint_count, 3);
///
/// assert_eq!(races[0].race_name, "Bahrain Grand Prix");
/// assert_eq!(races[0].date, date!(2022 - 03 - 20));
/// assert_eq!(races[0].time.unwrap(), time!(15:00:00));
/// ```
pub fn get_race_schedules(filters: Filters) -> Result<Vec<Race<Schedule>>> {
    get_response_max_limit(&Resource::RaceSchedule(filters))?
        .mr_data
        .table
        .into_races()?
        .into_iter()
        .map(|race| race.try_map(|payload| payload.into_schedule().map_err(into)))
        .collect()
}

/// Performs a GET request to the Ergast API for a single [`Race<Schedule>`] from
/// [`Resource::RaceSchedule`], identified by a [`RaceID`], a combination of a [`Race::season`] and
/// [`Race::round`]. An [`Error::NotFound`] is returned if the race is not found.
///
/// **Note:** The returned [`Race<Schedule>`] contains all the common fields in a [`Race`], e.g.
/// [`Race::race_name`], [`Race::circuit`], etc., so this function can be used to obtain general
/// information about a race weekend event.
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::RaceID;
/// use f1_data::ergast::{get::get_race_schedule, resource::Filters, time::macros::{date, time}};
///
/// let race = get_race_schedule(RaceID::from(2022, 1)).unwrap();
///
/// assert_eq!(race.race_name, "Bahrain Grand Prix");
/// assert_eq!(race.date, date!(2022 - 03 - 20));
/// assert_eq!(race.time.unwrap(), time!(15:00:00));
///
/// let schedule = race.schedule();
/// assert!(
///     schedule.first_practice.is_some()
///         && schedule.second_practice.is_some()
///         && schedule.third_practice.is_some()
///         && schedule.qualifying.is_some()
/// );
/// ```
pub fn get_race_schedule(race_id: RaceID) -> Result<Race<Schedule>> {
    get_race_schedules(Filters::new().season(race_id.season).round(race_id.round))
        .and_then(verify_has_one_element_and_extract)
}

/// Inner type of a [`Payload`] variant for a [`SessionResult`] type, e.g. the inner type of the
/// [`Payload::RaceResults`] variant is [`Vec<RaceResult>`].
type Inner<T> = Vec<T>;

/// The [`SessionResult`] trait allows the generic handling of all session result types, resource
/// requests, and extraction of the corresponding variant from [`Payload`].
///
/// For example, [`RaceResult`]s are requested via [`Resource::RaceResults`], and the response can
/// be extracted from the [`Payload::RaceResults`] variant.
///
/// The trait is implemented for [`QualifyingResult`], [`SprintResult`], and [`RaceResult`].
pub trait SessionResult
where
    Self: Sized,
{
    /// Wrap a [`Filters`] with the corresponding [`Resource`] variant for this [`SessionResult`].
    fn to_resource(filters: Filters) -> Resource;

    /// Extract the value from the corresponding [`Payload`] variant for this [`SessionResult`].
    fn try_inner_from(payload: Payload) -> Result<Inner<Self>>;
}

impl SessionResult for QualifyingResult {
    fn to_resource(filters: Filters) -> Resource {
        Resource::QualifyingResults(filters)
    }

    fn try_inner_from(payload: Payload) -> Result<Inner<Self>> {
        payload.into_qualifying_results().map_err(into)
    }
}

impl SessionResult for SprintResult {
    fn to_resource(filters: Filters) -> Resource {
        Resource::SprintResults(filters)
    }

    fn try_inner_from(payload: Payload) -> Result<Inner<Self>> {
        payload.into_sprint_results().map_err(into)
    }
}

impl SessionResult for RaceResult {
    fn to_resource(filters: Filters) -> Resource {
        Resource::RaceResults(filters)
    }

    fn try_inner_from(payload: Payload) -> Result<Inner<Self>> {
        payload.into_race_results().map_err(into)
    }
}

/// Performs a GET request to the Ergast API for the [`Resource`] corresponding to the requested
/// [`SessionResult`], with the argument [`Filters`], and returns a sequence of [`Race`]s, each with
/// a sequence of [`SessionResult`]s, processed from the inner [`Race`]s from [`Table`]. An
/// [`Error::MultiPage`] is returned if the results would not fit in a [`Page::with_max_limit`].
///
/// For example, [`get_session_results::<RaceResult>`] will perform a GET request to the Ergast API
/// for [`Resource::RaceResults`], and return a sequence of [`Race<Vec<RaceResult>>`], where the
/// [`Payload`] variant [`Payload::RaceResults`] has already been extracted and processed into
/// [`Race<Vec<RaceResult>>`], obviating the need to perform error checking and extraction of the
/// expected variants.
///
/// This function returns a sequence of [`SessionResult`]s for each of a sequence of [`Race`]s,
/// i.e. it returns [`Vec<Race<Vec<T>>>`]. If a single [`Race`] is expected in the response, or a
/// single [`SessionResult`] per [`Race`], or other, consider using one of the other methods with
/// the desired processing: [`get_session_results_for_event`], [`get_session_result_for_events`], or
/// [`get_session_result`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::ConstructorID;
/// use f1_data::ergast::{
///     get::get_session_results,
///     resource::Filters,
///     response::{Points, RaceResult, SprintResult},
/// };
///
/// let race_points = get_session_results::<RaceResult>(
///     Filters::new()
///         .season(2021)
///         .constructor_id(ConstructorID::from("red_bull")),
/// )
/// .unwrap()
/// .iter()
/// .map(|r| r.race_results().iter().map(|r| r.points).sum::<Points>())
/// .sum::<Points>();
///
/// let sprint_points = get_session_results::<SprintResult>(
///     Filters::new()
///         .season(2021)
///         .constructor_id(ConstructorID::from("red_bull")),
/// )
/// .unwrap()
/// .iter()
/// .map(|s| s.sprint_results().iter().map(|r| r.points).sum::<Points>())
/// .sum::<Points>();
///
/// assert_eq!(race_points + sprint_points, 585.5);
/// ```
pub fn get_session_results<T: SessionResult>(filters: Filters) -> Result<Vec<Race<Vec<T>>>> {
    get_response_max_limit(&T::to_resource(filters))?
        .mr_data
        .table
        .into_races()?
        .into_iter()
        .map(|race| race.try_map(|payload| T::try_inner_from(payload)))
        .collect()
}

/// Performs a GET request to the Ergast API for the [`Resource`] corresponding to the requested
/// [`SessionResult`], with the argument [`Filters`], and returns a sequence of [`SessionResult`]s
/// for a single [`Race`], processed from the inner [`Race`]s from [`Table`]. An
/// [`Error::MultiPage`] is returned if the results would not fit in a [`Page::with_max_limit`].
/// An [`Error::NotFound`] or [`Error::TooMany`] is returned if the expected number of [`Race`]s and
/// [`SessionResult`]s per [`Race`] are not found in the response.
///
/// For example, [`get_session_results_for_event::<RaceResult>`] will perform a GET request to the
/// Ergast API for [`Resource::RaceResults`], and return a single [`Race<Vec<RaceResult>>`], where
/// the [`Payload`] variant [`Payload::RaceResults`] has already been extracted and processed into
/// [`Race<Vec<RaceResult>>`], obviating the need to perform error checking and extraction of the
/// expected variants.
///
/// This function returns a singe [`Race`] containing a sequence of [`SessionResult`]s, i.e. it
/// returns a [`Race<Vec<T>>`]. If multiple [`Race`]s are expected in the response, or a single
/// [`SessionResult`] per [`Race`], or other, consider using one of the other methods with the
/// desired processing: [`get_session_results`], [`get_session_result_for_events`], or
/// [`get_session_result`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_session_results_for_event, resource::Filters, response::RaceResult};
///
/// let race = get_session_results_for_event::<RaceResult>(Filters::new().season(2021).round(22)).unwrap();
///
/// assert_eq!(race.race_name, "Abu Dhabi Grand Prix");
/// assert_eq!(race.race_results()[0].driver.family_name, "Verstappen");
/// assert_eq!(race.race_results()[0].position, 1);
/// assert_eq!(race.race_results()[1].driver.family_name, "Hamilton");
/// assert_eq!(race.race_results()[1].position, 2);
/// ```
pub fn get_session_results_for_event<T: SessionResult>(filters: Filters) -> Result<Race<Vec<T>>> {
    get_session_results(filters).and_then(verify_has_one_element_and_extract)
}

/// Performs a GET request to the Ergast API for the [`Resource`] corresponding to the requested
/// [`SessionResult`], with the argument [`Filters`], and returns a sequence of [`Race`]s with a
/// single [`SessionResult`] each, processed from the inner [`Race`]s from [`Table`]. An
/// [`Error::MultiPage`] is returned if the results would not fit in a [`Page::with_max_limit`].
/// An [`Error::NotFound`] or [`Error::TooMany`] is returned if the expected number of [`Race`]s and
/// [`SessionResult`]s per [`Race`] are not found in the response.
///
/// For example, [`get_session_result_for_events::<RaceResult>`] will perform a GET request to the
/// Ergast API for [`Resource::RaceResults`], and return a sequence of [`Race<RaceResult>`], where
/// the [`Payload`] variant [`Payload::RaceResults`] has already been extracted and processed into
/// [`Race<RaceResult>`], ensuring that each [`Race`] holds one and only one [`SessionResult`],
/// obviating the need to perform error checking and extraction of the expected variants.
///
/// This function returns a sequence of [`Race`]s containing a single [`SessionResult`] each, i.e.
/// it returns [`Vec<Race<T>>`]. If a single [`Race`] is expected in the response, or a single
/// [`SessionResult`] per [`Race`], or other, consider using one of the other methods with the
/// desired processing: [`get_session_results`], [`get_session_results_for_event`], or
/// [`get_session_result`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::DriverID;
/// use f1_data::ergast::{
///     get::get_session_result_for_events,
///     resource::Filters,
///     response::QualifyingResult
/// };
///
/// let seb_poles: u32 = get_session_result_for_events::<QualifyingResult>(
///     Filters::new().driver_id(DriverID::from("vettel")).qualifying_pos(1),
/// )
/// .unwrap()
/// .iter()
/// .map(|race| if race.qualifying_result().position == 1 { 1 } else { 0 })
/// .sum();
///
/// assert_eq!(seb_poles, 57);
/// ```
pub fn get_session_result_for_events<T: SessionResult>(filters: Filters) -> Result<Vec<Race<T>>> {
    get_session_results(filters)?
        .into_iter()
        .map(|race| race.try_map(verify_has_one_element_and_extract))
        .collect()
}

/// Performs a GET request to the Ergast API for the [`Resource`] corresponding to the requested
/// [`SessionResult`], with the argument [`Filters`], and returns a single [`Race`] with a single
/// [`SessionResult`], processed from the inner [`Race`]s from [`Table`]. An [`Error::MultiPage`] is
/// returned if the results would not fit in a [`Page::with_max_limit`]. An [`Error::NotFound`] or
/// [`Error::TooMany`] is returned if the expected number of [`Race`]s and [`SessionResult`]s per
/// [`Race`] are not found in the response.
///
/// For example, [`get_session_result::<RaceResult>`] will perform a GET request to the Ergast API
/// for [`Resource::RaceResults`], and return a single [`Race<RaceResult>`], where the [`Payload`]
/// variant [`Payload::RaceResults`] has already been extracted and processed into
/// [`Race<RaceResult>`], ensuring that one and only one [`Race`] is found, holding one and only
/// one [`SessionResult`], obviating the need to perform error checking and extraction of the
/// expected variants.
///
/// This function returns a single [`Race`]s containing a single [`SessionResult`], i.e. it returns
/// [`Race<T>`]. If multiple [`Race`]s or [`SessionResult`]s are expected in the response, consider
/// using one of the other methods with the desired processing: [`get_session_results`],
/// [`get_session_results_for_event`], or [`get_session_result_for_events`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::ergast::{get::get_session_result, resource::Filters, response::SprintResult};
///
/// let race = get_session_result::<SprintResult>(
///     Filters::new().season(2021).round(10).sprint_pos(1)).unwrap();
///
/// assert_eq!(race.sprint_result().position, 1);
/// assert_eq!(race.sprint_result().driver.family_name, "Verstappen");
/// ```
pub fn get_session_result<T: SessionResult>(filters: Filters) -> Result<Race<T>> {
    get_session_result_for_events(filters).and_then(verify_has_one_element_and_extract)
}

/// Convenience alias for [`get_session_results::<QualifyingResult>`].
pub fn get_qualifying_results(filters: Filters) -> Result<Vec<Race<Vec<QualifyingResult>>>> {
    get_session_results::<QualifyingResult>(filters)
}

/// Convenience alias for [`get_session_results_for_event::<QualifyingResult>`].
pub fn get_qualifying_results_for_event(filters: Filters) -> Result<Race<Vec<QualifyingResult>>> {
    get_session_results_for_event::<QualifyingResult>(filters)
}

/// Convenience alias for [`get_session_result_for_events::<QualifyingResult>`].
pub fn get_qualifying_result_for_events(filters: Filters) -> Result<Vec<Race<QualifyingResult>>> {
    get_session_result_for_events::<QualifyingResult>(filters)
}

/// Convenience alias for [`get_session_result::<QualifyingResult>`].
pub fn get_qualifying_result(filters: Filters) -> Result<Race<QualifyingResult>> {
    get_session_result::<QualifyingResult>(filters)
}

/// Convenience alias for [`get_session_results::<SprintResult>`].
pub fn get_sprint_results(filters: Filters) -> Result<Vec<Race<Vec<SprintResult>>>> {
    get_session_results::<SprintResult>(filters)
}

/// Convenience alias for [`get_session_results_for_event::<SprintResult>`].
pub fn get_sprint_results_for_event(filters: Filters) -> Result<Race<Vec<SprintResult>>> {
    get_session_results_for_event::<SprintResult>(filters)
}

/// Convenience alias for [`get_session_result_for_events::<SprintResult>`].
pub fn get_sprint_result_for_events(filters: Filters) -> Result<Vec<Race<SprintResult>>> {
    get_session_result_for_events::<SprintResult>(filters)
}

/// Convenience alias for [`get_session_result::<SprintResult>`].
pub fn get_sprint_result(filters: Filters) -> Result<Race<SprintResult>> {
    get_session_result::<SprintResult>(filters)
}
/// Convenience alias for [`get_session_results::<RaceResult>`].
pub fn get_race_results(filters: Filters) -> Result<Vec<Race<Vec<RaceResult>>>> {
    get_session_results::<RaceResult>(filters)
}

/// Convenience alias for [`get_session_results_for_event::<RaceResult>`].
pub fn get_race_results_for_event(filters: Filters) -> Result<Race<Vec<RaceResult>>> {
    get_session_results_for_event::<RaceResult>(filters)
}

/// Convenience alias for [`get_session_result_for_events::<RaceResult>`].
pub fn get_race_result_for_events(filters: Filters) -> Result<Vec<Race<RaceResult>>> {
    get_session_result_for_events::<RaceResult>(filters)
}

/// Convenience alias for [`get_session_result::<RaceResult>`].
pub fn get_race_result(filters: Filters) -> Result<Race<RaceResult>> {
    get_session_result::<RaceResult>(filters)
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
    get_response_max_limit(&Resource::FinishingStatus(filters))?
        .mr_data
        .table
        .into_status()
        .map_err(into)
}

/// Represents a flattened combination of a [`Lap`] and [`Timing`] for a single driver, indented to
/// make use more ergonomic, without nesting, when accessing a single driver's lap and timing data.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct DriverLap {
    /// Directly maps to [`Lap::number`] for a given [`Lap`].
    pub number: u32,
    /// Directly maps to [`Timing::position`] for a given driver's [`Timing`] in a given [`Lap`].
    pub position: u32,
    /// Directly maps to [`Timing::time`] for a given driver's [`Timing`] in a given [`Lap`].
    pub time: Duration,
}

impl DriverLap {
    /// Returns a [`Result<DriverLap>`] from the given [`Lap`], verifying that it contains a single
    /// [`Timing`] and that its `driver_id` field matches the passed [`DriverID`]. It returns
    /// [`Error::UnexpectedData`] if the data's `driver_id` does not match the argument's.
    pub fn try_from(lap: Lap, driver_id: &DriverID) -> Result<Self> {
        let timing = verify_has_one_element_and_extract(lap.timings)?;

        if timing.driver_id != *driver_id {
            return Err(Error::UnexpectedData(format!(
                "Expected driver_id '{}' but got '{}'",
                driver_id, timing.driver_id
            )));
        }

        Ok(Self {
            number: lap.number,
            position: timing.position,
            time: timing.time,
        })
    }
}

/// Performs a GET request to the Ergast API for [`Resource::LapTimes`] from a specified [`RaceID`]
/// and for a specified single [`DriverID`], returning a list of [`DriverLap`]s, which is a
/// flattened combination of [`Lap`]s and [`Timing`]s. An [`Error::MultiPage`] is returned if
/// `lap_times` would not fit in a [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::{DriverID, RaceID};
/// use f1_data::ergast::{get::get_driver_laps, time::duration_m_s_ms};
///
/// let laps = get_driver_laps(RaceID::from(2023, 4), &DriverID::from("leclerc")).unwrap();
/// assert_eq!(laps.len(), 51);
/// assert_eq!(laps[0].number, 1);
/// assert_eq!(laps[0].time, duration_m_s_ms(1, 50, 109));
///
/// assert_eq!(laps[0].position, 1);
/// assert_eq!(laps[2].position, 2)
/// ```
pub fn get_driver_laps(race_id: RaceID, driver_id: &DriverID) -> Result<Vec<DriverLap>> {
    get_response_max_limit(&Resource::LapTimes(LapTimeFilters {
        season: race_id.season,
        round: race_id.round,
        lap: None,
        driver_id: Some(driver_id.clone()),
    }))
    .and_then(verify_has_one_race_and_extract)?
    .payload
    .into_laps()
    .map_err(into)
    .map(into_iter)
    .and_then(|laps| laps.map(|lap| DriverLap::try_from(lap, driver_id)).collect())
}

/// Performs a GET request to the Ergast API for [`Resource::LapTimes`] from a specified [`RaceID`]
/// and for a specified single lap, returning a list of [`Timing`]s from the requested [`Lap`].
/// An [`Error::MultiPage`] is returned if `lap_times` would not fit in a [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::{DriverID, RaceID};
/// use f1_data::ergast::{get::get_lap_timings, time::duration_m_s_ms};
///
/// let timings = get_lap_timings(RaceID::from(2023, 4), 1).unwrap();
/// assert_eq!(timings.len(), 20);
/// assert_eq!(timings[0].driver_id, DriverID::from("leclerc"));
/// assert_eq!(timings[0].position, 1);
/// assert_eq!(timings[0].time, duration_m_s_ms(1, 50, 109));
/// ```
pub fn get_lap_timings(race_id: RaceID, lap: u32) -> Result<Vec<Timing>> {
    get_response_max_limit(&Resource::LapTimes(LapTimeFilters {
        season: race_id.season,
        round: race_id.round,
        lap: Some(lap),
        driver_id: None,
    }))
    .and_then(verify_has_one_race_and_extract)?
    .payload
    .into_laps()
    .map_err(into)
    .and_then(verify_has_one_element_and_extract)
    .map(|lap| lap.timings)
}

/// Performs a GET request to the Ergast API for [`Resource::PitStops`], with the passed argument
/// [`PitStopFilters`], and return the resulting inner [`PitStop`]s from `race.payload` in the
/// expected single [`Race`] element from [`Table`] in `resp.mr_data.table`. An [`Error::MultiPage`]
/// is returned if `payload` would not fit in a [`Page::with_max_limit`].
///
/// # Examples
///
/// ```no_run
/// use f1_data::id::DriverID;
/// use f1_data::ergast::{
///     get::get_pit_stops,
///     resource::PitStopFilters,
///     time::{duration_m_s_ms, macros::time},
///     response::PitStop};
///
/// let pit_stops = get_pit_stops(PitStopFilters::new(2023, 4)).unwrap();
/// assert_eq!(pit_stops.len(), 23);
/// assert_eq!(
///     pit_stops[0],
///     PitStop {
///         driver_id: DriverID::from("gasly"),
///         lap: 5,
///         stop: 1,
///         time: time!(15:13:22),
///         duration: duration_m_s_ms(0, 20, 235)
///     }
/// );
/// ```
pub fn get_pit_stops(filters: PitStopFilters) -> Result<Vec<PitStop>> {
    get_response_max_limit(&Resource::PitStops(filters))
        .and_then(verify_has_one_race_and_extract)?
        .payload
        .into_pit_stops()
        .map_err(into)
}

/// Call the provided function, retrying on HTTP errors, and forwarding anything else.
/// `max_attempt_count` and `retry_sleep` are used to control the retry behaviour.
pub fn retry_on_http_error<T>(
    f: impl Fn() -> Result<T>,
    max_attempt_count: usize,
    retry_sleep: std::time::Duration,
) -> Result<T> {
    for _ in 0..=max_attempt_count {
        match f() {
            Err(Error::Http(_)) => std::thread::sleep(retry_sleep),
            other => return other,
        }
    }
    panic!("Retried {max_attempt_count} times on HTTP errors, giving up");
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

/// Extract a single element `T` from [`Vec<T>`] into [`Result<T>`], enforcing that there is only
/// one element in the vector, returning [`Error::NotFound`] if it contained no elements, or
/// [`Error::TooMany`] if it contained more than one.
fn verify_has_one_element_and_extract<T>(mut sequence: Vec<T>) -> Result<T> {
    match sequence.len() {
        0 => Err(Error::NotFound),
        1 => Ok(sequence.remove(0)),
        _ => Err(Error::TooMany),
    }
}

/// Extract single [`Race`] from a [`Response`], into [`Result<Race>`], enforcing that there is only
/// one race in the [`Response`], returning [`Error::NotFound`] if the it contained no races, or
/// [`Error::TooMany`] if it contained more than one.
fn verify_has_one_race_and_extract(response: Response) -> Result<Race> {
    response
        .mr_data
        .table
        .into_races()
        .map_err(into)
        .and_then(verify_has_one_element_and_extract)
}

/// Shorthand for closure `|e| e.into()` and/or `std::convert::Into::into`.
fn into<T: Into<U>, U>(t: T) -> U {
    t.into()
}

/// Shorthand for closure `|v| v.into_iter()` and/or `std::iter::IntoIterator::into_iter`.
fn into_iter<T: IntoIterator>(t: T) -> T::IntoIter {
    t.into_iter()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use once_cell::sync::Lazy;

    use crate::{
        ergast::{
            resource::{Filters, LapTimeFilters, PitStopFilters, Resource},
            response::*,
        },
        id::{RoundID, SeasonID},
    };

    use super::*;
    use crate::ergast::tests::assets::*;

    /// Default maximum number of attempts to retry on HTTP errors, for [`retry_on_http_error`].
    const DEFAULT_HTTP_RETRY_MAX_ATTEMPT_COUNT: usize = 3;

    /// Default sleep duration between attempts to retry on HTTP errors, for [`retry_on_http_error`]
    const DEFAULT_HTTP_RETRY_SLEEP: std::time::Duration = std::time::Duration::from_secs(5);

    /// Forward to [`retry_on_http_error`] with default retry parameters.
    fn retry_http<T>(f: impl Fn() -> Result<T>) -> Result<T> {
        retry_on_http_error(f, DEFAULT_HTTP_RETRY_MAX_ATTEMPT_COUNT, DEFAULT_HTTP_RETRY_SLEEP)
    }

    /// Represents a constraint on the length of a list, e.g. a minimum or exact length.
    enum LenConstraint {
        Exactly(usize),
        Minimum(usize),
    }

    /// Call a `get_actual` function to get an actual list, and assert that this list contains every
    /// element in an expected lit, and that it meets a given length constraint, e.g. that is has a
    /// minimum or exact length. The actual list may be a superset of the expected list.
    fn assert_each_expected_in_actual<G, T>(
        get_actual: G,
        expected_list: &[T],
        actual_list_len_constraint: LenConstraint,
    ) where
        G: Fn() -> Result<Vec<T>>,
        T: PartialEq + core::fmt::Debug,
    {
        let actual_list = retry_http(|| get_actual()).unwrap();
        let actual_list = &actual_list;

        match actual_list_len_constraint {
            LenConstraint::Exactly(exact_len) => assert_eq!(actual_list.len(), exact_len),
            LenConstraint::Minimum(min_len) => assert!(actual_list.len() >= min_len),
        };

        assert!(!expected_list.is_empty());

        for expected in expected_list {
            assert!(actual_list.iter().find(|actual| actual == &expected).is_some());
        }
    }

    /// Call a `get` function for each expected element, asserting that it equals the actual result.
    fn assert_each_get_eq_expected<G, T>(get: G, expected_list: &[T])
    where
        G: Fn(&T) -> Result<T>,
        T: PartialEq + core::fmt::Debug,
    {
        assert!(!expected_list.is_empty());

        for expected in expected_list {
            assert_eq!(&retry_http(|| get(expected)).unwrap(), expected);
        }
    }

    /// Construct a [`Filters`] object with a given season and round.
    fn race_filters(season: SeasonID, round: RoundID) -> Filters {
        Filters::new().season(season).round(round)
    }

    /// Construct a [`Filters`] object with a given season and round, extracted from a [`Race`].
    fn race_filters_from<T>(race: &Race<T>) -> Filters {
        race_filters(race.season, race.round)
    }

    /// Call a `get_actual` function to an actual [`Race<Vec<T>>`], and assert that its list of
    /// session results contains every session result from an expected [`Race<Payload>`], and that
    /// it meets a length constraint, e.g. that is has a minimum or exact length. The actual list
    /// may be a superset of the expected list.
    fn assert_each_expected_session_result_in_actual_event<G, T>(
        get_actual: G,
        expected: &Race<Payload>,
        actual_payload_len_constraint: LenConstraint,
    ) where
        G: Fn() -> Result<Race<Vec<T>>>,
        T: SessionResult + PartialEq + Clone + core::fmt::Debug,
    {
        let actual = retry_http(|| get_actual()).unwrap();

        assert!(eq_race_info(&actual, expected));

        assert_each_expected_in_actual(
            || Ok(actual.payload.clone()),
            &expected.clone().map(|p| T::try_inner_from(p).unwrap()).payload,
            actual_payload_len_constraint,
        );
    }

    /// Call a `get` function for each session result from an expected [`Race<Payload>`], asserting
    /// that it equals the actual result. `add_result_filter` is called to modify the [`Filters`]
    /// for each expected result, in addition to the season/round from the expected [`Race`].
    fn assert_each_get_eq_expected_session_result<G, F, T>(get: G, add_result_filter: F, race: &Race<Payload>)
    where
        G: Fn(Filters) -> Result<Race<T>>,
        F: Fn(&T, Filters) -> Filters,
        T: SessionResult + Clone + PartialEq + core::fmt::Debug,
    {
        assert_each_get_eq_expected(
            |result| retry_http(|| get(add_result_filter(result, race_filters_from(race)))).map(|race| race.payload),
            &race.clone().map(|p| T::try_inner_from(p).unwrap()).payload,
        );
    }

    /// Call a `get` function and assert that the returned [`Result<Vec<T>>`] is [`Ok`], and that
    /// held sequence value is empty.
    fn assert_is_empty<G: Fn() -> Result<Vec<T>>, T>(get: G) {
        assert!(retry_http(|| get()).unwrap().is_empty());
    }

    /// Call a `get` function and assert that the returned [`Result`] is [`Err(Error::NotFound)`].
    fn assert_not_found<G: Fn() -> Result<T>, T>(get: G) {
        assert!(matches!(get(), Err(Error::NotFound)));
    }

    /// Call a `get` function and assert that the returned [`Result`] is [`Err(Error::TooMany)`].
    fn assert_too_many<G: Fn() -> Result<T>, T>(get: G) {
        assert!(matches!(get(), Err(Error::TooMany)));
    }

    // Resource::SeasonList
    // --------------------

    #[test]
    #[ignore]
    fn get_seasons() {
        assert_each_expected_in_actual(
            || super::get_seasons(Filters::none()),
            &SEASON_TABLE.as_seasons().unwrap(),
            LenConstraint::Minimum(74),
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
        assert_is_empty(|| super::get_seasons(Filters::new().season(1949)));
    }

    #[test]
    #[ignore]
    fn get_season_error_not_found() {
        assert_not_found(|| super::get_season(1949));
    }

    // Resource::DriverInfo
    // --------------------

    #[test]
    #[ignore]
    fn get_drivers() {
        assert_each_expected_in_actual(
            || super::get_drivers(Filters::none()),
            &DRIVER_TABLE.as_drivers().unwrap(),
            LenConstraint::Minimum(857),
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
        assert_is_empty(|| super::get_drivers(Filters::new().season(1949)));
    }

    #[test]
    #[ignore]
    fn get_driver_error_not_found() {
        assert_not_found(|| super::get_driver(DriverID::from("unknown")));
    }

    // Resource::ConstructorInfo
    // -------------------------

    #[test]
    #[ignore]
    fn get_constructors() {
        assert_each_expected_in_actual(
            || super::get_constructors(Filters::none()),
            &CONSTRUCTOR_TABLE.as_constructors().unwrap(),
            LenConstraint::Minimum(211),
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
        assert_is_empty(|| super::get_constructors(Filters::new().season(1949)));
    }

    #[test]
    #[ignore]
    fn get_constructor_error_not_found() {
        assert_not_found(|| super::get_constructor(ConstructorID::from("unknown")));
    }

    // Resource::CircuitInfo
    // ---------------------

    #[test]
    #[ignore]
    fn get_circuits() {
        assert_each_expected_in_actual(
            || super::get_circuits(Filters::none()),
            &CIRCUIT_TABLE.as_circuits().unwrap(),
            LenConstraint::Minimum(77),
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
        assert_is_empty(|| super::get_circuits(Filters::new().season(1949)));
    }

    #[test]
    #[ignore]
    fn get_circuit_error_not_found() {
        assert_not_found(|| super::get_circuit(CircuitID::from("unknown")));
    }

    // Resource::RaceSchedule
    // ----------------------

    fn map_schedules(races: Vec<Race>) -> Vec<Race<Schedule>> {
        races
            .into_iter()
            .map(|race| race.map(|payload| payload.into_schedule().unwrap()))
            .collect()
    }

    #[test]
    #[ignore]
    fn get_race_schedules() {
        // Calling [`get_race_schedules`] with no filters returns [`Error::MultiPage`], since there
        // have been more than 1000 races. As such, we are testing calls with by-season filters to
        // restrict the responses to a smaller, but still plural, element count, usually ~20.

        static RACE_SCHEDULES_COUNTS_BY_SEASON: Lazy<HashMap<u32, usize>> = Lazy::new(|| {
            HashMap::from([
                (1950, 7),
                (2003, 16),
                (2015, 19),
                (2020, 17),
                (2021, 22),
                (2022, 22),
                (2023, 22),
            ])
        });

        assert!(!RACE_SCHEDULES_BY_SEASON.is_empty());

        for (season, expected_list) in &*RACE_SCHEDULES_BY_SEASON {
            assert_each_expected_in_actual(
                || super::get_race_schedules(Filters::new().season(*season)),
                &map_schedules(expected_list.clone()),
                LenConstraint::Exactly(*RACE_SCHEDULES_COUNTS_BY_SEASON.get(season).unwrap()),
            );
        }
    }

    #[test]
    #[ignore]
    fn get_race_schedule() {
        assert_each_get_eq_expected(
            |race| super::get_race_schedule(RaceID::from(race.season, race.round)),
            &map_schedules(RACE_TABLE_SCHEDULE.clone().into_races().unwrap()),
        );
    }

    #[test]
    #[ignore]
    fn get_race_schedules_empty() {
        assert_is_empty(|| super::get_race_schedules(Filters::new().season(1949)));
    }

    #[test]
    #[ignore]
    fn get_race_schedule_error_not_found() {
        assert_not_found(|| super::get_race_schedule(RaceID::from(1949, 1)));
    }

    // Resource::QualifyingResults
    // ---------------------------

    #[test]
    #[ignore]
    fn get_qualifying_results() {
        assert_each_expected_in_actual(
            || super::get_qualifying_results(Filters::new().constructor_id("red_bull".into())),
            &RACES_QUALIFYING_RESULTS_RED_BULL,
            LenConstraint::Minimum(357),
        );
    }

    #[test]
    #[ignore]
    fn get_qualifying_results_for_event() {
        assert_each_expected_session_result_in_actual_event(
            || super::get_qualifying_results_for_event(race_filters(2003, 4)),
            &RACE_2003_4_QUALIFYING_RESULTS,
            LenConstraint::Exactly(20),
        );

        assert_each_expected_session_result_in_actual_event(
            || super::get_qualifying_results_for_event(race_filters(2023, 4)),
            &RACE_2023_4_QUALIFYING_RESULTS,
            LenConstraint::Exactly(20),
        );
    }

    #[test]
    #[ignore]
    fn get_qualifying_result_for_events() {
        assert_each_expected_in_actual(
            || super::get_qualifying_result_for_events(Filters::new().qualifying_pos(1)),
            &RACES_QUALIFYING_RESULT_P1,
            LenConstraint::Minimum(459),
        );

        assert_each_expected_in_actual(
            || super::get_qualifying_result_for_events(Filters::new().qualifying_pos(2)),
            &RACES_QUALIFYING_RESULT_P2,
            LenConstraint::Minimum(459),
        );
    }

    #[test]
    #[ignore]
    fn get_qualifying_result() {
        assert_each_get_eq_expected_session_result(
            super::get_qualifying_result,
            |result, filters| filters.qualifying_pos(result.position),
            &RACE_2003_4_QUALIFYING_RESULTS,
        );

        assert_each_get_eq_expected_session_result(
            super::get_qualifying_result,
            |result, filters| filters.qualifying_pos(result.position),
            &RACE_2023_4_QUALIFYING_RESULTS,
        );
    }

    #[test]
    #[ignore]
    fn get_qualifying_results_empty() {
        assert_is_empty(|| super::get_qualifying_results(Filters::new().season(1949)));
        assert_is_empty(|| super::get_qualifying_results(Filters::new().season(2021).qualifying_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_qualifying_results_for_event_error_not_found() {
        assert_not_found(|| super::get_qualifying_results_for_event(Filters::new().season(1949).round(1)));
    }

    #[test]
    #[ignore]
    fn get_qualifying_results_for_event_error_too_many() {
        assert_too_many(|| super::get_qualifying_results_for_event(Filters::new().season(2021)));
    }

    #[test]
    #[ignore]
    fn get_qualifying_result_for_events_empty() {
        assert_is_empty(|| super::get_qualifying_result_for_events(Filters::new().season(1949).qualifying_pos(1)));
        assert_is_empty(|| super::get_qualifying_result_for_events(Filters::new().season(2021).qualifying_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_qualifying_result_for_events_error_too_many() {
        assert_too_many(|| super::get_qualifying_result_for_events(Filters::new().season(2021)));
    }

    #[test]
    #[ignore]
    fn get_qualifying_result_error_not_found() {
        assert_not_found(|| super::get_qualifying_result(Filters::new().season(1949).round(1).qualifying_pos(1)));
        assert_not_found(|| super::get_qualifying_result(Filters::new().season(2021).round(10).qualifying_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_qualifying_result_error_too_many() {
        assert_too_many(|| super::get_qualifying_result(Filters::new().season(2021).qualifying_pos(1)));
        assert_too_many(|| super::get_qualifying_result(Filters::new().season(2021).round(10)));
    }

    // Resource::SprintResults
    // -----------------------

    #[test]
    #[ignore]
    fn get_sprint_results() {
        assert_each_expected_in_actual(
            || super::get_sprint_results(Filters::new().constructor_id("red_bull".into())),
            &RACES_SPRINT_RESULTS_RED_BULL,
            LenConstraint::Minimum(8),
        );
    }

    #[test]
    #[ignore]
    fn get_sprint_results_for_event() {
        assert_each_expected_session_result_in_actual_event(
            || super::get_sprint_results_for_event(race_filters(2023, 4)),
            &RACE_2023_4_SPRINT_RESULTS,
            LenConstraint::Exactly(20),
        );
    }

    #[test]
    #[ignore]
    fn get_sprint_result_for_events() {
        assert_each_expected_in_actual(
            || super::get_sprint_result_for_events(Filters::new().sprint_pos(1)),
            &RACES_SPRINT_RESULT_P1,
            LenConstraint::Minimum(8),
        );
    }

    #[test]
    #[ignore]
    fn get_sprint_result() {
        assert_each_get_eq_expected_session_result(
            super::get_sprint_result,
            |result, filters| filters.sprint_pos(result.position),
            &RACE_2023_4_SPRINT_RESULTS,
        );
    }

    #[test]
    #[ignore]
    fn get_sprint_results_empty() {
        assert_is_empty(|| super::get_sprint_results(Filters::new().season(1949)));
        assert_is_empty(|| super::get_sprint_results(Filters::new().season(2021).sprint_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_sprint_results_for_event_error_not_found() {
        assert_not_found(|| super::get_sprint_results_for_event(Filters::new().season(1949).round(1)));
    }

    #[test]
    #[ignore]
    fn get_sprint_results_for_event_error_too_many() {
        assert_too_many(|| super::get_sprint_results_for_event(Filters::new().season(2021)));
    }

    #[test]
    #[ignore]
    fn get_sprint_result_for_events_empty() {
        assert_is_empty(|| super::get_sprint_result_for_events(Filters::new().season(1949).sprint_pos(1)));
        assert_is_empty(|| super::get_sprint_result_for_events(Filters::new().season(2021).sprint_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_sprint_result_for_events_error_too_many() {
        assert_too_many(|| super::get_sprint_result_for_events(Filters::new().season(2021)));
    }

    #[test]
    #[ignore]
    fn get_sprint_result_error_not_found() {
        assert_not_found(|| super::get_sprint_result(Filters::new().season(1949).round(1).sprint_pos(1)));
        assert_not_found(|| super::get_sprint_result(Filters::new().season(2021).round(10).sprint_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_sprint_result_error_too_many() {
        assert_too_many(|| super::get_sprint_result(Filters::new().season(2021).sprint_pos(1)));
        assert_too_many(|| super::get_sprint_result(Filters::new().season(2021).round(10)));
    }

    // Resource::RaceResults
    // ---------------------

    #[test]
    #[ignore]
    fn get_race_results() {
        // @todo Ergast data seems to have an issue for the 2011-14-P1 race result which breaks the
        // parsing (Time.time is inconsistent with Time.millis), so the full "red_bull" query fails.
        // Use the commented out query below once this errors has been fixed in the Ergast API.

        // assert_each_expected_in_actual(
        //     &super::get_race_results(Filters::new().constructor_id("red_bull".into())).unwrap(),
        //     &RACES_RACE_RESULTS_RED_BULL,
        //     LenConstraint::Minimum(718),
        // );

        // @todo Remove once the query above can be used.
        assert_each_expected_in_actual(
            || super::get_race_results(Filters::new().constructor_id("red_bull".into()).season(2023)),
            &RACES_RACE_RESULTS_RED_BULL,
            LenConstraint::Minimum(11),
        );
    }

    #[test]
    #[ignore]
    fn get_race_results_for_event() {
        assert_each_expected_session_result_in_actual_event(
            || super::get_race_results_for_event(race_filters(2003, 4)),
            &RACE_2003_4_RACE_RESULTS,
            LenConstraint::Exactly(20),
        );

        assert_each_expected_session_result_in_actual_event(
            || super::get_race_results_for_event(race_filters(2021, 12)),
            &RACE_2021_12_RACE_RESULTS,
            LenConstraint::Exactly(20),
        );

        assert_each_expected_session_result_in_actual_event(
            || super::get_race_results_for_event(race_filters(2023, 4)),
            &RACE_2023_4_RACE_RESULTS,
            LenConstraint::Exactly(20),
        );
    }

    #[test]
    #[ignore]
    fn get_race_result_for_events() {
        assert_each_expected_in_actual(
            || super::get_race_result_for_events(Filters::new().driver_id("michael_schumacher".into())),
            &RACES_RACE_RESULT_MICHAEL,
            LenConstraint::Exactly(308),
        );

        assert_each_expected_in_actual(
            || super::get_race_result_for_events(Filters::new().driver_id("max_verstappen".into())),
            &RACES_RACE_RESULT_MAX,
            LenConstraint::Minimum(174),
        );
    }

    #[test]
    #[ignore]
    fn get_race_result() {
        assert_each_get_eq_expected_session_result(
            super::get_race_result,
            |result, filters| filters.finish_pos(result.position),
            &RACE_2021_12_RACE_RESULTS,
        );

        // @todo Cannot use all available race results because, counterintuitively, non-finishing
        // race results cannot be filtered by .finish_pos, even though .position would be set.
        // See [`Resource::RaceResults`], and try reaching out to Ergast maintainers about it.

        assert_each_get_eq_expected(
            |result| super::get_race_result(race_filters(2003, 4).finish_pos(result.position)).map(|race| race.payload),
            &RACE_2003_4_RACE_RESULTS.payload.as_race_results().unwrap()[0..2],
        );

        assert_each_get_eq_expected(
            |result| super::get_race_result(race_filters(2023, 4).finish_pos(result.position)).map(|race| race.payload),
            &RACE_2023_4_RACE_RESULTS.payload.as_race_results().unwrap()[0..2],
        );
    }

    #[test]
    #[ignore]
    fn get_race_results_empty() {
        assert_is_empty(|| super::get_race_results(Filters::new().season(1949)));
        assert_is_empty(|| super::get_race_results(Filters::new().season(2021).finish_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_race_results_for_event_error_not_found() {
        assert_not_found(|| super::get_race_results_for_event(Filters::new().season(1949).round(1)));
    }

    #[test]
    #[ignore]
    fn get_race_results_for_event_error_too_many() {
        assert_too_many(|| super::get_race_results_for_event(Filters::new().season(2021)));
    }

    #[test]
    #[ignore]
    fn get_race_result_for_events_empty() {
        assert_is_empty(|| super::get_race_result_for_events(Filters::new().season(1949).finish_pos(1)));
        assert_is_empty(|| super::get_race_result_for_events(Filters::new().season(2021).finish_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_race_result_for_events_error_too_many() {
        assert_too_many(|| super::get_race_result_for_events(Filters::new().season(2021)));
    }

    #[test]
    #[ignore]
    fn get_race_result_error_not_found() {
        assert_not_found(|| super::get_race_result(Filters::new().season(1949).round(1).finish_pos(1)));
        assert_not_found(|| super::get_race_result(Filters::new().season(2021).round(10).finish_pos(100)));
    }

    #[test]
    #[ignore]
    fn get_race_result_error_too_many() {
        assert_too_many(|| super::get_race_result(Filters::new().season(2021).finish_pos(1)));
        assert_too_many(|| super::get_race_result(Filters::new().season(2021).round(10)));
    }

    // Resource::FinishingStatus
    // -------------------------

    #[test]
    #[ignore]
    fn get_statuses() {
        assert_each_expected_in_actual(
            || super::get_statuses(Filters::new().season(2022)),
            &STATUS_TABLE_2022.as_status().unwrap(),
            LenConstraint::Exactly(29),
        );
    }

    #[test]
    #[ignore]
    fn get_statuses_empty() {
        assert_is_empty(|| super::get_statuses(Filters::new().season(1949)));
    }

    // Resource::LapTimes
    // ------------------

    fn assert_driver_lap_eq(driver_lap: &DriverLap, lap: &Lap, timing: &Timing) {
        assert_eq!(driver_lap.number, lap.number);
        assert_eq!(driver_lap.position, timing.position);
        assert_eq!(driver_lap.time, timing.time);
    }

    #[test]
    #[ignore]
    fn get_driver_laps() {
        let race_id = RaceID::from(2023, 4);
        let leclerc_laps = retry_http(|| super::get_driver_laps(race_id, &DriverID::from("leclerc"))).unwrap();
        let max_laps = retry_http(|| super::get_driver_laps(race_id, &DriverID::from("max_verstappen"))).unwrap();

        assert_driver_lap_eq(&leclerc_laps[0], &LAP_2023_4_L1, &TIMING_2023_4_L1_P1);
        assert_driver_lap_eq(&leclerc_laps[1], &LAP_2023_4_L2, &TIMING_2023_4_L2_P1);
        assert_driver_lap_eq(&max_laps[0], &LAP_2023_4_L1, &TIMING_2023_4_L1_P2);
        assert_driver_lap_eq(&max_laps[1], &LAP_2023_4_L2, &TIMING_2023_4_L2_P2);

        let mut current_lap = 1;

        for (leclerc, max) in leclerc_laps.iter().zip(max_laps.iter()) {
            assert_eq!(leclerc.number, current_lap);
            assert_eq!(max.number, current_lap);

            assert!(leclerc.position <= 3);

            if current_lap == 11 {
                assert_eq!(max.position, 7);
            } else {
                assert!(max.position <= 3);
            }

            current_lap += 1;
        }
    }

    #[test]
    #[ignore]
    fn get_lap_timings() {
        let l1 = || super::get_lap_timings(RaceID::from(2023, 4), 1);
        let l2 = || super::get_lap_timings(RaceID::from(2023, 4), 2);

        assert_each_expected_in_actual(l1, &LAP_2023_4_L1.timings, LenConstraint::Exactly(20));
        assert_each_expected_in_actual(l2, &LAP_2023_4_L2.timings, LenConstraint::Exactly(20));
    }

    #[test]
    #[ignore]
    fn get_driver_laps_error_not_found() {
        assert_not_found(|| super::get_driver_laps(RaceID::from(1949, 1), &DriverID::from("leclerc")));
        assert_not_found(|| super::get_driver_laps(RaceID::from(2023, 4), &DriverID::from("abate")));
    }

    #[test]
    #[ignore]
    fn get_lap_timings_error_not_found() {
        assert_not_found(|| super::get_lap_timings(RaceID::from(1949, 1), 1));
        assert_not_found(|| super::get_lap_timings(RaceID::from(2023, 4), 100));
    }

    #[test]
    #[ignore]
    fn get_response_page_lap_times_race_2023_4() {
        let resp = retry_http(|| get_response_page(&Resource::LapTimes(LapTimeFilters::new(2023, 4)), Page::default()))
            .unwrap();

        let actual = verify_has_one_race_and_extract(resp).unwrap();
        let expected = &RACE_2023_4_LAPS;

        assert!(eq_race_info(&actual, expected));

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
    fn get_pit_stops() {
        assert_each_expected_in_actual(
            || super::get_pit_stops(PitStopFilters::new(2023, 4)),
            &RACE_2023_4_PIT_STOPS.payload.as_pit_stops().unwrap(),
            LenConstraint::Exactly(23),
        );
    }

    #[test]
    #[ignore]
    fn get_pit_stops_error_not_found() {
        assert_not_found(|| super::get_pit_stops(PitStopFilters::new(1949, 1)));
    }

    #[test]
    #[ignore]
    fn get_response_pit_stops_race_2023_4() {
        let resp = retry_http(|| get_response(&Resource::PitStops(PitStopFilters::new(2023, 4)))).unwrap();
        let race = verify_has_one_race_and_extract(resp).unwrap();

        assert!(eq_race_info(&race, &RACE_2023_4_PIT_STOPS));
        assert_eq!(race.payload.as_pit_stops().unwrap().len(), 23);
    }

    // Pagination, get_response_page, get_response, get_response_max_limit
    // -------------------------------------------------------------------

    #[test]
    #[ignore]
    fn get_response_single_page() {
        let resp = retry_http(|| get_response(&Resource::SeasonList(Filters::new().season(1950)))).unwrap();

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
        let resp = retry_http(|| get_response(&Resource::SeasonList(Filters::none())));
        assert!(matches!(resp, Err(Error::MultiPage)));
    }

    #[test]
    #[ignore]
    fn get_response_max_limit_single_page() {
        let resp = retry_http(|| get_response_max_limit(&Resource::SeasonList(Filters::none()))).unwrap();

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
        let resp = retry_http(|| get_response_max_limit(&Resource::LapTimes(LapTimeFilters::new(2023, 1))));
        assert!(matches!(resp, Err(Error::MultiPage)));
    }

    #[test]
    #[ignore]
    fn get_response_page_multi_page() {
        let req = Resource::SeasonList(Filters::none());
        let page = Page::with_limit(5);

        let mut resp = retry_http(|| get_response_page(&req, page.clone())).unwrap();
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

            resp = retry_http(|| get_response_page(&req, pagination.next_page().unwrap().into())).unwrap();

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
