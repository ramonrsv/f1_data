/// Each variant of the [`Resource`] enumeration represents a given resource that can be requested
/// from the Ergast API, and it contains any options or filters that can be applied to the request.
// @todo Add examples once the `get_*` API has been settled
#[derive(Debug)]
pub enum Resource {
    /// Get a list of seasons currently supported by the API. Each season listed in a response is
    /// uniquely identified by the year it took place in, returned in
    /// [`response::Season::season`](crate::ergast::response::Season::season), e.g. `"2023"` for the
    /// _2023 Formula One World Championship_. The season year can be used to filter requests for
    /// other resources, via [`Filters::year`].
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
    /// used to filter requests for other resources, via [`Filters::year`] and [`Filters::round`],
    /// respectively.
    ///
    /// **Note:**: Round indexes may not be contiguous, if there are scheduled race cancellations
    /// due to weather, pandemics, etc.
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
    /// [`response::Result::grid`](crate::ergast::response::Result::grid) for sprints and races,
    /// respectively.
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/qualifying/>
    QualifyingResults(Filters),
    /// Get a list of sprint event results.
    ///
    /// **Note:** Sprint results are only available for races where there is a `Sprint` element in
    /// the schedule, returned in [`response::Race::sprint`](crate::ergast::response::Race::sprint).
    ///
    /// Directly maps to <https://ergast.com/mrd/methods/sprint/>
    SprintResults(Filters),
    /// Get a list of race results. Various of the returned values can be used to filter requests
    /// for other resources, via fields of [`Filters`]. The grid position, returned in
    /// [`response::Result::grid`](crate::ergast::response::Result::grid), can be used in
    /// [`Filters::grid_pos`]. A grid position of `0`, or [`Filters::GRID_PIT_LANE`], indicates that
    /// a driver started from the pit lane. The finishing position, returned in
    /// [`response::Result::position`](crate::ergast::response::Result::position), can be used in
    /// [`Filters::finish_pos`].
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/results/>
    RaceResults(Filters),
    /// Get a list of finishing status codes supported by the API. While each status has a textual
    /// representation, e.g. `"Finished"`, `"Accident"`, `"Collision"`, etc., it is uniquely
    /// identified by a numeric ID, returned in
    /// [`response::Status::status_id`](crate::ergast::response::Status::status_id). This unique ID
    /// can be used to filter requests for other resources, via [`Filters::finishing_status`].
    ///
    /// Directly maps to <http://ergast.com/mrd/methods/status/>
    FinishingStatus(Filters),
    // These resources are not yet supported.
    #[doc(hidden)]
    LapTimes,
    #[doc(hidden)]
    PitStops,
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
    /// use f1_data::ergast::resource::{Resource, Filters};
    ///
    /// let request = Resource::DriverInfo(Filters {
    ///     driver_id: Some("leclerc".to_string()),
    ///     ..Filters::none()
    /// });
    ///
    /// assert_eq!(
    ///     request.to_url(),
    ///     "http://ergast.com/api/f1/drivers/leclerc.json".to_string()
    /// );
    /// ```
    pub fn to_url(&self) -> String {
        let (resource_key, filters) = match self {
            Self::SeasonList(f) => ("/seasons", f),
            Self::DriverInfo(f) => ("/drivers", f),
            Self::ConstructorInfo(f) => ("/constructors", f),
            Self::CircuitInfo(f) => ("/circuits", f),
            Self::RaceSchedule(f) => ("/races", f),
            Self::QualifyingResults(f) => ("/qualifying", f),
            Self::SprintResults(f) => ("/sprint", f),
            Self::RaceResults(f) => ("/results", f),
            Self::FinishingStatus(f) => ("/status", f),
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

        format!(
            "{}{}.json",
            Resource::ERGAST_BASE_URL,
            filters
                .iter()
                .filter(|(key, val)| !val.is_empty() || key == &resource_key)
                .map(|(key, val)| format!("{}{}", key, val))
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

/// Can be used to filter a given [`Resource`] for the Ergast API by a number of fields, all of
/// which are optional and can be set simultaneously. [`Filters::none`] sets all fields to `None`,
/// i.e. no filtering is performed.
///
/// # Examples
///
/// ```
/// use f1_data::ergast::resource::Filters;
///
/// let filters = Filters {
///     driver_id: Some("alonso".to_string()),
///     circuit_id: Some("spa".to_string()),
///     ..Filters::none()
/// };
/// assert_eq!(filters.driver_id, Some("alonso".to_string()));
/// assert_eq!(filters.circuit_id, Some("spa".to_string()));
///
/// assert!(
///     filters.year.is_none()
///         && filters.round.is_none()
///         && filters.constructor_id.is_none()
///         && filters.qualifying_pos.is_none()
///         /* ... */
/// );
/// ```
#[derive(Debug)]
pub struct Filters {
    /// Restrict responses to a given championship season, identified by the year it took place in,
    /// e.g. `2023` for the _2023 Formula One World Championship_. See [`Resource::SeasonList`] to
    /// get a list of seasons currently supported by the API.
    pub year: Option<u32>,
    /// Restrict responses to a specific race, identified by the round index starting from `1`, in a
    /// specific season. See [`Resource::RaceSchedule`] to get a list of rounds for a given season.
    ///
    /// **Note:** A [`Filters::year`] is required if this field is set, in order to uniquely
    /// identify a race.
    ///
    /// **Note:** Round indexes may not be contiguous, if there are scheduled race cancellations
    /// due to weather, pandemics, etc.
    ///
    /// # Panics
    ///
    /// [`Filters::round`] being set requires that [`Filters::year`] be set as well, else
    /// certain methods may panic. The inverse is not true, [`Filters::year`] can be set
    /// without [`Filters::round`].
    pub round: Option<u32>,
    /// Restrict responses to those in which a given driver, identified by a unique ID, features,
    /// e.g. seasons or races in which the driver competed, constructors for which they drove, etc.
    /// See [`Resource::DriverInfo`] to get a list of all available driver IDs.
    pub driver_id: Option<String>,
    /// Restrict responses to those in which a given constructors, identified by a unique ID,
    /// features, e.g. seasons or races in which the constructor competed, drivers that drove for
    /// them, etc. See [`Resource::ConstructorInfo`] to get a list of all available constructor IDs.
    pub constructor_id: Option<String>,
    /// Restrict responses to those in which a given circuit, identified by a unique ID, features,
    /// e.g. races that took place in that circuit, seasons that held such a race, drivers that
    /// competed in that circuit, etc. See [`Resource::CircuitInfo`] to get a list of all available
    /// circuit IDs.
    pub circuit_id: Option<String>,
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
    pub finishing_status: Option<u32>,
}

impl Filters {
    /// Value that can be set in [`Filters::grid_pos`] field to indicate a driver that started
    /// the race from the pit lane.
    pub const GRID_PIT_LANE: u32 = crate::ergast::response::GRID_PIT_LANE;

    /// Returns a [`Filters`] object with all fields set to `None`, i.e. requesting no filtering.
    ///
    /// # Examples
    ///
    /// ```
    /// use f1_data::ergast::resource::Filters;
    ///
    /// let filters = Filters::none();
    /// assert!(
    ///     filters.year.is_none()
    ///         && filters.round.is_none()
    ///         && filters.constructor_id.is_none()
    ///         && filters.qualifying_pos.is_none()
    ///         /* ... */
    /// );
    /// ```
    pub fn none() -> Filters {
        Filters {
            year: None,
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

    /// Format a generic `Option<T>`; None as "", and Some(val) as "/{val}"
    fn fmt_from_opt<T: std::fmt::Display>(field: &Option<T>) -> String {
        if let Some(val) = field {
            format!("/{val}")
        } else {
            String::new()
        }
    }

    /// Return a list of (<resource_key>, <formatted_value>) for all possible filters
    fn to_formatted_pairs(&self) -> Vec<(&str, String)> {
        Vec::from([
            ("", Self::fmt_from_opt(&self.year)),
            ("", Self::fmt_from_opt(&self.round)),
            ("/drivers", Self::fmt_from_opt(&self.driver_id)),
            ("/constructors", Self::fmt_from_opt(&self.constructor_id)),
            ("/circuits", Self::fmt_from_opt(&self.circuit_id)),
            ("/qualifying", Self::fmt_from_opt(&self.qualifying_pos)),
            ("/grid", Self::fmt_from_opt(&self.grid_pos)),
            ("/results", Self::fmt_from_opt(&self.finish_pos)),
            ("/fastest", Self::fmt_from_opt(&self.fastest_lap_rank)),
            ("/status", Self::fmt_from_opt(&self.finishing_status)),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_ergast_base_url() {
        assert_eq!(Resource::ERGAST_BASE_URL, "http://ergast.com/api/f1")
    }

    fn url(tail: &str) -> String {
        format!("{}{}", Resource::ERGAST_BASE_URL, tail)
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
                driver_id: Some("leclerc".to_string()),
                ..Filters::none()
            })
            .to_url(),
            url("/drivers/leclerc.json")
        );
    }

    #[test]
    fn resource_to_url_non_resource_filters() {
        assert_eq!(
            Resource::SeasonList(Filters {
                driver_id: Some("leclerc".to_string()),
                ..Filters::none()
            })
            .to_url(),
            url("/drivers/leclerc/seasons.json")
        );
    }

    #[test]
    fn resource_to_url_mixed_filters() {
        assert_eq!(
            Resource::DriverInfo(Filters {
                driver_id: Some("leclerc".to_string()),
                constructor_id: Some("ferrari".to_string()),
                ..Filters::none()
            })
            .to_url(),
            url("/constructors/ferrari/drivers/leclerc.json")
        );
    }

    #[test]
    fn filters_fields() {
        let filters = Filters::none();
        assert!(
            filters.year.is_none()
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
            driver_id: Some("alonso".to_string()),
            circuit_id: Some("spa".to_string()),
            ..Filters::none()
        };
        assert_eq!(filters.driver_id, Some("alonso".to_string()));
        assert_eq!(filters.circuit_id, Some("spa".to_string()));

        assert!(
            filters.year.is_none()
                && filters.round.is_none()
                && filters.constructor_id.is_none()
                && filters.qualifying_pos.is_none()
                && filters.grid_pos.is_none()
                && filters.finish_pos.is_none()
                && filters.fastest_lap_rank.is_none()
                && filters.finishing_status.is_none()
        );
    }
}
