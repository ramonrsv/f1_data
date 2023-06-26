use url::Url;

use crate::{
    ergast::response::Pagination,
    id::{CircuitID, ConstructorID, DriverID, RoundID, SeasonID, StatusID},
};

/// Each variant of the [`Resource`] enumeration represents a given resource that can be requested
/// from the Ergast API, and it contains any options or filters that can be applied to the request.
// @todo Add examples once the `get_*` API has been settled
#[derive(Clone, Debug)]
pub enum Resource {
    /// Get a list of seasons currently supported by the API. Each season listed in a response is
    /// uniquely identified by the year it took place in, returned in
    /// [`response::Season::season`](crate::ergast::response::Season::season), e.g. `2023` for the
    /// _2023 Formula One World Championship_. The season year can be used to filter requests for
    /// other resources, via [`Filters::season`].
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/seasons/>
    SeasonList(Filters),

    /// Get a list of drivers within the series, and information about them. Each driver listed in
    /// a response is identified by a unique ID, returned in
    /// [`response::Driver::driver_id`](crate::ergast::response::Driver::driver_id), e.g. `"alonso"`
    /// for _Fernando Alonso_. These unique IDs can be used to filter requests for other resources,
    /// via [`Filters::driver_id`].
    ///
    /// **Note:** While the unique ID is usually the driver's surname, it may be different if it
    /// would be ambiguous, e.g. `"max_verstappen"` and `"verstappen"` identify _Max Verstappen_ and
    /// _Jos Verstappen_, respectively. As such, that convention should not be relied on; only
    /// values returned by the API are guaranteed to be valid.
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/drivers/>
    DriverInfo(Filters),

    /// Get a list of constructors within the series, and information about them. Each constructor
    /// listed in a response is identified by a unique ID, returned in
    /// [`response::Constructor::constructor_id`](crate::ergast::response::Constructor::constructor_id),
    /// e.g. `"mclaren"` for _McLaren_. These unique IDs can be used to filter requests for other
    /// resources, via [`Filters::constructor_id`].
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/constructors/>
    ConstructorInfo(Filters),

    /// Get a list of circuits within the series, and information about them. Each circuit listed in
    /// a response is identified by a unique ID, returned in
    /// [`response::Circuit::circuit_id`](crate::ergast::response::Circuit::circuit_id), e.g.
    /// `"spa"` for _Circuit de Spa-Francorchamps_. These unique IDs can be used to filter requests
    /// for other resources, via [`Filters::circuit_id`].
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/circuits/>
    CircuitInfo(Filters),

    /// Get a schedule of races within the series, and information about them. Each race can be
    /// uniquely identified by the season year and round index, starting from `1`, returned in
    /// [`response::Race::season`](crate::ergast::response::Race::season) and
    /// [`response::Race::round`](crate::ergast::response::Race::round), respectively. These can be
    /// used to filter requests for other resources, via [`Filters::season`] and [`Filters::round`],
    /// respectively.
    ///
    /// **Note:** Schedule details before 2022 are limited to the date/time of the Grand Prix.
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/schedule/>
    RaceSchedule(Filters),

    /// Get a list of qualifying results. The qualifying position, returned in
    /// [`response::QualifyingResult::position`](crate::ergast::response::QualifyingResult::position),
    /// can be used to filter requests for other resources, via [`Filters::qualifying_pos`].
    ///
    /// **Note:** Qualifying results are only fully supported from the 2003 season onwards.
    ///
    /// **Note:** The starting grid positions may be different to the qualifying positions, due to
    /// penalties, mechanical problems, and various sprint event configurations. The starting grid
    /// positions are recorded in
    /// [`response::SprintResult::grid`](crate::ergast::response::SprintResult::grid) and
    /// [`response::RaceResult::grid`](crate::ergast::response::RaceResult::grid) for sprints and
    /// races, respectively.
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/qualifying/>
    QualifyingResults(Filters),

    /// Get a list of sprint event results.
    ///
    /// **Note:** Sprint results are only available for races where there is a `Sprint` element in
    /// the schedule.
    ///
    /// **Note:** The value of
    /// [`response::SprintResult::position_text`](crate::ergast::response::SprintResult::position_text)
    /// is either an integer (finishing position), “R” (retired), “D” (disqualified),
    /// “E” (excluded), “W” (withdrawn), “F” (failed to qualify) or “N” (not classified). Further
    /// information is given by
    /// [`response::SprintResult::status`](crate::ergast::response::SprintResult::status).
    ///
    /// **Note:** A grid position of `0`, or [`Filters::GRID_PIT_LANE`], indicates that a driver
    /// started from the pit lane.
    ///
    /// Directly maps to <https://ergast.com/mrd/methods/sprint/>
    SprintResults(Filters),

