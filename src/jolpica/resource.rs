//! Types to identify various [`Resource`]s and [`Filters`] that can be requested from the
//! [jolpica-f1](https://github.com/jolpica/jolpica-f1) API.
//!
//! These map directly to
//! [jolpica-f1 API endpoints](https://github.com/jolpica/jolpica-f1/blob/main/docs/README.md#endpoints-and-documentation)
//! and route parameters, e.g. for
//! [race results](https://github.com/jolpica/jolpica-f1/blob/main/docs/endpoints/results.md#route-parameters).

use url::Url;

use crate::{
    id::{CircuitID, ConstructorID, DriverID, RoundID, SeasonID, StatusID},
    jolpica::{api::JOLPICA_API_PAGINATION, response::Pagination},
};

#[cfg(doc)]
use crate::jolpica::{
    self, api,
    response::{
        Circuit, Constructor, Driver, QualifyingResult, Race, RaceResult, Response, Season, SprintResult, Status,
    },
};

/// Each variant of the [`Resource`] enumeration represents a given resource that can be requested
/// from the jolpica-f1 API, and it contains any options/filters that can be applied to the request.
// @todo Add examples once the `get_*` API has been settled
#[derive(Clone, Debug)]
pub enum Resource {
    /// Get a list of seasons currently supported by the API. Each season listed in a response is
    /// uniquely identified by the year it took place in, returned in [`Season::season`], e.g.
    /// `2023` for the _2023 Formula One World Championship_. The season year can be used to filter
    /// requests for other resources, via [`Filters::season`].
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/seasons/>
    SeasonList(Filters),

    /// Get a list of drivers within the series, and information about them. Each driver listed in
    /// a response is identified by a unique ID, returned in [`Driver::driver_id`], e.g. `"alonso"`
    /// for _Fernando Alonso_. These unique IDs can be used to filter requests for other resources,
    /// via [`Filters::driver_id`].
    ///
    /// **Note:** While the unique ID is usually the driver's surname, it may be different if it
    /// would be ambiguous, e.g. `"max_verstappen"` and `"verstappen"` identify _Max Verstappen_ and
    /// _Jos Verstappen_, respectively. As such, that convention should not be relied on; only
    /// values returned by the API are guaranteed to be valid.
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/drivers/>
    DriverInfo(Filters),

    #[allow(clippy::doc_markdown)] // False positive, complains about "_McLaren_".
    /// Get a list of constructors within the series, and information about them. Each constructor
    /// listed in a response is identified by a unique ID, returned in
    /// [`Constructor::constructor_id`], e.g. `"mclaren"` for _McLaren_. These unique IDs can be
    /// used to filter requests for other resources, via [`Filters::constructor_id`].
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/constructors/>
    ConstructorInfo(Filters),

    /// Get a list of circuits within the series, and information about them. Each circuit listed in
    /// a response is identified by a unique ID, returned in [`Circuit::circuit_id`], e.g. `"spa"`
    /// for _Circuit de Spa-Francorchamps_. These unique IDs can be used to filter requests for
    /// other resources, via [`Filters::circuit_id`].
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/circuits/>
    CircuitInfo(Filters),

    /// Get a schedule of races within the series, and information about them. Each race can be
    /// uniquely identified by the season year and round index, starting from `1`, returned in
    /// [`Race::season`] and [`Race::round`], respectively. These can be used to filter requests for
    /// other resources, via [`Filters::season`] and [`Filters::round`], respectively.
    ///
    /// **Note:** Schedule details before 2022 are limited to the date/time of the Grand Prix.
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/schedule/>
    RaceSchedule(Filters),

    /// Get a list of qualifying results. The qualifying position, returned in
    /// [`QualifyingResult::position`], can be used to filter requests for other resources, via
    /// [`Filters::qualifying_pos`].
    ///
    /// **Note:** Qualifying results are only fully supported from the 2003 season onwards.
    ///
    /// **Note:** The starting grid positions may be different to the qualifying positions, due to
    /// penalties, mechanical problems, and various sprint event configurations. The starting grid
    /// positions are recorded in [`SprintResult::grid`] and [`RaceResult::grid`] for sprints and
    /// races, respectively.
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/qualifying/>
    QualifyingResults(Filters),

    /// Get a list of sprint event results. Various of the returned value can be used to filter
    /// requests for other resources, via fields of [`Filters`]. The finishing position, returned in
    /// [`SprintResult::position`], can be used in [`Filters::sprint_pos`].
    ///
    /// **Note:** Sprint results are only available for races where there is a `Sprint` element in
    /// the schedule.
    ///
    /// **Note:** The value of [`SprintResult::position_text`] is either an integer
    /// (finishing position), “R” (retired), “D” (disqualified), “E” (excluded), “W” (withdrawn),
    /// “F” (failed to qualify) or “N” (not classified). Further information is given by
    /// [`SprintResult::status`].
    ///
    /// **Note:** A grid position of `0`, or [`api::GRID_PIT_LANE`], indicates that a driver started
    /// from the pit lane.
    ///
    /// **Note:** The [`Filters::sprint_pos`] field, which can be set to filter results based on
    /// [`SprintResult::position`], is only valid for results where the driver finished the sprint,
    /// i.e. where [`SprintResult::position_text`] is a numeric value.
    ///
    /// Directly maps to <https://ergast.com/mrd/methods/sprint/>
    SprintResults(Filters),