    /// Get a list of race results. Various of the returned values can be used to filter requests
    /// for other resources, via fields of [`Filters`]. The grid position, returned in
    /// [`response::RaceResult::grid`](crate::ergast::response::RaceResult::grid), can be used in
    /// [`Filters::grid_pos`]. The finishing position, returned in
    /// [`response::RaceResult::position`](crate::ergast::response::RaceResult::position), can be
    /// used in [`Filters::finish_pos`].
    ///
    /// **Note:** The value of
    /// [`response::RaceResult::position_text`](crate::ergast::response::RaceResult::position_text)
    /// is either an integer (finishing position), “R” (retired), “D” (disqualified),
    /// “E” (excluded), “W” (withdrawn), “F” (failed to qualify) or “N” (not classified). Further
    /// information is given by
    /// [`response::RaceResult::status`](crate::ergast::response::RaceResult::status).
    ///
    /// **Note:** A grid position of `0`, or [`Filters::GRID_PIT_LANE`], indicates that a driver
    /// started from the pit lane.
    ///
    /// **Note:** Fastest lap times are included from the 2004 season onwards, returned in
    /// [`response::RaceResult::fastest_lap`](crate::ergast::response::RaceResult::fastest_lap)
    ///
    /// **Note:** The car number that a driver achieved a given result with, returned in
    /// [`response::RaceResult::number`](crate::ergast::response::RaceResult::number), may differ
    /// from a driver's permanent number, since those were only implemented in the 2014 season
    /// onwards, and in cases where the reigning champion chose to use `1` rather than their
    /// permanent driver number. Drivers' permanent numbers, if they exist, are returned in
    /// [`response::Driver::permanent_number`](crate::ergast::response::Driver::permanent_number).
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/results/>
    RaceResults(Filters),

    /// Get a list of finishing status codes supported by the API, as well as a count of the
    /// occurrence of each in a given period, e.g. season, race, etc.. While each status has a
    /// textual representation, e.g. `"Finished"`, `"Accident"`, `"Collision"`, etc., it is uniquely
    /// identified by a numeric ID, returned in
    /// [`response::Status::status_id`](crate::ergast::response::Status::status_id). This unique ID
    /// can be used to filter requests for other resources, via [`Filters::finishing_status`].
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/status/>
    FinishingStatus(Filters),

    /// Get lap timing data for a given race.
    ///
    /// **Note:** Lap time data is available from the 1996 season onwards.
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/laps/>
    LapTimes(LapTimeFilters),

    /// Get pit stops data for a given race.
    ///
    /// **Note:** Pit stop data is available from the 2012 season onwards.
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/pitstops/>
    PitStops(PitStopFilters),

    // These resources are not yet supported.
    #[doc(hidden)]
    DriverStandings,
    #[doc(hidden)]
    ConstructorStandings,
}

impl Resource {
    /// The base URL at which requests will be made for Ergast's RESTful API.
    // @todo This should probably be configurable, e.g. to support mirrors, caches, etc.
    pub const ERGAST_BASE_URL: &str = "http://ergast.com/api/f1";

    /// Produce a URL with which to request the given [`Resource`] from Ergast's RESTful API,
    /// including any filters that may have been requested.
    ///
    /// # Examples
    ///
    /// ```
    /// # use url::Url;
    /// use f1_data::id::DriverID;
    /// use f1_data::ergast::resource::{Resource, Filters};
    ///
    /// let request = Resource::DriverInfo(Filters {
    ///     driver_id: Some(DriverID::from("leclerc")),
    ///     ..Filters::none()
    /// });
    ///
    /// assert_eq!(
    ///     request.to_url(),
    ///     Url::parse("http://ergast.com/api/f1/drivers/leclerc.json").unwrap()
    /// );
    /// ```
    pub fn to_url(&self) -> Url {
        type DynFF<'a> = &'a dyn FiltersFormatter;

        let (resource_key, filters) = match self {
            Self::SeasonList(f) => ("/seasons", f as DynFF),
            Self::DriverInfo(f) => ("/drivers", f as DynFF),
            Self::ConstructorInfo(f) => ("/constructors", f as DynFF),
            Self::CircuitInfo(f) => ("/circuits", f as DynFF),
            Self::RaceSchedule(f) => ("/races", f as DynFF),
            Self::QualifyingResults(f) => ("/qualifying", f as DynFF),
            Self::SprintResults(f) => ("/sprint", f as DynFF),
            Self::RaceResults(f) => ("/results", f as DynFF),
            Self::FinishingStatus(f) => ("/status", f as DynFF),
            Self::LapTimes(f) => ("/laps", f as DynFF),
            Self::PitStops(f) => ("/pitstops", f as DynFF),
            _ => panic!("Unsupported resource: {:?}", self),
        };

        let mut filters = filters.to_formatted_pairs();

        // Move/add the resource key (which might also be a filter key) to/at the end, as that is
        // what the API expects to get the expected response, even if the resource key has a filter.
        let found = filters.iter().enumerate().find(|(_, f)| f.0 == resource_key);

        let resource = if let Some((idx, _)) = found {
            filters.remove(idx)
        } else {
            (resource_key, "".to_string())
        };

        filters.push(resource);

        Url::parse(&format!(
            "{}{}.json",
            Resource::ERGAST_BASE_URL,
            filters
                .iter()
                .filter(|(key, val)| !val.is_empty() || key == &resource_key)
                .map(|(key, val)| format!("{}{}", key, val))
                .collect::<Vec<_>>()
                .join("")
        ))
        .unwrap()
    }

    pub fn to_url_with(&self, page: Page) -> Url {
        let mut url = self.to_url();

        url.query_pairs_mut()
            .extend_pairs([("limit", page.limit.to_string()), ("offset", page.offset.to_string())]);

        url
    }
}

/// Trait that all filter structs for [`Resource`]s must implement, used to format resource URLs
trait FiltersFormatter {
    /// Return a list of (<resource_key>, <formatted_value>) for all possible filters
    fn to_formatted_pairs(&self) -> Vec<(&'static str, String)>;
}

/// Can be used to filter a given [`Resource`] from the Ergast API by a number of parameters,
/// identified by the struct fields, all of which are optional and can be set simultaneously.
///
/// Although most field combinations are valid, this interface makes no(few) efforts to verify or
/// enforce the validity of constructed combinations. Error checking is left up to the Ergast API
/// and error handling should be done at the API call site, e.g. via the [`crate::ergast::get`]
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
/// use f1_data::ergast::resource::Filters;
///
/// let filters = Filters {
///     season: Some(2023),
///     round: Some(1),
///     driver_id: Some(DriverID::from("alonso")),
///     constructor_id: Some(ConstructorID::from("aston_martin")),
///     circuit_id: Some(CircuitID::from("baku")),
///     qualifying_pos: Some(6),
///     grid_pos: Some(6),
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
    /// to indicate a driver that started the race from the pit lane.
    pub const GRID_PIT_LANE: u32 = crate::ergast::response::GRID_PIT_LANE;

    /// Returns a [`Filters`] object with all fields set to `None`, i.e. requesting no filtering.
    /// This method is identical to [`Filters::none`]; both are provided to maximize readability.
    pub fn new() -> Self {
        Self::none()
    }

    /// Returns a [`Filters`] object with all fields set to `None`, i.e. requesting no filtering.
    /// This method is identical to [`Filters::new`]; both are provided to maximize readability.
    pub fn none() -> Self {
        Self {
            season: None,
            round: None,
            driver_id: None,
            constructor_id: None,
            circuit_id: None,
            qualifying_pos: None,
            grid_pos: None,
            finish_pos: None,
            fastest_lap_rank: None,
            finishing_status: None,
        }
    }

    /// Field-update method for the `season` field.
    pub fn season(self, season: SeasonID) -> Self {
        Self {
            season: Some(season),
            ..self
        }
    }

    /// Field-update method for the `round` field.
    pub fn round(self, round: RoundID) -> Self {
        Self {
            round: Some(round),
            ..self
        }
    }

    /// Field-update method for the `driver_id` field.
    pub fn driver_id(self, driver_id: DriverID) -> Self {
        Self {
            driver_id: Some(driver_id),
            ..self
        }
    }

    /// Field-update method for the `constructor_id` field.
    pub fn constructor_id(self, constructor_id: ConstructorID) -> Self {
        Self {
            constructor_id: Some(constructor_id),
            ..self
        }
    }

    /// Field-update method for the `circuit_id` field.
    pub fn circuit_id(self, circuit_id: CircuitID) -> Self {
        Self {
            circuit_id: Some(circuit_id),
            ..self
        }
    }

    /// Field-update method for the `qualifying_pos` field.
    pub fn qualifying_pos(self, qualifying_pos: u32) -> Self {
        Self {
            qualifying_pos: Some(qualifying_pos),
            ..self
        }
    }

    /// Field-update method for the `grid_pos` field.
    pub fn grid_pos(self, grid_pos: u32) -> Self {
        Self {
            grid_pos: Some(grid_pos),
            ..self
        }
    }