    /// Get a list of race results. Various of the returned values can be used to filter requests
    /// for other resources, via fields of [`Filters`]. The grid position, returned in
    /// [`RaceResult::grid`], can be used in [`Filters::grid_pos`]. The finishing position, returned
    /// in [`RaceResult::position`], can be used in [`Filters::finish_pos`].
    ///
    /// **Note:** The value of [`RaceResult::position_text`] is either an integer
    /// (finishing position), “R” (retired), “D” (disqualified), “E” (excluded), “W” (withdrawn),
    /// “F” (failed to qualify) or “N” (not classified). Further information is given by
    /// [`RaceResult::status`].
    ///
    /// **Note:** A grid position of `0`, or [`api::GRID_PIT_LANE`], indicates that a driver started
    /// from the pit lane.
    ///
    /// **Note:** Fastest lap times are included from the 2004 season onwards, returned in
    /// [`RaceResult::fastest_lap`].
    ///
    /// **Note:** The car number that a driver achieved a given result with, returned in
    /// [`RaceResult::number`], may differ from a driver's permanent number, since those were only
    /// implemented in the 2014 season onwards, and in cases where the reigning champion chose to
    /// use `1` rather than their permanent driver number. Drivers' permanent numbers, if they
    /// exist, are returned in [`Driver::permanent_number`].
    ///
    /// **Note:** The [`Filters::finish_pos`] field, which can be set to filter results based on
    /// [`RaceResult::position`], is only valid for results where the driver finished the race,
    /// i.e. where [`RaceResult::position_text`] is a numeric value.
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/results/>
    RaceResults(Filters),

    /// Get a list of finishing status codes supported by the API, as well as a count of the
    /// occurrence of each in a given period, e.g. season, race, etc.. While each status has a
    /// textual representation, e.g. `"Finished"`, `"Accident"`, `"Collision"`, etc., it is uniquely
    /// identified by a numeric ID, returned in [`Status::status_id`]. This unique ID can be used to
    /// filter requests for other resources, via [`Filters::finishing_status`].
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/status/>
    FinishingStatus(Filters),

    /// Get lap timing data for a given race.
    ///
    /// **Note:** Lap time data is available from the 1996 season onwards.
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/laps/>
    LapTimes(LapTimeFilters),

    /// Get pit stops data for a given race.
    ///
    /// **Note:** Pit stop data is available from the 2011 season onwards.
    ///
    /// Directly maps to <https://api.jolpi.ca/ergast/f1/pitstops/>
    PitStops(PitStopFilters),

    // These resources are not yet supported.
    #[doc(hidden)]
    DriverStandings,
    #[doc(hidden)]
    ConstructorStandings,
}

impl Resource {
    /// Produces a URL with which to request a given [`Resource`] from the jolpica-f1 API,
    /// including any filters that may have been requested.
    ///
    /// # Examples
    ///
    /// ```
    /// # use url::Url;
    /// # use f1_data::id::DriverID;
    /// # use f1_data::jolpica::resource::{Filters, Resource};
    /// #
    /// let request = Resource::DriverInfo(Filters {
    ///     driver_id: Some(DriverID::from("leclerc")),
    ///     ..Filters::none()
    /// });
    ///
    /// assert_eq!(
    ///     request.to_url(),
    ///     Url::parse("https://api.jolpi.ca/ergast/f1/drivers/leclerc.json").unwrap()
    /// );
    /// ```
    pub fn to_url(&self) -> Url {
        self.to_url_with_base_and_opt_page(crate::jolpica::api::JOLPICA_API_BASE_URL, None)
    }

    /// Produce a URL with which to request a specific [`Page`] of a given [`Resource`] from the
    /// jolpica-f1 API, including any filters that may have been requested.
    ///
    /// # Examples
    ///
    /// ```
    /// # use url::Url;
    /// # use f1_data::id::DriverID;
    /// # use f1_data::jolpica::resource::{Filters, Page, Resource};
    /// #
    /// let request = Resource::DriverInfo(Filters {
    ///     driver_id: Some(DriverID::from("leclerc")),
    ///     ..Filters::none()
    /// });
    ///
    /// assert_eq!(
    ///     request.to_url_with(Page::with_limit(100)),
    ///     Url::parse("https://api.jolpi.ca/ergast/f1/drivers/leclerc.json?limit=100&offset=0")
    ///         .unwrap()
    /// );
    /// ```
    pub fn to_url_with(&self, page: Page) -> Url {
        self.to_url_with_base_and_opt_page(crate::jolpica::api::JOLPICA_API_BASE_URL, Some(page))
    }

    /// Produces a URL with which to request, optionally a given [`Page`] of, a given [`Resource`]
    /// from a specified base URL, including any filters that may have been requested.
    ///
    /// This method is primarily intended for internal use, as the core implementation that the
    /// simpler [`to_url`][Self::to_url] and [`to_url_with`][Self::to_url_with] methods forward to.
    /// It is provided here to cover any edge use cases, e.g. requesting from alternate servers.
    ///
    /// # Examples
    ///
    /// ```
    /// # use url::Url;
    /// # use f1_data::id::DriverID;
    /// # use f1_data::jolpica::resource::{Filters, Page, Resource};
    /// #
    /// let request = Resource::DriverInfo(Filters {
    ///     driver_id: Some(DriverID::from("leclerc")),
    ///     ..Filters::none()
    /// });
    ///
    /// assert_eq!(
    ///     request.to_url_with_base_and_opt_page("https://example.com", Some(Page::with_limit(100))),
    ///     Url::parse("https://example.com/drivers/leclerc.json?limit=100&offset=0").unwrap()
    /// );
    /// ```
    pub fn to_url_with_base_and_opt_page(&self, base: &str, page: Option<Page>) -> Url {
        let mut url = Url::parse(&format!("{}{}.json", base, self.to_endpoint())).unwrap();

        if let Some(page) = page {
            // re. the lint, this use case is by design, according to `Url`'s docs.
            #[allow(unused_results)]
            let _ = url
                .query_pairs_mut()
                .extend_pairs([("limit", page.limit.to_string()), ("offset", page.offset.to_string())]);
        }

        url
    }