    /// Field-update method for the `finish_pos` field.
    pub fn finish_pos(self, finish_pos: u32) -> Self {
        Self {
            finish_pos: Some(finish_pos),
            ..self
        }
    }

    /// Field-update method for the `fastest_lap_rank` field.
    pub fn fastest_lap_rank(self, fastest_lap_rank: u32) -> Self {
        Self {
            fastest_lap_rank: Some(fastest_lap_rank),
            ..self
        }
    }

    /// Field-update method for the `finishing_status` field.
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

/// Can be used to filter [`Resource::LapTimes`] from the Ergast API by a number of parameters,
/// identified by the struct fields, some of which are required, and the rest are optional and can
/// be set simultaneously.
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
/// use f1_data::id::DriverID;
/// use f1_data::ergast::resource::LapTimeFilters;
///
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
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LapTimeFilters {
    /// Indicates a specific championship season, identified by the year it took place in.
    /// This is a required field, along with [`LapTimeFilters::round`], to uniquely identify a race.
    pub season: SeasonID,

    /// Indicates a specific race in a season, identified by the round index, starting from `1`.
    /// This is a required field, along with [`LapTimeFilters::season`], to uniquely identify a race.
    pub round: RoundID,

    /// Restrict responses to data for a single lap, identified by an index, starting from `1`.
    pub lap: Option<u32>,

    /// Restrict responses to data for a single driver's race laps, identified by a unique ID.
    pub driver_id: Option<DriverID>,
}

impl LapTimeFilters {
    /// Returns a [`LapTimeFilters`] object with the required fields set as per the arguments, and
    /// the rest of the fields set to `None`, i.e. requesting no filtering on those parameters.
    pub fn new(season: SeasonID, round: RoundID) -> Self {
        Self {
            season,
            round,
            lap: None,
            driver_id: None,
        }
    }

    /// Field-update method for the `lap` field.
    pub fn lap(self, lap: u32) -> Self {
        Self { lap: Some(lap), ..self }
    }

    /// Field-update method for the `driver_id` field.
    pub fn driver_id(self, driver_id: DriverID) -> Self {
        Self {
            driver_id: Some(driver_id),
            ..self
        }
    }
}

/// Can be used to filter [`Resource::PitStops`] from the Ergast API by a number of parameters,
/// identified by the struct fields, some of which are required, and the rest are optional and can
/// be set simultaneously.
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
/// use f1_data::id::DriverID;
/// use f1_data::ergast::resource::PitStopFilters;
///
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
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct PitStopFilters {
    /// Indicates a specific championship season, identified by the year it took place in.
    /// This is a required field, along with [`PitStopFilters::round`], to uniquely identify a race.
    pub season: SeasonID,

    /// Indicates a specific race in a season, identified by the round index, starting from `1`.
    /// This is a required field, along with [`PitStopFilters::season`], to uniquely identify a race.
    pub round: RoundID,

    /// Restrict responses to pit stops that took place on a specific lap, identified by an index,
    /// starting from `1`. The response will be empty if no pit stops took place in that lap.
    pub lap: Option<u32>,

    /// Restrict responses to pit stops for a single driver's car, identified by a unique ID.
    pub driver_id: Option<DriverID>,

    /// Restrict responses to specific pit stops, identified by an index, starting from `1`. The
    /// response will be empty if not enough pit stops took place, e.g. `2` for a one-stop race.
    pub pit_stop: Option<u32>,
}

impl PitStopFilters {
    /// Returns a [`PitStopFilters`] object with the required fields set as per the arguments, and
    /// the rest of the fields set to `None`, i.e. requesting no filtering on those parameters.
    pub fn new(season: SeasonID, round: RoundID) -> Self {
        Self {
            season,
            round,
            lap: None,
            driver_id: None,
            pit_stop: None,
        }
    }

    /// Field-update method for the `lap` field.
    pub fn lap(self, lap: u32) -> Self {
        Self { lap: Some(lap), ..self }
    }

    /// Field-update method for the `driver_id` field.
    pub fn driver_id(self, driver_id: DriverID) -> Self {
        Self {
            driver_id: Some(driver_id),
            ..self
        }
    }

    /// Field-update method for the `pit_stop` field.
    pub fn pit_stop(self, pit_stop: u32) -> Self {
        Self {
            pit_stop: Some(pit_stop),
            ..self
        }
    }
}

/// Format a generic `Option<T>`; None as "", and Some(val) as "/{val}"
fn fmt_from_opt<T: std::fmt::Display>(field: &Option<T>) -> String {
    if let Some(val) = field {
        format!("/{val}")
    } else {
        String::new()
    }
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Page {
    limit: u32,
    offset: u32,
}

impl Page {
    pub const DEFAULT_LIMIT: u32 = 30;
    pub const DEFAULT_OFFSET: u32 = 0;

    pub const MAX_LIMIT: u32 = 1000;

    pub fn with(limit: u32, offset: u32) -> Self {
        assert!(limit <= Self::MAX_LIMIT);

        Self { limit, offset }
    }

    pub fn with_offset(offset: u32) -> Self {
        Self::with(Self::DEFAULT_LIMIT, offset)
    }

    pub fn with_limit(limit: u32) -> Self {
        Self::with(limit, Self::DEFAULT_OFFSET)
    }

    pub fn with_max_limit() -> Self {
        Self::with_limit(Self::MAX_LIMIT)
    }

    pub fn limit(&self) -> u32 {
        self.limit
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn next(&self) -> Self {
        Self {
            offset: self.offset + self.limit,
            ..*self
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::with(Self::DEFAULT_LIMIT, Self::DEFAULT_OFFSET)
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
    use super::*;

    #[test]
    fn resource_ergast_base_url() {
        assert_eq!(Resource::ERGAST_BASE_URL, "http://ergast.com/api/f1")
    }

    fn url(tail: &str) -> Url {
        Url::parse(&format!("{}{}", Resource::ERGAST_BASE_URL, tail)).unwrap()
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
            url("/drivers.json?limit=1000&offset=0")
        );
    }

    #[test]
    #[should_panic]
    fn resource_to_url_round_without_season_filter_panics() {
        Resource::RaceSchedule(Filters {
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
    fn filters() {
        let filters = Filters::none();
        assert!(
            filters.season.is_none()
                && filters.round.is_none()
                && filters.driver_id.is_none()
                && filters.constructor_id.is_none()
                && filters.circuit_id.is_none()
                && filters.qualifying_pos.is_none()
                && filters.grid_pos.is_none()
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

        assert!(
            filters.season.is_none()
                && filters.round.is_none()
                && filters.constructor_id.is_none()
                && filters.qualifying_pos.is_none()
                && filters.grid_pos.is_none()
                && filters.finish_pos.is_none()
                && filters.fastest_lap_rank.is_none()
                && filters.finishing_status.is_none()
        );

        let filters = Filters::new().driver_id("alonso".into()).circuit_id("spa".into());
        assert_eq!(filters.driver_id, Some("alonso".into()));
        assert_eq!(filters.circuit_id, Some("spa".into()));

        assert!(
            filters.season.is_none()
                && filters.round.is_none()
                && filters.constructor_id.is_none()
                && filters.qualifying_pos.is_none()
                && filters.grid_pos.is_none()
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
        assert!(filters.lap.is_none() && filters.driver_id.is_none());

        let filters = LapTimeFilters {
            lap: Some(1),
            ..LapTimeFilters::new(2023, 4)
        };
        assert_eq!(filters.season, 2023);
        assert_eq!(filters.round, 4);
        assert_eq!(filters.lap, Some(1));
        assert!(filters.driver_id.is_none());

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
        assert!(filters.lap.is_none() && filters.driver_id.is_none() && filters.pit_stop.is_none());

        let filters = PitStopFilters {
            lap: Some(1),
            ..PitStopFilters::new(2023, 4)
        };
        assert_eq!(filters.season, 2023);
        assert_eq!(filters.round, 4);
        assert_eq!(filters.lap, Some(1));
        assert!(filters.driver_id.is_none() && filters.pit_stop.is_none());

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

        assert!(!formatted_pairs.is_empty());
        assert_eq!(formatted_pairs[0].0, "");
    }

    #[test]
    fn page_construction() {
        assert_eq!(Page::with(20, 5), Page { limit: 20, offset: 5 });
        assert_eq!(Page::with_limit(20), Page { limit: 20, offset: 0 });

        assert_eq!(
            Page::with_offset(5),
            Page {
                limit: Page::DEFAULT_LIMIT,
                offset: 5
            }
        );

        assert_eq!(
            Page::with_max_limit(),
            Page {
                limit: Page::MAX_LIMIT,
                offset: Page::DEFAULT_OFFSET
            }
        );

        assert_eq!(
            Page::default(),
            Page {
                limit: Page::DEFAULT_LIMIT,
                offset: Page::DEFAULT_OFFSET
            }
        );
    }

    #[test]
    #[should_panic]
    fn page_construction_panics() {
        Page::with_limit(2000);
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