    /// Produces the endpoint path to request the given [`Resource`] from the jolpica-f1 API,
    /// including any filters that may have been requested.
    ///
    /// This method is primarily intended for internal use, as a building block used by all other
    /// `to_url_*` methods, e.g. [`to_url`][Self::to_url] and [`to_url_with`][Self::to_url_with].
    /// It is provided here to cover any edge use cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # use f1_data::id::{ConstructorID, DriverID};
    /// # use f1_data::jolpica::resource::{Filters, Resource};
    /// #
    /// let request = Resource::DriverInfo(Filters {
    ///     driver_id: Some(DriverID::from("leclerc")),
    ///     constructor_id: Some(ConstructorID::from("ferrari")),
    ///     ..Filters::none()
    /// });
    ///
    /// assert_eq!(request.to_endpoint(), "/constructors/ferrari/drivers/leclerc");
    /// ```
    pub fn to_endpoint(&self) -> String {
        type DynFF<'a> = &'a dyn FiltersFormatter;

        // re. the lints, I don't see a clean way to remove the cast without making the code worse
        // or using type ascription, which is De-RFCed: https://github.com/rust-lang/rfcs/pull/3307
        #[allow(trivial_casts)]
        let (resource_key, filters) = match self {
            Self::SeasonList(f) => ("/seasons", f as DynFF<'_>),
            Self::DriverInfo(f) => ("/drivers", f as DynFF<'_>),
            Self::ConstructorInfo(f) => ("/constructors", f as DynFF<'_>),
            Self::CircuitInfo(f) => ("/circuits", f as DynFF<'_>),
            Self::RaceSchedule(f) => ("/races", f as DynFF<'_>),
            Self::QualifyingResults(f) => ("/qualifying", f as DynFF<'_>),
            Self::SprintResults(f) => ("/sprint", f as DynFF<'_>),
            Self::RaceResults(f) => ("/results", f as DynFF<'_>),
            Self::FinishingStatus(f) => ("/status", f as DynFF<'_>),
            Self::LapTimes(f) => ("/laps", f as DynFF<'_>),
            Self::PitStops(f) => ("/pitstops", f as DynFF<'_>),
            _ => panic!("Unsupported resource: {self:?}"),
        };

        let mut filters = filters.to_formatted_pairs();

        // Move/add the resource key (which might also be a filter key) to/at the end, as that is
        // what the API expects to get the expected response, even if the resource key has a filter.
        let found = filters.iter().enumerate().find(|(_, f)| f.0 == resource_key);

        let resource = if let Some((idx, _)) = found {
            filters.remove(idx)
        } else {
            (resource_key, String::new())
        };

        filters.push(resource);

        filters
            .iter()
            .filter(|(key, val)| !val.is_empty() || key == &resource_key)
            .fold(String::new(), |mut acc, (key, val)| {
                acc.push_str(key);
                acc.push_str(val);
                acc
            })
    }
}

/// Trait that all filter structs for [`Resource`]s must implement, used to format resource URLs
trait FiltersFormatter {
    /// Return a list of (`resource_key`, `formatted_value`) for all possible filters
    fn to_formatted_pairs(&self) -> Vec<(&'static str, String)>;
}

/// Can be used to filter a given [`Resource`] from the jolpica-f1 API by a number of parameters,
/// identified by the struct fields, all of which are optional and can be set simultaneously.
///
/// Although most field combinations are valid, this interface makes no(few) efforts to verify or
/// enforce the validity of constructed combinations. Error checking is left up to the jolpica-f1
/// API and error handling should be done at the API call site, e.g. via the [`jolpica::agent`]
/// module. [`Filters`] objects can be constructed in multiple ways, which are demonstrated in the
/// examples and listed below:
///
///    1. Struct instantiation, explicitly setting all fields to `None` or `Some(value)`
///    2. Methods [`Filters::new`] and [`Filters::none`], both of which set all fields to `None`
///    3. _Struct update syntax_, setting relevant fields and filling the rest with above methods
///    4. Field-update methods, which can be chained, and each of which overrides a single field
///
/// # Examples
///
/// ```
/// use f1_data::id::{CircuitID, ConstructorID, DriverID, StatusID};
/// use f1_data::jolpica::resource::Filters;
///
/// let filters = Filters {
///     season: Some(2023),
///     round: Some(1),
///     driver_id: Some(DriverID::from("alonso")),
///     constructor_id: Some(ConstructorID::from("aston_martin")),
///     circuit_id: Some(CircuitID::from("baku")),
///     qualifying_pos: Some(6),
///     grid_pos: Some(6),
///     sprint_pos: None,
///     finish_pos: Some(4),
///     fastest_lap_rank: Some(3),
///     finishing_status: Some(StatusID::from(1u32)),
/// };
///
/// assert_eq!(filters.season, Some(2023));
/// assert_eq!(filters.round, Some(1));
/// /* ... */
///
/// let filters = Filters {
///     season: Some(2023),
///     round: Some(1),
///     ..Filters::none()
/// };
///
/// assert_eq!(filters.season, Some(2023));
/// assert_eq!(filters.round, Some(1));
/// assert!(filters.driver_id.is_none() /* ... */);
///
/// let filters = Filters::new().season(2023).round(1);
///
/// assert_eq!(filters.season, Some(2023));
/// assert_eq!(filters.round, Some(1));
/// assert!(filters.driver_id.is_none() /* ... */);
/// ```
#[must_use]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Filters {
    /// Restrict responses to a given championship season, identified by the year it took place in,
    /// e.g. `2023` for the _2023 Formula One World Championship_. See [`Resource::SeasonList`] to
    /// get a list of seasons currently supported by the API.
    pub season: Option<SeasonID>,

    /// Restrict responses to a specific race, identified by the round index starting from `1`, in a
    /// specific season. See [`Resource::RaceSchedule`] to get a list of rounds for a given season.
    ///
    /// **Note:** A [`Filters::season`] is required if this field is set, in order to uniquely
    /// identify a race.
    ///
    /// # Panics
    ///
    /// [`Filters::round`] being set requires that [`Filters::season`] be set as well, else
    /// certain methods may panic. The inverse is not true, [`Filters::season`] can be set
    /// without [`Filters::round`].
    pub round: Option<RoundID>,

    /// Restrict responses to those in which a given driver, identified by a unique ID, features,
    /// e.g. seasons or races in which the driver competed, constructors for which they drove, etc.
    /// See [`Resource::DriverInfo`] to get a list of all available driver IDs.
    pub driver_id: Option<DriverID>,

    /// Restrict responses to those in which a given constructors, identified by a unique ID,
    /// features, e.g. seasons or races in which the constructor competed, drivers that drove for
    /// them, etc. See [`Resource::ConstructorInfo`] to get a list of all available constructor IDs.
    pub constructor_id: Option<ConstructorID>,

    /// Restrict responses to those in which a given circuit, identified by a unique ID, features,
    /// e.g. races that took place in that circuit, seasons that held such a race, drivers that
    /// competed in that circuit, etc. See [`Resource::CircuitInfo`] to get a list of all available
    /// circuit IDs.
    pub circuit_id: Option<CircuitID>,

    /// Restrict responses to those in which a qualifying result with a specific position features,
    /// e.g. drivers or constructors that achieved a specific qualifying position, etc. See
    /// [`Resource::QualifyingResults`] for more information.
    pub qualifying_pos: Option<u32>,

    /// Restrict responses to those in which a race result with a specific grid position features,
    /// e.g. race results for all pole sitters, drivers that have started from a given position,
    /// etc. A grid position of `0`, or [`Filters::GRID_PIT_LANE`], indicates that a driver started
    /// from the pit lane. See [`Resource::RaceResults`] for more information.
    pub grid_pos: Option<u32>,

    /// Restrict responses to those in which a sprint result with a specific finishing position
    /// features, e.g. drivers that have won a sprint, etc. This is a numeric value, even if a
    /// driver did not finish a sprint. See [`Resource::SprintResults`] for more information.
    pub sprint_pos: Option<u32>,

    /// Restrict responses to those in which a race result with a specific finishing position
    /// features, e.g. drivers or constructors that have won a race, etc. This is a numeric value,
    /// even if a driver did not finish a race. See [`Resource::RaceResults`] for more information.
    pub finish_pos: Option<u32>,

    /// Restrict responses to those in which a given fastest lap rank, of a driver's fastest lap
    /// compared to other drivers' fastest laps, features, e.g. drivers that had the fastest lap
    /// in a race, etc. The rank starts at `1` for the fastest lap of a race.
    pub fastest_lap_rank: Option<u32>,

    /// Restrict responses to those that feature a specific finish status, e.g. race results where
    /// a driver had an `"Accident"`. This field should be the unique numeric ID for a finishing
    /// status, not the textual representation. See [`Resource::FinishingStatus`] to get a list of
    /// all supported unique finishing status codes.
    pub finishing_status: Option<StatusID>,
}

impl Filters {
    /// Value that can be set in [`Filters::grid_pos`] field, or updated with field-update method,
    /// to indicate a driver that started the race from the pit lane. See [`api::GRID_PIT_LANE`];
    pub const GRID_PIT_LANE: u32 = crate::jolpica::api::GRID_PIT_LANE;

    /// Returns a [`Filters`] object with all fields set to `None`, i.e. requesting no filtering.
    /// This method is identical to [`Filters::none`]; both are provided to maximize readability.
    pub const fn new() -> Self {
        Self::none()
    }

    /// Returns a [`Filters`] object with all fields set to `None`, i.e. requesting no filtering.
    /// This method is identical to [`Filters::new`]; both are provided to maximize readability.
    pub const fn none() -> Self {
        Self {
            season: None,
            round: None,
            driver_id: None,
            constructor_id: None,
            circuit_id: None,
            qualifying_pos: None,
            grid_pos: None,
            sprint_pos: None,
            finish_pos: None,
            fastest_lap_rank: None,
            finishing_status: None,
        }
    }

    /// Field-update method for the [`season`][field@Filters::season] field.
    pub fn season(self, season: SeasonID) -> Self {
        Self {
            season: Some(season),
            ..self
        }
    }

    /// Field-update method for the [`round`][field@Filters::round] field.
    pub fn round(self, round: RoundID) -> Self {
        Self {
            round: Some(round),
            ..self
        }
    }

    /// Field-update method for the [`driver_id`][field@Filters::driver_id] field.
    pub fn driver_id(self, driver_id: DriverID) -> Self {
        Self {
            driver_id: Some(driver_id),
            ..self
        }
    }

    /// Field-update method for the [`constructor_id`][field@Filters::constructor_id] field.
    pub fn constructor_id(self, constructor_id: ConstructorID) -> Self {
        Self {
            constructor_id: Some(constructor_id),
            ..self
        }
    }

    /// Field-update method for the [`circuit_id`][field@Filters::circuit_id] field.
    pub fn circuit_id(self, circuit_id: CircuitID) -> Self {
        Self {
            circuit_id: Some(circuit_id),
            ..self
        }
    }

    /// Field-update method for the [`qualifying_pos`][field@Filters::qualifying_pos] field.
    pub fn qualifying_pos(self, qualifying_pos: u32) -> Self {
        Self {
            qualifying_pos: Some(qualifying_pos),
            ..self
        }
    }

    /// Field-update method for the [`grid_pos`][field@Filters::grid_pos] field.
    pub fn grid_pos(self, grid_pos: u32) -> Self {
        Self {
            grid_pos: Some(grid_pos),
            ..self
        }
    }

    /// Field-update method for the [`sprint_pos`][field@Filters::sprint_pos] field.
    pub fn sprint_pos(self, sprint_pos: u32) -> Self {
        Self {
            sprint_pos: Some(sprint_pos),
            ..self
        }
    }

    /// Field-update method for the [`finish_pos`][field@Filters::finish_pos] field.
    pub fn finish_pos(self, finish_pos: u32) -> Self {
        Self {
            finish_pos: Some(finish_pos),
            ..self
        }
    }

    /// Field-update method for the [`fastest_lap_rank`][field@Filters::fastest_lap_rank] field.
    pub fn fastest_lap_rank(self, fastest_lap_rank: u32) -> Self {
        Self {
            fastest_lap_rank: Some(fastest_lap_rank),
            ..self
        }
    }

    /// Field-update method for the [`finishing_status`][field@Filters::finishing_status] field.
    pub fn finishing_status(self, finishing_status: StatusID) -> Self {
        Self {
            finishing_status: Some(finishing_status),
            ..self
        }
    }
}

impl Default for Filters {
    fn default() -> Self {
        Self::new()
    }
}

/// Can be used to filter [`Resource::LapTimes`] from the jolpica-f1 API by a number of required and
/// optional parameters, identified by the struct fields, which can be set simultaneously.
///
/// Except for some additional enforcement of required fields, e.g. [`LapTimeFilters::season`], and
/// a smaller supported subset of parameters, this type is meant to function similarly to
/// [`Filters`], so any similar usage is not documented. One key difference is the changes to the
/// signature of the [`new`](LapTimeFilters::new`) methods, which has parameters for the required
/// fields in this type.
///
/// # Examples
///
/// ```
/// # use f1_data::id::DriverID;
/// # use f1_data::jolpica::resource::LapTimeFilters;
/// #
/// let filters = LapTimeFilters {
///     season: 2023,
///     round: 4,
///     lap: Some(1),
///     driver_id: Some(DriverID::from("alonso")),
/// };
///
/// assert_eq!(filters.season, 2023);
/// assert_eq!(filters.lap, Some(1));
/// /* ... */
///
/// let filters = LapTimeFilters {
///     lap: Some(1),
///     ..LapTimeFilters::new(2023, 4)
/// };
///
/// assert_eq!(filters.season, 2023);
/// assert_eq!(filters.lap, Some(1));
/// /* ... */
///
/// let filters = LapTimeFilters::new(2023, 4).lap(1);
///
/// assert_eq!(filters.season, 2023);
/// assert_eq!(filters.lap, Some(1));
/// /* ... */
/// ```
#[must_use]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LapTimeFilters {
    /// Indicates a specific championship season, identified by the year it took place in.
    /// This is a required field, along with [`LapTimeFilters::round`], to uniquely identify a race.
    pub season: SeasonID,

    /// Indicates a specific race in a season, identified by the round index, starting from `1`.
    /// This is a required field, along with [`LapTimeFilters::season`], to uniquely identify a
    /// race.
    pub round: RoundID,

    /// Restrict responses to data for a single lap, identified by an index, starting from `1`.
    pub lap: Option<u32>,

    /// Restrict responses to data for a single driver's race laps, identified by a [`DriverID`].
    pub driver_id: Option<DriverID>,
}

impl LapTimeFilters {
    /// Returns a [`LapTimeFilters`] object with the required fields set as per the arguments, and
    /// the rest of the fields set to `None`, i.e. requesting no filtering on those parameters.
    pub const fn new(season: SeasonID, round: RoundID) -> Self {
        Self {
            season,
            round,
            lap: None,
            driver_id: None,
        }
    }

    /// Field-update method for the [`lap`][field@LapTimeFilters::lap] field.
    pub fn lap(self, lap: u32) -> Self {
        Self { lap: Some(lap), ..self }
    }

    /// Field-update method for the [`driver_id`][field@LapTimeFilters::driver_id] field.
    pub fn driver_id(self, driver_id: DriverID) -> Self {
        Self {
            driver_id: Some(driver_id),
            ..self
        }
    }
}

/// Can be used to filter [`Resource::PitStops`] from the jolpica-f1 API by a number of required and
/// optional parameters, identified by the struct fields, which can be set simultaneously.
///
/// Except for some additional enforcement of required fields, e.g. [`PitStopFilters::season`], and
/// a smaller supported subset of parameters, this type is meant to function similarly to
/// [`Filters`], so any similar usage is not documented. One key difference is the changes to the
/// signature of the [`new`](PitStopFilters::new`) method, which has parameters for the required
/// fields in this type.
///
/// # Examples
///
/// ```
/// # use f1_data::id::DriverID;
/// # use f1_data::jolpica::resource::PitStopFilters;
/// #
/// let filters = PitStopFilters {
///     season: 2023,
///     round: 4,
///     lap: Some(1),
///     driver_id: Some(DriverID::from("alonso")),
///     pit_stop: Some(1),
/// };
///
/// assert_eq!(filters.season, 2023);
/// assert_eq!(filters.pit_stop, Some(1));
/// /* ... */
///
/// let filters = PitStopFilters {
///     pit_stop: Some(1),
///     ..PitStopFilters::new(2023, 4)
/// };
///
/// assert_eq!(filters.season, 2023);
/// assert_eq!(filters.pit_stop, Some(1));
/// /* ... */
///
/// let filters = PitStopFilters::new(2023, 4).pit_stop(1);
///
/// assert_eq!(filters.season, 2023);
/// assert_eq!(filters.pit_stop, Some(1));
/// /* ... */
/// ```
#[must_use]
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PitStopFilters {
    /// Indicates a specific championship season, identified by the year it took place in.
    /// This is a required field, along with [`PitStopFilters::round`], to uniquely identify a race.
    pub season: SeasonID,

    /// Indicates a specific race in a season, identified by the round index, starting from `1`.
    /// This is a required field, along with [`PitStopFilters::season`], to uniquely identify a
    /// race.
    pub round: RoundID,

    /// Restrict responses to pit stops that took place on a specific lap, identified by an index,
    /// starting from `1`. The response will be empty if no pit stops took place in that lap.
    pub lap: Option<u32>,

    /// Restrict responses to pit stops for a single driver's car, identified by a [`DriverID`].
    pub driver_id: Option<DriverID>,

    /// Restrict responses to specific pit stops, identified by an index, starting from `1`. The
    /// response will be empty if not enough pit stops took place, e.g. `2` for a one-stop race.
    pub pit_stop: Option<u32>,
}

impl PitStopFilters {
    /// Returns a [`PitStopFilters`] object with the required fields set as per the arguments, and
    /// the rest of the fields set to [`None`], i.e. requesting no filtering on those parameters.
    pub const fn new(season: SeasonID, round: RoundID) -> Self {
        Self {
            season,
            round,
            lap: None,
            driver_id: None,
            pit_stop: None,
        }
    }

    /// Field-update method for the [`lap`][field@PitStopFilters::lap] field.
    pub fn lap(self, lap: u32) -> Self {
        Self { lap: Some(lap), ..self }
    }

    /// Field-update method for the [`driver_id`][field@PitStopFilters::driver_id] field.
    pub fn driver_id(self, driver_id: DriverID) -> Self {
        Self {
            driver_id: Some(driver_id),
            ..self
        }
    }

    /// Field-update method for the [`pit_stop`][field@PitStopFilters::pit_stop] field.
    pub fn pit_stop(self, pit_stop: u32) -> Self {
        Self {
            pit_stop: Some(pit_stop),
            ..self
        }
    }
}

#[allow(clippy::ref_option)] // Fix would be very verbose for little gain
/// Format a generic `Option<T>`; None as "", and Some(val) as "/{val}"
fn fmt_from_opt<T: std::fmt::Display>(field: &Option<T>) -> String {
    field.as_ref().map_or(String::new(), |val| format!("/{val}"))
}

impl FiltersFormatter for Filters {
    fn to_formatted_pairs(&self) -> Vec<(&'static str, String)> {
        // .round cannot be set without .season
        assert!(!(self.round.is_some() && self.season.is_none()));

        Vec::from([
            ("", fmt_from_opt(&self.season)),
            ("", fmt_from_opt(&self.round)),
            ("/drivers", fmt_from_opt(&self.driver_id)),
            ("/constructors", fmt_from_opt(&self.constructor_id)),
            ("/circuits", fmt_from_opt(&self.circuit_id)),
            ("/qualifying", fmt_from_opt(&self.qualifying_pos)),
            ("/grid", fmt_from_opt(&self.grid_pos)),
            ("/sprint", fmt_from_opt(&self.sprint_pos)),
            ("/results", fmt_from_opt(&self.finish_pos)),
            ("/fastest", fmt_from_opt(&self.fastest_lap_rank)),
            ("/status", fmt_from_opt(&self.finishing_status)),
        ])
    }
}

impl FiltersFormatter for LapTimeFilters {
    fn to_formatted_pairs(&self) -> Vec<(&'static str, String)> {
        Vec::from([
            ("", fmt_from_opt(&Some(self.season))),
            ("", fmt_from_opt(&Some(self.round))),
            ("/laps", fmt_from_opt(&self.lap)),
            ("/drivers", fmt_from_opt(&self.driver_id)),
        ])
    }
}

impl FiltersFormatter for PitStopFilters {
    fn to_formatted_pairs(&self) -> Vec<(&'static str, String)> {
        Vec::from([
            ("", fmt_from_opt(&Some(self.season))),
            ("", fmt_from_opt(&Some(self.round))),
            ("/laps", fmt_from_opt(&self.lap)),
            ("/drivers", fmt_from_opt(&self.driver_id)),
            ("/pitstops", fmt_from_opt(&self.pit_stop)),
        ])
    }
}

/// Identifies a specific pagination page for a given [`Resource`] from the jolpica-f1 API.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Page {
    limit: u32,
    offset: u32,
}

impl Page {
    /// Create an instance of [`Page`] with the given limit and offset.
    pub fn with(limit: u32, offset: u32) -> Self {
        assert!(limit <= JOLPICA_API_PAGINATION.max_limit);

        Self { limit, offset }
    }

    /// Create an instance of [`Page`] with the given offset and the default limit.
    pub fn with_offset(offset: u32) -> Self {
        Self::with(JOLPICA_API_PAGINATION.default_limit, offset)
    }

    /// Create an instance of [`Page`] with the given limit and the default offset.
    pub fn with_limit(limit: u32) -> Self {
        Self::with(limit, JOLPICA_API_PAGINATION.default_offset)
    }

    /// Create an instance of [`Page`] with the maximum limit and default offset.
    pub fn with_max_limit() -> Self {
        Self::with_limit(JOLPICA_API_PAGINATION.max_limit)
    }

    /// Access the limit of this [`Page`].
    pub const fn limit(&self) -> u32 {
        self.limit
    }

    /// Access the offset of this [`Page`].
    pub const fn offset(&self) -> u32 {
        self.offset
    }

    /// Return the next [`Page`] in the sequence, with the same limit, by incrementing the offset.
    #[must_use]
    pub const fn next(&self) -> Self {
        Self {
            offset: self.offset + self.limit,
            ..*self
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::with(JOLPICA_API_PAGINATION.default_limit, JOLPICA_API_PAGINATION.default_offset)
    }
}

impl From<Pagination> for Page {
    /// Create an instance of [`Page`] from one of [`Pagination`]
    fn from(pagination: Pagination) -> Self {
        Self::with(pagination.limit, pagination.offset)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::asserts::*;
    use shadow_asserts::assert_eq;

    use super::*;

    #[test]
    fn resource_jolpica_base_url() {
        assert_eq!(crate::jolpica::api::JOLPICA_API_BASE_URL, "https://api.jolpi.ca/ergast/f1")
    }

    fn url(tail: &str) -> Url {
        Url::parse(&format!("{}{}", crate::jolpica::api::JOLPICA_API_BASE_URL, tail)).unwrap()
    }

    #[test]
    fn resource_to_url_no_filters() {
        assert_eq!(Resource::SeasonList(Filters::none()).to_url(), url("/seasons.json"));
        assert_eq!(Resource::DriverInfo(Filters::none()).to_url(), url("/drivers.json"));
        assert_eq!(Resource::ConstructorInfo(Filters::none()).to_url(), url("/constructors.json"));
        assert_eq!(Resource::CircuitInfo(Filters::none()).to_url(), url("/circuits.json"));
        assert_eq!(Resource::RaceSchedule(Filters::none()).to_url(), url("/races.json"));
        assert_eq!(Resource::QualifyingResults(Filters::none()).to_url(), url("/qualifying.json"));
        assert_eq!(Resource::SprintResults(Filters::none()).to_url(), url("/sprint.json"));
        assert_eq!(Resource::RaceResults(Filters::none()).to_url(), url("/results.json"));
        assert_eq!(Resource::FinishingStatus(Filters::none()).to_url(), url("/status.json"));
    }

    #[test]
    fn resource_to_url_resource_filter() {
        assert_eq!(
            Resource::DriverInfo(Filters {
                driver_id: Some("leclerc".into()),
                ..Filters::none()
            })
            .to_url(),
            url("/drivers/leclerc.json")
        );

        assert_eq!(
            Resource::QualifyingResults(Filters {
                qualifying_pos: Some(1),
                ..Filters::none()
            })
            .to_url(),
            url("/qualifying/1.json")
        );

        assert_eq!(
            Resource::SprintResults(Filters {
                sprint_pos: Some(1),
                ..Filters::none()
            })
            .to_url(),
            url("/sprint/1.json")
        );

        assert_eq!(
            Resource::RaceResults(Filters {
                finish_pos: Some(1),
                ..Filters::none()
            })
            .to_url(),
            url("/results/1.json")
        );

        assert_eq!(
            Resource::FinishingStatus(Filters {
                finishing_status: Some(1),
                ..Filters::none()
            })
            .to_url(),
            url("/status/1.json")
        );
    }

    #[test]
    fn resource_to_url_non_resource_filters() {
        assert_eq!(
            Resource::SeasonList(Filters {
                driver_id: Some("leclerc".into()),
                ..Filters::none()
            })
            .to_url(),
            url("/drivers/leclerc/seasons.json")
        );

        assert_eq!(
            Resource::DriverInfo(Filters {
                constructor_id: Some("ferrari".into()),
                circuit_id: Some("spa".into()),
                qualifying_pos: Some(1),
                ..Filters::none()
            })
            .to_url(),
            url("/constructors/ferrari/circuits/spa/qualifying/1/drivers.json")
        );
    }

    #[test]
    fn resource_to_url_mixed_filters() {
        assert_eq!(
            Resource::DriverInfo(Filters {
                driver_id: Some("leclerc".into()),
                constructor_id: Some("ferrari".into()),
                circuit_id: Some("spa".into()),
                qualifying_pos: Some(1),
                ..Filters::none()
            })
            .to_url(),
            url("/constructors/ferrari/circuits/spa/qualifying/1/drivers/leclerc.json")
        );
    }

    #[test]
    fn resource_to_url_season_round_filters() {
        assert_eq!(
            Resource::DriverInfo(Filters {
                season: Some(2023),
                ..Filters::none()
            })
            .to_url(),
            url("/2023/drivers.json")
        );

        assert_eq!(
            Resource::SeasonList(Filters {
                season: Some(2023),
                round: Some(1),
                ..Filters::none()
            })
            .to_url(),
            url("/2023/1/seasons.json")
        );

        assert_eq!(
            Resource::RaceSchedule(Filters {
                season: Some(2023),
                round: Some(4),
                ..Filters::none()
            })
            .to_url(),
            url("/2023/4/races.json")
        );
    }

    #[test]
    fn resource_to_url_with_page() {
        assert_eq!(
            Resource::DriverInfo(Filters::none()).to_url_with(Page::with(10, 5)),
            url("/drivers.json?limit=10&offset=5")
        );

        assert_eq!(
            Resource::DriverInfo(Filters::none()).to_url_with(Page::with_offset(10)),
            url("/drivers.json?limit=30&offset=10")
        );

        assert_eq!(
            Resource::DriverInfo(Filters::none()).to_url_with(Page::with_max_limit()),
            url("/drivers.json?limit=100&offset=0")
        );
    }

    #[test]
    #[should_panic]
    fn resource_to_url_round_without_season_filter_panics() {
        let _unused = Resource::RaceSchedule(Filters {
            round: Some(1),
            ..Filters::none()
        })
        .to_url();
    }

    #[test]
    fn resource_lap_times_to_url() {
        assert_eq!(Resource::LapTimes(LapTimeFilters::new(2023, 4)).to_url(), url("/2023/4/laps.json"));

        assert_eq!(
            Resource::LapTimes(LapTimeFilters {
                season: 2023,
                round: 4,
                lap: Some(1),
                driver_id: Some("alonso".into())
            })
            .to_url(),
            url("/2023/4/drivers/alonso/laps/1.json")
        );
    }

    #[test]
    fn resource_pit_stops_to_url() {
        assert_eq!(Resource::PitStops(PitStopFilters::new(2023, 4)).to_url(), url("/2023/4/pitstops.json"));

        assert_eq!(
            Resource::PitStops(PitStopFilters {
                season: 2023,
                round: 4,
                lap: Some(1),
                driver_id: Some("alonso".into()),
                pit_stop: Some(1),
            })
            .to_url(),
            url("/2023/4/laps/1/drivers/alonso/pitstops/1.json")
        );
    }

    #[test]
    fn resource_to_url_with_base_and_opt_page() {
        assert_eq!(
            Resource::DriverInfo(Filters::none()).to_url_with_base_and_opt_page("https://example.com/api", None),
            Url::parse("https://example.com/api/drivers.json").unwrap()
        );

        assert_eq!(
            Resource::DriverInfo(Filters::none())
                .to_url_with_base_and_opt_page("https://example.com/api", Some(Page::with(10, 5))),
            Url::parse("https://example.com/api/drivers.json?limit=10&offset=5").unwrap()
        );
    }

    #[test]
    fn resource_to_endpoint() {
        assert_eq!(
            Resource::DriverInfo(Filters {
                constructor_id: Some("ferrari".into()),
                circuit_id: Some("spa".into()),
                qualifying_pos: Some(1),
                ..Filters::none()
            })
            .to_endpoint(),
            "/constructors/ferrari/circuits/spa/qualifying/1/drivers"
        );
    }

    #[test]
    fn filters() {
        let filters = Filters::none();
        assert_true!(
            filters.season.is_none()
                && filters.round.is_none()
                && filters.driver_id.is_none()
                && filters.constructor_id.is_none()
                && filters.circuit_id.is_none()
                && filters.qualifying_pos.is_none()
                && filters.grid_pos.is_none()
                && filters.sprint_pos.is_none()
                && filters.finish_pos.is_none()
                && filters.fastest_lap_rank.is_none()
                && filters.finishing_status.is_none()
        );

        let filters = Filters {
            driver_id: Some("alonso".into()),
            circuit_id: Some("spa".into()),
            ..Filters::none()
        };
        assert_eq!(filters.driver_id, Some("alonso".into()));
        assert_eq!(filters.circuit_id, Some("spa".into()));

        assert_true!(
            filters.season.is_none()
                && filters.round.is_none()
                && filters.constructor_id.is_none()
                && filters.qualifying_pos.is_none()
                && filters.grid_pos.is_none()
                && filters.sprint_pos.is_none()
                && filters.finish_pos.is_none()
                && filters.fastest_lap_rank.is_none()
                && filters.finishing_status.is_none()
        );

        let filters = Filters::new().driver_id("alonso".into()).circuit_id("spa".into());
        assert_eq!(filters.driver_id, Some("alonso".into()));
        assert_eq!(filters.circuit_id, Some("spa".into()));

        assert_true!(
            filters.season.is_none()
                && filters.round.is_none()
                && filters.constructor_id.is_none()
                && filters.qualifying_pos.is_none()
                && filters.grid_pos.is_none()
                && filters.sprint_pos.is_none()
                && filters.finish_pos.is_none()
                && filters.fastest_lap_rank.is_none()
                && filters.finishing_status.is_none()
        );

        assert_eq!(
            Filters {
                season: Some(2023),
                round: Some(1),
                driver_id: Some("alonso".into()),
                constructor_id: Some("aston_martin".into()),
                circuit_id: Some("baku".into()),
                qualifying_pos: Some(6),
                grid_pos: Some(6),
                sprint_pos: Some(1),
                finish_pos: Some(4),
                fastest_lap_rank: Some(3),
                finishing_status: Some(1),
            },
            Filters::new()
                .season(2023)
                .round(1)
                .driver_id("alonso".into())
                .constructor_id("aston_martin".into())
                .circuit_id("baku".into())
                .qualifying_pos(6)
                .grid_pos(6)
                .sprint_pos(1)
                .finish_pos(4)
                .fastest_lap_rank(3)
                .finishing_status(1)
        );
    }

    #[test]
    fn lap_time_filters() {
        let filters = LapTimeFilters::new(2023, 4);
        assert_eq!(filters.season, 2023);
        assert_eq!(filters.round, 4);
        assert_true!(filters.lap.is_none() && filters.driver_id.is_none());

        let filters = LapTimeFilters {
            lap: Some(1),
            ..LapTimeFilters::new(2023, 4)
        };
        assert_eq!(filters.season, 2023);
        assert_eq!(filters.round, 4);
        assert_eq!(filters.lap, Some(1));
        assert_true!(filters.driver_id.is_none());

        assert_eq!(
            LapTimeFilters {
                season: 2023,
                round: 4,
                lap: Some(1),
                driver_id: Some("alonso".into()),
            },
            LapTimeFilters::new(2023, 4).lap(1).driver_id("alonso".into())
        );
    }

    #[test]
    fn pit_stop_filters() {
        let filters = PitStopFilters::new(2023, 4);
        assert_eq!(filters.season, 2023);
        assert_eq!(filters.round, 4);
        assert_true!(filters.lap.is_none() && filters.driver_id.is_none() && filters.pit_stop.is_none());

        let filters = PitStopFilters {
            lap: Some(1),
            ..PitStopFilters::new(2023, 4)
        };
        assert_eq!(filters.season, 2023);
        assert_eq!(filters.round, 4);
        assert_eq!(filters.lap, Some(1));
        assert_true!(filters.driver_id.is_none() && filters.pit_stop.is_none());

        assert_eq!(
            PitStopFilters {
                season: 2023,
                round: 4,
                lap: Some(1),
                driver_id: Some("alonso".into()),
                pit_stop: Some(1),
            },
            PitStopFilters::new(2023, 4)
                .lap(1)
                .driver_id("alonso".into())
                .pit_stop(1)
        );
    }

    #[test]
    fn filters_to_formatted_pairs_lifetime() {
        let &mut formatted_pairs;

        {
            formatted_pairs = Filters::none().to_formatted_pairs();
        }

        assert_false!(formatted_pairs.is_empty());
        assert_eq!(formatted_pairs[0].0, "");
    }

    #[test]
    fn page_construction() {
        assert_eq!(Page::with(20, 5), Page { limit: 20, offset: 5 });
        assert_eq!(Page::with_limit(20), Page { limit: 20, offset: 0 });

        assert_eq!(
            Page::with_offset(5),
            Page {
                limit: JOLPICA_API_PAGINATION.default_limit,
                offset: 5
            }
        );

        assert_eq!(
            Page::with_max_limit(),
            Page {
                limit: JOLPICA_API_PAGINATION.max_limit,
                offset: JOLPICA_API_PAGINATION.default_offset
            }
        );

        assert_eq!(
            Page::default(),
            Page {
                limit: JOLPICA_API_PAGINATION.default_limit,
                offset: JOLPICA_API_PAGINATION.default_offset
            }
        );
    }

    #[test]
    #[should_panic]
    fn page_construction_panics() {
        let _ = Page::with_limit(2000);
    }

    #[test]
    fn page_next() {
        assert_eq!(Page::with(30, 0).next(), Page::with(30, 30));
        assert_eq!(Page::with(30, 10).next(), Page::with(30, 40));
    }

    #[test]
    fn page_from_pagination() {
        assert_eq!(
            Page::from(Pagination {
                limit: 30,
                offset: 10,
                total: 100
            }),
            Page::with(30, 10)
        );
    }
}
