//! Defines types that represent and can be used to deserialize JSON responses from the
//! [jolpica-f1](https://github.com/jolpica/jolpica-f1) API.
//!
//! [`Response`] represents a full API response, supporting variants corresponding to the possible
//! endpoints and returned [structures from the API](https://api.jolpi.ca/ergast/). There are also
//! a variety of convenience methods for extracting specific data and performing certain validation,
//! as well as more ergonomic types that can be returned when requesting specific resources.

use std::convert::Infallible;

use enum_as_inner::EnumAsInner;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_with::{DisplayFromStr, serde_as};
use url::Url;

use crate::{
    error::{Error, Result},
    id::{CircuitID, ConstructorID, DriverID, RoundID, SeasonID, StatusID},
    jolpica::time::{
        Date, DateTime, Duration, QualifyingTime, RaceTime, Time, deserialize_buggy_race_time, deserialize_duration,
        deserialize_optional_time, deserialize_time,
    },
};

#[cfg(doc)]
use crate::jolpica::resource::{Filters, Resource};

/// Represents a full JSON response from the jolpica-f1 API.
///
/// It contains metadata about the API and the response, and a single [`Table`] of data holding a
/// request-dependent variant. Note that, while [`Response`] can be deserialized from a full JSON
/// response, it actually represents the underlying `"MRData"` object, which is flattened in this
/// struct to improve ergonomics.
#[derive(PartialEq, Clone, Debug)]
pub struct Response {
    /// XML namespace, unused in the new jolpica-f1 API.
    pub xmlns: String,
    /// Racing series, currently always `"f1"`.
    pub series: String,
    /// URL of the API endpoint that produced this response.
    pub url: Url,
    /// Pagination information for this response.
    pub pagination: Pagination,
    /// The main data table contained in this response.
    pub table: Table,
}

impl Response {
    /// Returns a tuple with references to all the fields of this [`Response`] except for the
    /// [`pagination`](Self::pagination) and [`table`](Self::table) fields, to allow comparing the
    /// [`Response`]s' metadata for equality while ignoring pagination and table data.
    //
    // @todo If a new field is added to [`Response`], and this impl isn't updated accordingly, then
    // comparisons will silently fail - unit tests won't catch it. I haven't figured out a way to
    // solve this without adding generic parameters, inefficient cloning and discarding, etc.
    pub const fn as_info(&self) -> (&String, &String, &Url) {
        (&self.xmlns, &self.series, &self.url)
    }

    /// Returns a tuple with all the fields of this [`Response`] except for the
    /// [`pagination`](Self::pagination) and [`table`](Self::table) fields, to allow comparing the
    /// [`Response`]s' metadata for equality while ignoring pagination and table data. This method
    /// is more inefficient than [`as_info()`](Self::as_info) as it clones all of the fields.
    /// It should only be used when `as_info()`would be too inconvenient due to lifetime, etc.
    //
    // @todo See the comment in [`as_info()`](Self::as_info).
    pub fn to_info(&self) -> (String, String, Url) {
        (self.xmlns.clone(), self.series.clone(), self.url.clone())
    }

    // TableInnerLists
    // ---------------

    /// Extracts the inner list value from the corresponding [`Table`] variant for this
    /// [`TableInnerList`].
    ///
    /// For example, [`Response::into_table_list::<Season>()`] extracts from [`Response::table`]
    /// the inner [`Vec<Season>`] of the [`Table::Seasons`] variant.
    ///
    /// Convenience aliases are provided for all implemented [`TableInnerList`] types, e.g.
    /// [`Response::into_seasons()`] is an alias for [`Response::into_table_list::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableInnerList`] type `T`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}};
    /// # use f1_data::jolpica::response::Season;
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica
    ///     .get_response(&Resource::SeasonList(Filters::none()))
    ///     .unwrap();
    ///
    /// let seasons = resp.into_table_list::<Season>().unwrap();
    ///
    /// assert!(seasons.len() >= 74);
    /// assert_eq!(seasons[0].season, 1950);
    /// assert_eq!(seasons[73].season, 2023);
    /// ```
    pub fn into_table_list<T: TableInnerList>(self) -> Result<Vec<T>> {
        T::try_into_inner_from(self.table)
    }

    /// Extracts an expected single element from the inner list for the corresponding [`Table`]
    /// variant for this [`TableInnerList`].
    ///
    /// This method is similar to [`Response::into_table_list::<T>()`], but verifies that one and
    /// only one element is present in the extracted list, returning that element directly. For
    /// example, [`Response::into_single_table_list_element::<Season>()`] extracts from
    /// [`Response::table`] the inner [`Vec<Season>`] of the [`Table::Seasons`] variant, verifies
    /// that it contains only one element, then extracts and returns that single [`Season`].
    ///
    /// Convenience aliases are provided for all implemented [`TableInnerList`] types, e.g.
    /// [`Response::into_season()`] is an alias for
    /// [`Response::into_single_table_list_element::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableInnerList`] type `T`. Returns an [`Error::NotFound`] if the extracted list
    /// is empty, or an [`Error::TooMany`] if it contains more than one element.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}};
    /// # use f1_data::jolpica::response::Season;
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica
    ///     .get_response(&Resource::SeasonList(Filters::new().season(2023)))
    ///     .unwrap();
    ///
    /// let season = resp.into_single_table_list_element::<Season>().unwrap();
    ///
    /// assert_eq!(season.season, 2023);
    /// assert_eq!(
    ///     season.url.as_str(),
    ///     "https://en.wikipedia.org/wiki/2023_Formula_One_World_Championship"
    /// );
    /// ```
    pub fn into_single_table_list_element<T: TableInnerList>(self) -> Result<T> {
        self.into_table_list().and_then(verify_has_one_element_and_extract)
    }

    /// Gets a reference to the inner list value from the corresponding [`Table`] variant for this
    /// [`TableInnerList`].
    ///
    /// This method is similar to [`Response::into_table_list::<T>()`], but it returns a reference
    /// and does not consume the [`Response`]. For example, [`Response::as_table_list::<Season>()`]
    /// gets a reference to the inner [`Vec<Season>`] of the [`Table::Seasons`] variant in
    /// [`Response::table`].
    ///
    /// Convenience aliases are provided for all implemented [`TableInnerList`] types, e.g.
    /// [`Response::as_season()`] is an alias for [`Response::as_table_list::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableInnerList`] type `T`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}};
    /// # use f1_data::jolpica::response::Season;
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica
    ///     .get_response(&Resource::SeasonList(Filters::none()))
    ///     .unwrap();
    ///
    /// let seasons = resp.as_table_list::<Season>().unwrap();
    ///
    /// assert!(seasons.len() >= 74);
    /// assert_eq!(seasons[0].season, 1950);
    /// assert_eq!(seasons[73].season, 2023);
    /// ```
    pub fn as_table_list<T: TableInnerList>(&self) -> Result<&Vec<T>> {
        T::try_as_inner_from(&self.table)
    }

    /// Gets a reference to an expected single element from the inner list for the corresponding
    /// [`Table`] variant for this [`TableInnerList`].
    ///
    /// This method is similar to [`Response::as_table_list::<T>()`], but verifies that one and
    /// only one element is present in the list, returning a reference to that element directly.
    /// It's also similar to [`Response::into_single_table_list_element::<T>()`], but it returns a
    /// reference instead of consuming the [`Response`]. For example,
    /// [`Response::as_single_table_list_element::<Season>()`] gets a reference to the
    /// [`Vec<Season>`] from the [`Table::Seasons`] variant in [`Response::table`], verifies that it
    /// contains only one element, then returns a reference to that single [`Season`].
    ///
    /// Convenience aliases are provided for all implemented [`TableInnerList`] types, e.g.
    /// [`Response::as_season()`] is an alias for
    /// [`Response::as_single_table_list_element::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableInnerList`] type `T`. Returns an [`Error::NotFound`] if the extracted list
    /// is empty, or an [`Error::TooMany`] if it contains more than one element.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}};
    /// # use f1_data::jolpica::response::Season;
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica
    ///     .get_response(&Resource::SeasonList(Filters::new().season(2023)))
    ///     .unwrap();
    ///
    /// let season = resp.as_single_table_list_element::<Season>().unwrap();
    ///
    /// assert_eq!(season.season, 2023);
    /// assert_eq!(
    ///     season.url.as_str(),
    ///     "https://en.wikipedia.org/wiki/2023_Formula_One_World_Championship"
    /// );
    /// ```
    pub fn as_single_table_list_element<T: TableInnerList>(&self) -> Result<&T> {
        self.as_table_list()
            .map(Vec::as_slice)
            .and_then(verify_has_one_element)
            .map(|s| s.first().unwrap())
    }

    // Races and SessionResults
    // ------------------------

    /// Extracts the inner list of [`Race<Payload>`]s from the [`Table::Races`] variant and maps
    /// them to [`Race<Schedule>`]s.
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant is not
    /// [`Table::Races`], and an [`Error::BadPayloadVariant`] if the contained [`Payload`] variant
    /// is not [`Payload::Schedule`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}, time::macros::date};
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica
    ///     .get_response(&Resource::RaceSchedule(Filters::new().season(2024)))
    ///     .unwrap();
    ///
    /// let races = resp.into_race_schedules().unwrap();
    ///
    /// assert_eq!(races.len(), 24);
    /// assert_eq!(races[0].season, 2024);
    /// assert_eq!(races.first().unwrap().round, 1);
    /// assert_eq!(races.last().unwrap().round, 24);
    ///
    /// let sprint_count = races.iter().filter(|race| race.schedule().sprint.is_some()).count();
    /// assert_eq!(sprint_count, 6);
    ///
    /// assert_eq!(races[0].circuit.circuit_name, "Bahrain International Circuit");
    /// assert_eq!(races[0].date, date!(2024 - 3 - 2));
    /// assert_eq!(races[0].schedule().qualifying.unwrap().date, date!(2024 - 3 - 1));
    /// ```
    pub fn into_race_schedules(self) -> Result<Vec<Race<Schedule>>> {
        self.into_races()?
            .into_iter()
            .map(|race| race.try_map(|payload| payload.into_schedule().map_err(into)))
            .collect()
    }

    /// Extracts an expected single element from the inner list of [`Race<Payload>`]s from the
    /// [`Table::Races`] variant and maps it to [`Race<Schedule>`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant is not
    /// [`Table::Races`], or an [`Error::BadPayloadVariant`] if the contained [`Payload`] variant is
    /// not [`Payload::Schedule`]. Returns an [`Error::NotFound`] if the extracted list is empty, or
    /// an [`Error::TooMany`] if it contains more than one element.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}, time::macros::date};
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica
    ///     .get_response(&Resource::RaceSchedule(Filters::new().season(2024).round(1)))
    ///     .unwrap();
    ///
    /// let race = resp.into_race_schedule().unwrap();
    ///
    /// assert_eq!(race.season, 2024);
    /// assert_eq!(race.round, 1);
    ///
    /// assert_eq!(race.circuit.circuit_name, "Bahrain International Circuit");
    /// assert_eq!(race.date, date!(2024 - 3 - 2));
    /// assert_eq!(race.schedule().qualifying.unwrap().date, date!(2024 - 3 - 1));
    /// ```
    pub fn into_race_schedule(self) -> Result<Race<Schedule>> {
        self.into_race_schedules().and_then(verify_has_one_element_and_extract)
    }

    /// Extracts the inner list of [`Race<Payload>`]s from the [`Table::Races`] variant, each with
    /// an inner list of <code>T = [PayloadInnerList]</code>s, and maps them to [`Race<Vec<T>>`]s.
    ///
    /// For example,
    /// [`into_many_races_with_many_session_results::<RaceResult>()`](Self::into_many_races_with_many_session_results)
    /// will return a sequence of [`Race<Vec<RaceResult>>`]s, where the [`Payload`] variant
    /// [`Payload::RaceResults`] has already been extracted and processed into
    /// [`Race<Vec<RaceResult>>`], obviating the need to perform error checking and extraction of
    /// the expected variants.
    ///
    /// This function returns a sequence of <code>T = [PayloadInnerList]</code>s for each of a
    /// sequence of [`Race`]s, i.e. it returns [`Vec<Race<Vec<T>>>`]. If a single [`Race`] is
    /// expected in the response, or a single `T` per [`Race`], or other, consider using one of
    /// the other methods with the desired processing:
    /// [`into_one_race_with_many_session_results`][Self::into_one_race_with_many_session_results],
    /// [`into_many_races_with_one_session_result`][Self::into_many_races_with_one_session_result],
    /// or [`into_one_race_with_one_session_result`][Self::into_one_race_with_one_session_result].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant is not
    /// [`Table::Races`], or an [`Error::BadPayloadVariant`] if the contained [`Payload`] variant is
    /// not the variant corresponding to the <code>T = [PayloadInnerList]</code>, e.g. if it's not
    /// [`Payload::RaceResults`] when <code>T = [RaceResult]</code>.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::{
    /// #     id::ConstructorID,
    /// #     jolpica::{
    /// #         agent::Agent,
    /// #         resource::{Filters, Resource},
    /// #         response::{Points, RaceResult, SprintResult},
    /// #     },
    /// # };
    /// # let jolpica = Agent::default();
    /// #
    /// let red_bull_2021_filter = Filters::new()
    ///     .season(2021).constructor_id(ConstructorID::from("red_bull"));
    ///
    /// let resp_race_results = jolpica.get_response(
    ///     &Resource::RaceResults(red_bull_2021_filter.clone())).unwrap();
    /// let resp_sprint_results = jolpica.get_response(
    ///     &Resource::SprintResults(red_bull_2021_filter.clone())).unwrap();
    ///
    /// let race_points = resp_race_results
    ///     .into_many_races_with_many_session_results::<RaceResult>()
    ///     .unwrap()
    ///     .iter()
    ///     .map(|r| r.race_results().iter().map(|r| r.points).sum::<Points>())
    ///     .sum::<Points>();
    ///
    /// let sprint_points = resp_sprint_results
    ///     .into_many_races_with_many_session_results::<SprintResult>()
    ///     .unwrap()
    ///     .iter()
    ///     .map(|s| s.sprint_results().iter().map(|r| r.points).sum::<Points>())
    ///     .sum::<Points>();
    ///
    /// assert_eq!(race_points + sprint_points, 585.5);
    /// ```
    pub fn into_many_races_with_many_session_results<T: PayloadInnerList>(self) -> Result<Vec<Race<Vec<T>>>> {
        self.into_races()?
            .into_iter()
            .map(|race| race.try_map(|payload| T::try_into_inner_from(payload)))
            .collect()
    }

    /// Extracts the single expected [`Race<Payload>`] from the [`Table::Races`] variant, with
    /// an inner list of <code>T = [PayloadInnerList]</code>s, and maps it to a [`Race<Vec<T>>`].
    ///
    /// For example,
    /// [`into_one_race_with_many_session_results::<RaceResult>`][Self::into_one_race_with_many_session_results]
    /// will return a single [`Race<Vec<RaceResult>>`], where the [`Payload`] variant
    /// [`Payload::RaceResults`] has already been extracted and processed into a single
    /// [`Race<Vec<RaceResult>>`], obviating the need to perform error checking and extraction of
    /// the expected variants.
    ///
    /// This function returns a single [`Race`] containing a sequence of
    /// <code>T = [PayloadInnerList]</code>s, i.e. it returns a [`Race<Vec<T>>`]. If multiple
    /// [`Race`]s are expected in the response, or a single `T` per [`Race`], or other, consider
    /// using one of the other methods with the desired processing:
    /// [`into_many_races_with_many_session_results`][Self::into_many_races_with_many_session_results],
    /// [`into_many_races_with_one_session_result`][Self::into_many_races_with_one_session_result],
    /// or [`into_one_race_with_one_session_result`][Self::into_one_race_with_one_session_result].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant is not
    /// [`Table::Races`], or an [`Error::BadPayloadVariant`] if the contained [`Payload`] variant is
    /// not the variant corresponding to the <code>T = [PayloadInnerList]</code>, e.g. if it's not
    /// [`Payload::RaceResults`] when <code>T = [RaceResult]</code>. An [`Error::NotFound`] or
    /// [`Error::TooMany`] is returned if the expected number of [`Race`]s and
    /// <code>T = [PayloadInnerList]</code>s per [`Race`] are not found in the response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}, response::RaceResult};
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica.get_response(&Resource::RaceResults(
    ///     Filters::new().season(2021).round(22))).unwrap();
    ///
    /// let race = resp.into_one_race_with_many_session_results::<RaceResult>().unwrap();
    ///
    /// assert_eq!(race.race_name, "Abu Dhabi Grand Prix");
    /// assert_eq!(race.race_results()[0].driver.family_name, "Verstappen");
    /// assert_eq!(race.race_results()[0].position, 1);
    /// assert_eq!(race.race_results()[1].driver.family_name, "Hamilton");
    /// assert_eq!(race.race_results()[1].position, 2);
    /// ```
    pub fn into_one_race_with_many_session_results<T: PayloadInnerList>(self) -> Result<Race<Vec<T>>> {
        self.into_many_races_with_many_session_results::<T>()
            .and_then(verify_has_one_element_and_extract)
    }

    /// Extracts the inner list of [`Race<Payload>`]s from the [`Table::Races`] variant, each with
    /// with a single expected <code>T = [PayloadInnerList]</code>, and maps them to [`Race<T>`]s.
    ///
    /// For example,
    /// [`into_many_races_with_one_session_result::<RaceResult>`][Self::into_many_races_with_one_session_result]
    /// will return a sequence of [`Race<RaceResult>`], where the [`Payload`] variant
    /// [`Payload::RaceResults`] has already been extracted and processed into [`Race<RaceResult>`],
    /// ensuring that each [`Race`] holds one and only one <code>T = [PayloadInnerList]</code>,
    /// obviating the need to perform error checking and extraction of the expected variants.
    ///
    /// This function returns a sequence of [`Race`]s containing a single
    /// <code>T = [PayloadInnerList]</code> each, i.e. it returns [`Vec<Race<T>>`]. If a single
    /// [`Race`] is expected in the response, or multiple `T`s per [`Race`], or other, consider
    /// using one of the other methods with the desired processing:
    /// [`into_many_races_with_many_session_results`][Self::into_many_races_with_many_session_results],
    /// [`into_one_race_with_many_session_results`][Self::into_one_race_with_many_session_results],
    /// or [`into_one_race_with_one_session_result`][Self::into_one_race_with_one_session_result].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant is not
    /// [`Table::Races`], or an [`Error::BadPayloadVariant`] if the contained [`Payload`] variant is
    /// not the variant corresponding to the <code>T = [PayloadInnerList]</code>, e.g. if it's not
    /// [`Payload::RaceResults`] when <code>T = [RaceResult]</code>. An [`Error::NotFound`] or
    /// [`Error::TooMany`] is returned if the expected number of [`Race`]s and
    /// <code>T = [PayloadInnerList]</code>s per [`Race`] are not found in the response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::id::DriverID;
    /// # use f1_data::jolpica::{
    /// #     agent::Agent,
    /// #     resource::{Filters, Resource},
    /// #     response::QualifyingResult
    /// # };
    /// # let jolpica = Agent::default();
    /// #
    /// # // @todo Replace .map(...) with .qualifying_pos(1) when that filter is fixed in the API.
    /// let resp = jolpica.get_response(&Resource::QualifyingResults(
    ///     Filters::new().driver_id(DriverID::from("vettel"))
    /// )).unwrap();
    ///
    /// let seb_poles: u32 = resp.into_many_races_with_one_session_result::<QualifyingResult>()
    ///     .unwrap()
    ///     .iter()
    ///     .map(|race| {
    ///         if race.qualifying_result().position == 1 { 1 } else { 0 }
    ///     })
    ///     .sum();
    ///
    /// assert_eq!(seb_poles, 57);
    /// ```
    pub fn into_many_races_with_one_session_result<T: PayloadInnerList>(self) -> Result<Vec<Race<T>>> {
        self.into_many_races_with_many_session_results::<T>()?
            .into_iter()
            .map(|race| race.try_map(verify_has_one_element_and_extract))
            .collect()
    }

    /// Extracts the single expected [`Race<Payload>`] from the [`Table::Races`] variant, with
    /// a single expected <code>T = [PayloadInnerList]</code>, and maps it to a [`Race<T>`].
    ///
    /// For example,
    /// [`into_one_race_with_one_session_result::<RaceResult>`][Self::into_one_race_with_one_session_result]
    /// will return a single [`Race<RaceResult>`], where the [`Payload`] variant
    /// [`Payload::RaceResults`] has already been extracted and processed into [`Race<RaceResult>`],
    /// ensuring that one and only one [`Race`] is found, holding one and only one
    /// <code>T = [PayloadInnerList]</code>, obviating the need to perform error checking and
    /// extraction of the expected variants.
    ///
    /// This function returns a single [`Race`]s containing a single
    /// <code>T = [PayloadInnerList]</code> , i.e. it returns [`Race<T>`]. If multiple [`Race`]s or
    /// `T`s are expected in the response, consider using one of the other methods with the
    /// desired processing:
    /// [`into_many_races_with_many_session_results`][Self::into_many_races_with_many_session_results],
    /// [`into_one_race_with_many_session_results`][Self::into_one_race_with_many_session_results], or
    /// [`into_many_races_with_one_session_result`][Self::into_many_races_with_one_session_result].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant is not
    /// [`Table::Races`], or an [`Error::BadPayloadVariant`] if the contained [`Payload`] variant is
    /// not the variant corresponding to the <code>T = [PayloadInnerList]</code>, e.g. if it's not
    /// [`Payload::RaceResults`] when <code>T = [RaceResult]</code>. An [`Error::NotFound`] or
    /// [`Error::TooMany`] is returned if the expected number of [`Race`]s and
    /// <code>T = [PayloadInnerList]</code>s per [`Race`] are not found in the response.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use f1_data::jolpica::{agent::Agent, resource::{Filters, Resource}, response::SprintResult};
    /// # let jolpica = Agent::default();
    /// #
    /// let resp = jolpica.get_response(&Resource::SprintResults(
    ///     Filters::new().season(2021).round(10).sprint_pos(1)
    /// )).unwrap();
    ///
    /// let race = resp.into_one_race_with_one_session_result::<SprintResult>().unwrap();
    ///
    /// assert_eq!(race.sprint_result().position, 1);
    /// assert_eq!(race.sprint_result().driver.family_name, "Verstappen");
    /// ```
    pub fn into_one_race_with_one_session_result<T: PayloadInnerList>(self) -> Result<Race<T>> {
        self.into_many_races_with_one_session_result::<T>()
            .and_then(verify_has_one_element_and_extract)
    }

    // Laps, Timings, PitStops
    // -----------------------

    pub fn into_driver_laps(self, driver_id: &DriverID) -> Result<Vec<DriverLap>> {
        Ok(self)
            .and_then(verify_has_one_race_and_extract)?
            .payload
            .into_laps()?
            .into_iter()
            .map(|lap| DriverLap::try_from(lap, driver_id))
            .collect()
    }

    pub fn into_lap_timings(self) -> Result<Vec<Timing>> {
        Ok(self)
            .and_then(verify_has_one_race_and_extract)?
            .payload
            .into_laps()
            .map_err(into)
            .and_then(verify_has_one_element_and_extract)
            .map(|lap| lap.timings)
    }

    pub fn into_pit_stops(self) -> Result<Vec<PitStop>> {
        Ok(self)
            .and_then(verify_has_one_race_and_extract)?
            .payload
            .into_pit_stops()
            .map_err(into)
    }

    // Convenience aliases for into/as_table_list(s)::<T> and into/as_single_table_list_element::<T>
    // Aliases for TableInnerList's: Season, Driver, Constructor, Circuit, Status, Race<Payload>
    // ---------------------------------------------------------------------------------------------

    /// Alias for [`into_table_list::<Season>()`](Self::into_table_list).
    pub fn into_seasons(self) -> Result<Vec<Season>> {
        self.into_table_list::<Season>()
    }

    /// Alias for
    /// [`into_single_table_list_element::<Season>()`](Self::into_single_table_list_element).
    pub fn into_season(self) -> Result<Season> {
        self.into_single_table_list_element::<Season>()
    }

    /// Alias for [`as_table_list::<Season>()`](Self::as_table_list).
    pub fn as_seasons(&self) -> Result<&Vec<Season>> {
        self.as_table_list::<Season>()
    }

    /// Alias for [`as_single_table_list_element::<Season>()`](Self::as_single_table_list_element).
    pub fn as_season(&self) -> Result<&Season> {
        self.as_single_table_list_element::<Season>()
    }

    /// Alias for [`into_table_list::<Driver>()`](Self::into_table_list).
    pub fn into_drivers(self) -> Result<Vec<Driver>> {
        self.into_table_list::<Driver>()
    }

    /// Alias for
    /// [`into_single_table_list_element::<Driver>()`](Self::into_single_table_list_element).
    pub fn into_driver(self) -> Result<Driver> {
        self.into_single_table_list_element::<Driver>()
    }

    /// Alias for [`as_table_list::<Driver>()`](Self::as_table_list).
    pub fn as_drivers(&self) -> Result<&Vec<Driver>> {
        self.as_table_list::<Driver>()
    }

    /// Alias for [`as_single_table_list_element::<Driver>()`](Self::as_single_table_list_element).
    pub fn as_driver(&self) -> Result<&Driver> {
        self.as_single_table_list_element::<Driver>()
    }

    /// Alias for [`into_table_list::<Constructor>()`](Self::into_table_list).
    pub fn into_constructors(self) -> Result<Vec<Constructor>> {
        self.into_table_list::<Constructor>()
    }

    /// Alias for
    /// [`into_single_table_list_element::<Constructor>()`](Self::into_single_table_list_element).
    pub fn into_constructor(self) -> Result<Constructor> {
        self.into_single_table_list_element::<Constructor>()
    }

    /// Alias for [`as_table_list::<Constructor>()`](Self::as_table_list).
    pub fn as_constructors(&self) -> Result<&Vec<Constructor>> {
        self.as_table_list::<Constructor>()
    }

    /// Alias for
    /// [`as_single_table_list_element::<Constructor>()`](Self::as_single_table_list_element).
    pub fn as_constructor(&self) -> Result<&Constructor> {
        self.as_single_table_list_element::<Constructor>()
    }

    /// Alias for [`as_table_list::<Circuit>()`](Self::as_table_list).
    pub fn into_circuits(self) -> Result<Vec<Circuit>> {
        self.into_table_list::<Circuit>()
    }

    /// Alias for
    /// [`into_single_table_list_element::<Circuit>()`](Self::into_single_table_list_element).
    pub fn into_circuit(self) -> Result<Circuit> {
        self.into_single_table_list_element::<Circuit>()
    }

    /// Alias for [`as_table_list::<Circuit>()`](Self::as_table_list).
    pub fn as_circuits(&self) -> Result<&Vec<Circuit>> {
        self.as_table_list::<Circuit>()
    }

    /// Alias for
    /// [`as_single_table_list_element::<Circuit>()`](Self::as_single_table_list_element).
    pub fn as_circuit(&self) -> Result<&Circuit> {
        self.as_single_table_list_element::<Circuit>()
    }

    /// Alias for [`as_table_list::<Status>()`](Self::as_table_list).
    pub fn into_statuses(self) -> Result<Vec<Status>> {
        self.into_table_list::<Status>()
    }

    /// Alias for
    /// [`into_single_table_list_element::<Status>()`](Self::into_single_table_list_element).
    pub fn into_status(self) -> Result<Status> {
        self.into_single_table_list_element::<Status>()
    }

    /// Alias for [`as_table_list::<Status>()`](Self::as_table_list).
    pub fn as_statuses(&self) -> Result<&Vec<Status>> {
        self.as_table_list::<Status>()
    }

    /// Alias for [`as_single_table_list_element::<Status>()`](Self::as_single_table_list_element).
    pub fn as_status(&self) -> Result<&Status> {
        self.as_single_table_list_element::<Status>()
    }

    /// Alias for [`into_table_list::<Race<Payload>>()`](Self::into_table_list).
    pub fn into_races(self) -> Result<Vec<Race<Payload>>> {
        self.into_table_list::<Race<Payload>>()
    }

    /// Alias for
    /// [`into_single_table_list_element::<Race<Payload>>()`](Self::into_single_table_list_element).
    pub fn into_race(self) -> Result<Race<Payload>> {
        self.into_single_table_list_element::<Race<Payload>>()
    }

    /// Alias for [`as_table_list::<Race<Payload>>()`](Self::as_table_list).
    pub fn as_races(&self) -> Result<&Vec<Race<Payload>>> {
        self.as_table_list::<Race<Payload>>()
    }

    /// Alias for
    /// [`as_single_table_list_element::<Race<Payload>>()`](Self::as_single_table_list_element).
    pub fn as_race(&self) -> Result<&Race<Payload>> {
        self.as_single_table_list_element::<Race<Payload>>()
    }
}

impl<'de> Deserialize<'de> for Response {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Proxy {
            #[serde(rename = "MRData")]
            mr_data: MrData,
        }

        #[derive(Deserialize)]
        struct MrData {
            xmlns: String,
            series: String,
            url: Url,
            #[serde(flatten)]
            pagination: Pagination,
            #[serde(flatten)]
            table: Table,
        }

        let mr_data = Proxy::deserialize(deserializer)?.mr_data;

        Ok(Self {
            xmlns: mr_data.xmlns,
            series: mr_data.series,
            url: mr_data.url,
            pagination: mr_data.pagination,
            table: mr_data.table,
        })
    }
}

/// Represents pagination information included in a jolpica-f1 API response.
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Pagination {
    /// Maximum number of results returned in a given page.
    #[serde_as(as = "DisplayFromStr")]
    pub limit: u32,
    /// Offset of the current page within the total results.
    #[serde_as(as = "DisplayFromStr")]
    pub offset: u32,
    /// Total number of results available across all pages.
    #[serde_as(as = "DisplayFromStr")]
    pub total: u32,
}

impl Pagination {
    /// Returns `true` if this is the last page of results, `false` otherwise.
    pub const fn is_last_page(&self) -> bool {
        (self.offset + self.limit) >= self.total
    }

    /// Returns `true` if this is a single-page response, `false` otherwise.
    pub const fn is_single_page(&self) -> bool {
        (self.offset == 0) && self.is_last_page()
    }

    /// Returns the [`Pagination`] for the next page of results, or `None` if this is the last page.
    pub const fn next_page(&self) -> Option<Self> {
        if self.is_last_page() {
            None
        } else {
            Some(Self {
                offset: self.offset + self.limit,
                ..*self
            })
        }
    }
}

/// Represents all the possible different lists of data that may be returned in a [`Response`] from
/// the jolpica-f1 API.
///
/// For example, [`Table::Seasons`] corresponds to the `"SeasonTable"` property key in the JSON
/// response, containing a list of [`Season`]s which corresponds to the `"Seasons"` property key.
/// One and only of these tables may be returned in a given response, depending on the requested
/// [`Resource`], which is represented by this enum's different variants.
///
/// The variants and inner fields may be matched and accessed via the usual pattern matching, or
/// via accessor functions provided by [`enum-as-inner`](https://crates.io/crates/enum-as-inner).
///
/// # Examples:
///
/// ```
/// # use url::Url;
/// # use f1_data::jolpica::response::{Season, Table};
/// #
/// let table = Table::Seasons {
///     seasons: vec![Season {
///         season: 2022,
///         url: Url::parse("http://empty.org").unwrap(),
///     }],
/// };
///
/// let Table::Seasons { ref seasons } = table else {
///     panic!("Expected Seasons variant")
/// };
/// assert_eq!(seasons[0].season, 2022);
///
/// assert_eq!(table.as_seasons().unwrap()[0].season, 2022);
/// ```
#[derive(Deserialize, EnumAsInner, PartialEq, Clone, Debug)]
pub enum Table {
    /// Contains a list of [`Season`]s, and corresponds to the `"SeasonTable"` property key in the
    /// JSON response from the jolpica-f1 API.
    #[serde(rename = "SeasonTable")]
    Seasons {
        /// List of [`Season`]s, corresponding to the `"Seasons"` property key in the JSON response.
        #[serde(rename = "Seasons")]
        seasons: Vec<Season>,
    },
    /// Contains a list of [`Driver`]s, and corresponds to the `"DriverTable"` property key in the
    /// JSON response from the jolpica-f1 API.
    #[serde(rename = "DriverTable")]
    Drivers {
        /// List of [`Driver`]s, corresponding to the `"Drivers"` property key in the JSON response.
        #[serde(rename = "Drivers")]
        drivers: Vec<Driver>,
    },
    /// Contains a list of [`Constructor`]s, and corresponds to the `"ConstructorTable"` property
    /// key in the JSON response from the jolpica-f1 API.
    #[serde(rename = "ConstructorTable")]
    Constructors {
        /// List of [`Constructor`]s, corresponding to the `"Constructors"` property key in the JSON
        /// response.
        #[serde(rename = "Constructors")]
        constructors: Vec<Constructor>,
    },
    /// Contains a list of [`Circuit`]s, and corresponds to the `"CircuitTable"` property key in the
    /// JSON response from the jolpica-f1 API.
    #[serde(rename = "CircuitTable")]
    Circuits {
        /// List of [`Circuit`]s, corresponding to the `"Circuits"` property key in the JSON
        /// response.
        #[serde(rename = "Circuits")]
        circuits: Vec<Circuit>,
    },
    /// Contains a list of [`Race`]s, and corresponds to the `"RaceTable"` property key in the
    /// JSON response from the jolpica-f1 API.
    #[serde(rename = "RaceTable")]
    Races {
        /// List of [`Race`]s, corresponding to the `"Races"` property key in the JSON response.
        #[serde(rename = "Races")]
        races: Vec<Race>,
    },
    /// Contains a list of [`Status`]es, and corresponds to the `"StatusTable"` property key in the
    /// JSON response from the jolpica-f1 API.
    #[serde(rename = "StatusTable")]
    Status {
        /// List of [`Status`]es, corresponding to the `"Status"` property key in the JSON response.
        #[serde(rename = "Status")]
        status: Vec<Status>,
    },
}

/// Inner list type of a [`Table`] variant for a [`TableInnerList`] type, and of a [`Payload`]
/// variant for a [`PayloadInnerList`] type. This is unlikely to change from [`Vec<T>`].
///
/// For example, the inner list type of the [`Table::Seasons`] variant is [`Vec<Season>`], and the
/// inner list type of the [`Payload::RaceResults`] variant is [`Vec<RaceResult>`].
type InnerList<T> = Vec<T>;

/// This trait allows for the generic extraction of the inner list types of all [`Table`] variants
///
/// For example, [`Season`]s can be extracted from a [`Response`]'s [`Response::table`], from the
/// [`Table::Seasons`] variant, via  [`T::try_into_inner_from()`](Self::try_into_inner_from).
///
/// The trait is implemented for [`Season`], [`Driver`], [`Constructor`], [`Circuit`], [`Status`],
/// and [`Race<Payload>`].
pub trait TableInnerList
where
    Self: Sized,
{
    /// Extract the inner value from the corresponding [`Table`] variant for this
    /// [`TableInnerList`], e.g. a [`Vec<Season>`] from the [`Table::Seasons`] variant for
    /// [`Season`].
    fn try_into_inner_from(table: Table) -> Result<InnerList<Self>>;

    /// Get a reference to the inner value from the corresponding [`Table`] variant for this
    /// [`TableInnerList`], e.g. a <code>&[`Vec<Season>`]</code> from the [`Table::Seasons`]
    /// variant.
    fn try_as_inner_from(table: &Table) -> Result<&InnerList<Self>>;
}

/// Holds information about a Formula 1 season.
///
/// Requested via [`Resource::SeasonList`] and returned in [`Table::Seasons`].
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Season {
    /// Unique identifier for the season, i.e. the year in which it took place, e.g. `2024` for the
    /// _2024 Formula One World Championship_.
    #[serde_as(as = "DisplayFromStr")]
    pub season: SeasonID,
    /// URL to the Wikipedia page for this season, e.g. for the `2024` season:
    /// [`"https://en.wikipedia.org/wiki/2024_Formula_One_World_Championship"`](https://en.wikipedia.org/wiki/2024_Formula_One_World_Championship)
    pub url: Url,
}

impl TableInnerList for Season {
    fn try_into_inner_from(table: Table) -> Result<InnerList<Self>> {
        table.into_seasons().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&InnerList<Self>> {
        table.as_seasons().ok_or(Error::BadTableVariant)
    }
}

/// Holds information about a Formula 1 driver.
///
/// Requested via [`Resource::DriverInfo`] and returned in [`Table::Drivers`].
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    /// Unique identifier for the driver, e.g. `"max_verstappen"` for _Max Verstappen_.
    pub driver_id: DriverID,
    /// Permanent number associated with the driver, if any, e.g. `33` for _Max Verstappen_.
    ///
    /// Permanent numbers were introduced in the 2014 season, so drivers that raced before then
    /// may not have one, represented by `None`. Drivers may also have used other numbers at
    /// some point in their career, e.g. when substituting for another driver, when only
    /// participating in free-practice sessions, when using the number `1`, etc. The number `1` is
    /// reserved for the previous season's World Drivers' Champion, although it is not mandatory for
    /// the driver to run the number. Most notably, Max Verstappen has used the number `1` since
    /// 2022, following his titles in 2021, 2022, 2023, and 2024. For more information, see the
    /// [List of Formula One driver numbers](https://en.wikipedia.org/wiki/List_of_Formula_One_driver_numbers)
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub permanent_number: Option<u32>,
    /// Three-letter code associated with the driver, if any, e.g. `"VER"` for _Max Verstappen_.
    // @todo Add more information about three-letter codes; there are exceptions and special cases.
    pub code: Option<String>,
    /// URL to the Wikipedia page for this driver, e.g. for _Max Verstappen_:
    /// [`"https://en.wikipedia.org/wiki/Max_Verstappen"`](https://en.wikipedia.org/wiki/Max_Verstappen)
    pub url: Url,
    /// Given name of the driver, e.g. `"Max"` for _Max Verstappen_.
    pub given_name: String,
    /// Family name of the driver, e.g. `"Verstappen"` for _Max Verstappen_.
    pub family_name: String,
    /// Date of birth of the driver, e.g. `1997-09-30` for _Max Verstappen_.
    pub date_of_birth: Date,
    /// Nationality of the driver, e.g. `"Dutch"` for _Max Verstappen_.
    pub nationality: String,
}

impl Driver {
    /// Returns the full name of this [`Driver`], i.e. the concatenation of
    /// [`given_name`](Self::given_name) and [`family_name`](Self::family_name).
    pub fn full_name(&self) -> String {
        format!("{} {}", self.given_name, self.family_name)
    }
}

impl TableInnerList for Driver {
    fn try_into_inner_from(table: Table) -> Result<InnerList<Self>> {
        table.into_drivers().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&InnerList<Self>> {
        table.as_drivers().ok_or(Error::BadTableVariant)
    }
}

/// Holds information about a Formula 1 constructor/team.
///
/// Requested via [`Resource::ConstructorInfo`] and returned in [`Table::Constructors`].
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    /// Unique identifier for the constructor, e.g. `"red_bull"` for _Red Bull Racing_.
    pub constructor_id: ConstructorID,
    /// URL to the Wikipedia page for this constructor, e.g. for _Red Bull Racing_:
    /// [`"https://en.wikipedia.org/wiki/Red_Bull_Racing"`](https://en.wikipedia.org/wiki/Red_Bull_Racing)
    pub url: Url,
    /// Name of the constructor, e.g. `"Red Bull"` for _Red Bull Racing_.
    pub name: String,
    /// Nationality of the constructor, e.g. `"Austrian"` for _Red Bull Racing_.
    pub nationality: String,
}

impl TableInnerList for Constructor {
    fn try_into_inner_from(table: Table) -> Result<InnerList<Self>> {
        table.into_constructors().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&InnerList<Self>> {
        table.as_constructors().ok_or(Error::BadTableVariant)
    }
}

/// Holds information about a Formula 1 circuit/track.
///
/// Requested via [`Resource::CircuitInfo`] and returned in [`Table::Circuits`].
#[derive(Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    /// Unique identifier for the circuit, e.g. `"spa"` for the _Circuit de Spa-Francorchamps_.
    pub circuit_id: CircuitID,
    /// URL to the Wikipedia page for this circuit, e.g. for the _Circuit de Spa-Francorchamps_:
    /// [`"https://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps"`](https://en.wikipedia.org/wiki/Circuit_de_Spa-Francorchamps)
    pub url: Url,
    /// Name of the circuit, e.g. `"Circuit de Spa-Francorchamps"` for the _Circuit de
    /// Spa-Francorchamps_.
    pub circuit_name: String,
    /// Geographical location of the circuit, represented by a [`Location`].
    #[serde(rename = "Location")]
    pub location: Location,
}

impl TableInnerList for Circuit {
    fn try_into_inner_from(table: Table) -> Result<InnerList<Self>> {
        table.into_circuits().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&InnerList<Self>> {
        table.as_circuits().ok_or(Error::BadTableVariant)
    }
}

/// Holds information about a Formula 1 session result status.
///
/// Requested via [`Resource::FinishingStatus`] and returned in [`Table::Status`].
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    /// Unique numerical identifier for the status, e.g. `1` for "Finished".
    #[serde_as(as = "DisplayFromStr")]
    pub status_id: StatusID,
    /// Number of occurrences of this status in a given time period, e.g. during a season.
    #[serde_as(as = "DisplayFromStr")]
    pub count: u32,
    /// Description of the status, e.g. `"Finished"`.
    pub status: String,
}

impl TableInnerList for Status {
    fn try_into_inner_from(table: Table) -> Result<InnerList<Self>> {
        table.into_status().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&InnerList<Self>> {
        table.as_status().ok_or(Error::BadTableVariant)
    }
}

/// This generic struct represents a race weekend event, corresponding to the list element type
/// under the `"RaceTable.Races"` property key in the JSON response from the jolpica-f1 API. The
/// generic type parameter `T` represents the type of payload that may be returned, depending on the
/// requested [`Resource`]. The default <code>T = [Payload]</code> accepts all possible payload
/// types, but the `T` parameter may be specified during postprocessing to restrict the payload
/// type, e.g. by `get_*` API functions that know the expected payload variant.
#[serde_as]
#[derive(Deserialize, Eq, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Race<T = Payload> {
    /// Unique identifier, i.e. year, for the season in which this race weekend event takes place,
    /// e.g. `2023` for the _2023 Formula One World Championship_. See [`Season::season`] and
    /// [`Filters::season`].
    #[serde_as(as = "DisplayFromStr")]
    pub season: SeasonID,
    /// Identifier for this race weekend event within the season, a numerical index starting from
    /// `1` for the first one of the season. See [`Resource::RaceSchedule`] and [`Filters::round`].
    #[serde_as(as = "DisplayFromStr")]
    pub round: RoundID,
    /// URL to the Wikipedia page for this race weekend event, e.g. for the 2023 Belgian Grand Prix:
    /// [`https://en.wikipedia.org/wiki/2023_Belgian_Grand_Prix`](https://en.wikipedia.org/wiki/2023_Belgian_Grand_Prix)
    pub url: Url,
    /// Name of this race weekend event across seasons, e.g. the `"Belgian Grand Prix"`.
    pub race_name: String,
    /// Circuit that this race weekend event takes place at, e.g. the _Circuit de Spa-Francorchamps_
    /// for the Belgian Grand Prix.
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    /// Date that this race takes place on, e.g. `2023-07-30` for the 2023 Belgian Grand Prix.
    ///
    /// This is the date of the Sunday race. See [`Schedule`] for the dates of other sessions.
    pub date: Date,
    #[serde(default, deserialize_with = "deserialize_optional_time")]
    /// Time that this race starts at, e.g. `13:00:00Z` for the 2023 Belgian Grand Prix.
    ///
    /// This is the time of the Sunday race. See [`Schedule`] for the times of other sessions.
    /// For some historical races the time may not be available, represented by `None`.
    pub time: Option<Time>,
    #[serde(flatten)]
    /// Payload data associated with this race weekend event, of generic type `T`.
    ///
    /// By default, <code>T = [Payload]</code> accepts all possible payload variants that may be
    /// returned by the jolpica-f1 API depending on the requested [`Resource`]. However, the type
    /// parameter `T` may be specified to provide a processed payload, e.g. [`Vec<RaceResult>`]
    /// from the [`Payload::RaceResults`] variant, [`RaceResult`] for a single result, etc. See
    /// [`Race::try_map()`] to map [`Race<Payload>`] into more specific [`Race<T>`] types.
    pub payload: T,
}

impl<T> Race<T> {
    /// Returns a tuple with references to all the fields of this [`Race`] except for the `payload`
    /// field, to allow comparing [`Race`]s for equality while ignoring [`payload`](Self::payload).
    //
    // @todo If a new field is added to [`Race`], and this impl isn't updated accordingly, then
    // comparisons will silently fail - unit tests won't catch it. I haven't figured out a way to
    // solve this without a lot of inefficient cloning to discard payload and compare [`Race<Void>`]
    pub const fn as_info(&self) -> (&SeasonID, &RoundID, &Url, &String, &Circuit, &Date, &Option<Time>) {
        (&self.season, &self.round, &self.url, &self.race_name, &self.circuit, &self.date, &self.time)
    }

    /// Returns a tuple with all the fields of this [`Race`] except for the `payload` field, to
    /// allow comparing [`Race`]s for equality while ignoring [`payload`](Self::payload). This
    /// method is more inefficient than [`as_info()`](Self::as_info) as it clones all of the fields.
    /// It should only be used when `as_info()`would be too inconvenient due to lifetime, etc.
    //
    // @todo See the comment in [`as_info()`](Self::as_info).
    pub fn to_info(&self) -> (SeasonID, RoundID, Url, String, Circuit, Date, Option<Time>) {
        (
            self.season,
            self.round,
            self.url.clone(),
            self.race_name.clone(),
            self.circuit.clone(),
            self.date,
            self.time,
        )
    }

    /// Maps a [`Race<T>`] to a [`Result<Race<U>, E>`] by applying a type `T` -> `U` conversion
    /// function, which may fail with error `E`, to the payload, and keeping all the other fields.
    // @todo This implementation can be simplified if/once the type_chaining_struct_update feature
    // is implemented and stabilized; see tracking https://github.com/rust-lang/rust/issues/86555.
    pub fn try_map<U, F, E>(self, op: F) -> std::result::Result<Race<U>, E>
    where
        F: FnOnce(T) -> std::result::Result<U, E>,
        E: std::error::Error,
    {
        Ok(Race::<U> {
            season: self.season,
            round: self.round,
            url: self.url,
            race_name: self.race_name,
            circuit: self.circuit,
            date: self.date,
            time: self.time,
            payload: op(self.payload)?,
        })
    }

    /// Maps a [`Race<T>`] to a [`Race<U>`] by applying a type `T` -> `U` conversion function to the
    /// payload and keeping all other fields unchanged.
    pub fn map<U, F>(self, op: F) -> Race<U>
    where
        F: FnOnce(T) -> U,
    {
        self.try_map(|payload| Ok::<_, Infallible>(op(payload))).unwrap()
    }

    /// Constructs a [`Race<T>`] from a [`Race<U>`] and a payload argument of type `T`.
    pub fn from<U>(race: Race<U>, payload: T) -> Self {
        race.map(|_| payload)
    }
}

impl TableInnerList for Race<Payload> {
    fn try_into_inner_from(table: Table) -> Result<InnerList<Self>> {
        table.into_races().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&InnerList<Self>> {
        table.as_races().ok_or(Error::BadTableVariant)
    }
}

/// Holds scheduling information for sessions of a Formula 1 race weekend event.
///
/// Requested via [`Resource::RaceSchedule`] and returned in [`Payload::Schedule`].
#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Schedule {
    /// Date and time of the first free-practice session, if any.
    #[serde(rename = "FirstPractice")]
    pub first_practice: Option<DateTime>,
    /// Date and time of the second free-practice session, if any.
    #[serde(rename = "SecondPractice")]
    pub second_practice: Option<DateTime>,
    /// Date and time of the third free-practice session, if any.
    #[serde(rename = "ThirdPractice")]
    pub third_practice: Option<DateTime>,
    /// Date and time of the qualifying session, if any.
    #[serde(rename = "Qualifying")]
    pub qualifying: Option<DateTime>,
    /// Date and time of the sprint session, if any.
    #[serde(rename = "Sprint")]
    pub sprint: Option<DateTime>,
    /// Date and time of the sprint shootout session, if any.
    ///
    /// This is a dedicated qualifying session for the sprint race. It was dubbed "sprint shootout"
    /// in 2023, when it was first introduced, but in 2024 it was renamed to "sprint qualifying".
    #[serde(rename = "SprintShootout")]
    pub sprint_shootout: Option<DateTime>,
    /// Date and time of the sprint qualifying session, if any.
    ///
    /// This is a dedicated qualifying session for the sprint race. It was dubbed "sprint shootout"
    /// in 2023, when it was first introduced, but in 2024 it was renamed to "sprint qualifying".
    #[serde(rename = "SprintQualifying")]
    pub sprint_qualifying: Option<DateTime>,
}

impl Race<Schedule> {
    /// Returns a reference to the field [`Race::payload`], a [`Schedule`].
    pub const fn schedule(&self) -> &Schedule {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a [`Schedule`].
    pub fn into_schedule(self) -> Schedule {
        self.payload
    }
}

/// [`Payload`] represents all the possible different data elements that be me returned as part of
/// a [`Race`] in a [`Response`] from the jolpica-f1 API.
///
/// For example, [`Payload::SprintResults`] corresponds to the `"SprintResults"` property key in the
/// JSON response, which is a list of [`SprintResult`]. One and only one of these payloads may be
/// returned in a given response, depending on the requested [`Resource`], which is represented by
/// the different variants of this enum.
///
/// The variants and inner values may be matched and accessed via the usual pattern matching, or via
/// accessor functions provided by  [`enum-as-inner`](https://crates.io/crates/enum-as-inner).
///
/// # Examples:
///
/// ```
/// # use url::Url;
/// # use f1_data::jolpica::response::{Payload, SprintResult};
/// #
/// let payload = Payload::Laps(vec![]);
///
/// let Payload::Laps(laps) = &payload else {
///     panic!("Expected Laps variant");
/// };
/// assert!(laps.is_empty());
///
/// assert!(payload.as_laps().unwrap().is_empty());
/// ```
#[derive(EnumAsInner, PartialEq, Clone, Debug)]
pub enum Payload {
    /// Contains a list of [`QualifyingResult`]s, and corresponds to the `"QualifyingResults"`
    /// property key in the JSON response from the jolpica-f1 API.
    QualifyingResults(Vec<QualifyingResult>),

    /// Contains a list of [`SprintResult`]s, and corresponds to the `"SprintResults"` property key
    /// in the JSON response from the jolpica-f1 API.
    SprintResults(Vec<SprintResult>),

    /// Contains a list of [`RaceResult`]s, and corresponds to the `"Results"` property key in the
    /// JSON response from the jolpica-f1 API.
    RaceResults(Vec<RaceResult>),

    /// Contains a list of [`Lap`]s, and corresponds to the `"Laps"` property key in the JSON
    /// response from the jolpica-f1 API.
    Laps(Vec<Lap>),

    /// Contains a list of [`PitStop`]s, and corresponds to the `"PitStops"` property key in the
    /// JSON response from the jolpica-f1 API.
    PitStops(Vec<PitStop>),

    /// Contains a [`Schedule`] object, and corresponds to the absence of a tag property key in the
    /// JSON response from the jolpica-f1 API. That is, all the elements of a schedule are flattened
    /// directly into the [`Race`] object in JSON.
    ///
    /// **Note:** Because of the untagged nature of this variant, and because all of the fields of
    /// [`Schedule`] are optional, it no payload is returned this variant will be the one being set.
    /// This is also a valid response from the jolpica-f1 API, e.g. for races prior to 2022, where
    /// scheduling information was limited to the date/time of the Grand Prix (race), which is
    /// already included in the [`Race`] object, as it does not depend on the `Resource` request.
    Schedule(Schedule),
}

impl<'de> Deserialize<'de> for Payload {
    /// Custom deserializer for [`Payload`]. It is functionally not very different from the one
    /// provided by the [`Deserialize`] derive macro, except that, if there are any problems when
    /// parsing one of the tagged variants - i.e. not [`Payload::Schedule`] - it will produce an
    /// [`Err`] with a helpful message indicating what went wrong during parsing. The default
    /// implementation would just result in [`Payload::Schedule`] with all fields set to [`None`],
    /// which usually later manifests as a cryptic and unhelpful [`Error::BadPayloadVariant`].
    // @todo See if this could be implemented without a custom deserializer, or if it's something
    // that could and should be improved in serde: https://github.com/serde-rs/serde/pull/2403
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        fn from_value<'de, T, D>(value: serde_json::Value) -> std::result::Result<T, D::Error>
        where
            T: DeserializeOwned,
            D: Deserializer<'de>,
        {
            serde_json::from_value(value).map_err(serde::de::Error::custom)
        }

        #[derive(Deserialize)]
        enum Proxy {
            QualifyingResults(serde_json::Value),
            SprintResults(serde_json::Value),
            #[serde(rename = "Results")]
            RaceResults(serde_json::Value),
            Laps(serde_json::Value),
            PitStops(serde_json::Value),
            #[serde(untagged)]
            Schedule(Schedule),
        }

        match Proxy::deserialize(deserializer)? {
            Proxy::QualifyingResults(value) => from_value::<_, D>(value).map(Self::QualifyingResults),
            Proxy::SprintResults(value) => from_value::<_, D>(value).map(Self::SprintResults),
            Proxy::RaceResults(value) => from_value::<_, D>(value).map(Self::RaceResults),
            Proxy::Laps(value) => from_value::<_, D>(value).map(Self::Laps),
            Proxy::PitStops(value) => from_value::<_, D>(value).map(Self::PitStops),
            Proxy::Schedule(schedule) => Ok(Self::Schedule(schedule)),
        }
    }
}

/// This trait allows the generic extraction of the inner list types of all [`Payload`] variants.
///
/// For example, [`RaceResult`]s can be extracted from a [`Race`]'s [`Race::payload`], from the
/// [`Payload::RaceResults`] variant, via  [`T::try_into_inner_from()`](Self::try_into_inner_from).
///
/// The trait is implemented for [`QualifyingResult`], [`SprintResult`], and [`RaceResult`].
pub trait PayloadInnerList
where
    Self: Sized,
{
    /// Extract the inner value from the corresponding [`Payload`] variant for this
    /// [`PayloadInnerList`], e.g. a [`Vec<RaceResult>`] from the [`Payload::RaceResults`] variant.
    fn try_into_inner_from(payload: Payload) -> Result<InnerList<Self>>;
}

/// Holds information about a driver's qualifying result in a Formula 1 qualifying session.
///
/// Requested via [`Resource::QualifyingResults`] and returned in [`Payload::QualifyingResults`].
///
/// Qualifying session formats have changed over the years, and have not always included the
/// three-stage knockout format (Q1, Q2, Q3) that has been in use since 2006. As such, the `q1-3`
/// fields are populated in that order based on the number of stages that were held in the session.
/// For example, in 2000 there was only a single qualifying session, so only `q1` is populated. In
/// 2005 there were two sessions, so `q1` and `q2` are populated. From 2006 onwards all three
/// sessions are held, so all three fields are populated. If a driver was eliminated in an
/// earlier session, the later session fields are not populated, represented by `None`. Qualifying
/// results are not available prior to 1994, so all three stage fields are `None` for those years.
///
/// See [Formula One qualifying](https://en.wikipedia.org/wiki/Formula_One_race_weekend#Qualifying)
/// for more details about the different qualifying formats, including sprint qualifying sessions.
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct QualifyingResult {
    /// Driver's car number during the qualifying session.
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    /// Driver's qualifying position after the session, `1` being pole position.
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    /// The driver that this qualifying result corresponds to.
    #[serde(rename = "Driver")]
    pub driver: Driver,
    /// The constructor/team that the driver was driving for during the qualifying session.
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    /// Driver's qualifying times for the Q1/first "knockout" stage of the qualifying session.
    ///
    /// Qualifying results may not be available, e.g. prior to 1994, represented by [`None`].
    #[serde(rename = "Q1")]
    pub q1: Option<QualifyingTime>,
    /// Driver's qualifying times for the Q2/second "knockout" stage of the qualifying session.
    ///
    /// Qualifying results may not be available, e.g. if the driver was eliminated in Q1, or on
    /// any of the years that only had a single qualifying session, represented by [`None`].
    #[serde(rename = "Q2")]
    pub q2: Option<QualifyingTime>,
    /// Driver's qualifying times for the Q3/third "knockout" stage of the qualifying session.
    ///
    /// Qualifying results may not be available, e.g. if the driver was eliminated in Q1 or Q2, or
    /// on any of the years that had only one or two qualifying sessions, represented by [`None`].
    #[serde(rename = "Q3")]
    pub q3: Option<QualifyingTime>,
}

impl Race<Vec<QualifyingResult>> {
    /// Returns a reference to the field [`Race::payload`], a list of [`QualifyingResult`]s.
    pub fn qualifying_results(&self) -> &[QualifyingResult] {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a list of [`QualifyingResult`]s.
    pub fn into_qualifying_results(self) -> Vec<QualifyingResult> {
        self.payload
    }
}

impl Race<QualifyingResult> {
    /// Returns a reference to the field [`Race::payload`], a single [`QualifyingResult`].
    pub const fn qualifying_result(&self) -> &QualifyingResult {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a single [`QualifyingResult`].
    pub fn into_qualifying_result(self) -> QualifyingResult {
        self.payload
    }
}

impl PayloadInnerList for QualifyingResult {
    fn try_into_inner_from(payload: Payload) -> Result<InnerList<Self>> {
        payload.into_qualifying_results().map_err(into)
    }
}

/// Represents points awarded, e.g. for a sprint/race finish, fastest lap, etc.
///
/// These are represented as floating point because some events may award fractional points, e.g.
/// the 2021 Belgian GP only awarded half points, meaning P1, P3, and P10 received `x.5` points.
pub type Points = f32;

/// Holds information about a driver's result in a Formula 1 sprint session.
///
/// Requested via [`Resource::SprintResults`] and returned in [`Payload::SprintResults`].
#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SprintResult {
    /// Driver's car number during the sprint.
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    /// Driver's classified position in the sprint, even if they did not finish.
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    /// Indicates the driver's result in the sprint, e.g. [`Position::Finished`] containing
    /// the finishing position if the driver finished, or any of other possible outcomes, e.g.
    /// [`Position::Retired`], [`Position::Disqualified`], [`Position::Withdrawn`],etc.
    pub position_text: Position,
    /// Points awarded to the driver for their result in the sprint.
    #[serde_as(as = "DisplayFromStr")]
    pub points: Points,
    /// The driver that this sprint result corresponds to.
    #[serde(rename = "Driver")]
    pub driver: Driver,
    /// The constructor/team that the driver was driving for during the sprint.
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    /// Driver's starting grid position for the sprint.
    #[serde_as(as = "DisplayFromStr")]
    pub grid: u32,
    /// Number of laps completed by the driver during the sprint.
    #[serde_as(as = "DisplayFromStr")]
    pub laps: u32,
    /// Driver's status at the end of the sprint, e.g. `"Finished"`, `"Retired"`, etc.
    pub status: String,
    /// Full sprint duration for the driver, including possibly a delta to the sprint leader/P1.
    /// This is only present if a driver finished in the lead lap, if their status is `"Finished"`.
    // @todo If and when the API bug is fixed, this can be changed back to:
    // #[serde(rename = "Time")]
    #[serde(rename = "Time", default, deserialize_with = "deserialize_buggy_race_time")]
    pub time: Option<RaceTime>,
    /// Information about the driver's fastest lap during the sprint.
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

impl Race<Vec<SprintResult>> {
    /// Returns a reference to the field [`Race::payload`], a list of [`SprintResult`]s.
    pub fn sprint_results(&self) -> &[SprintResult] {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a list of [`SprintResult`]s.
    pub fn into_sprint_results(self) -> Vec<SprintResult> {
        self.payload
    }
}

impl Race<SprintResult> {
    /// Returns a reference to the field [`Race::payload`], a single [`SprintResult`].
    pub const fn sprint_result(&self) -> &SprintResult {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a single [`SprintResult`].
    pub fn into_sprint_result(self) -> SprintResult {
        self.payload
    }
}

impl PayloadInnerList for SprintResult {
    fn try_into_inner_from(payload: Payload) -> Result<InnerList<Self>> {
        payload.into_sprint_results().map_err(into)
    }
}

/// Holds information about a driver's result in a Formula 1 Grand Prix (race session).
///
/// Requested via [`Resource::RaceResults`] and returned in [`Payload::RaceResults`].
#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RaceResult {
    /// Driver's car number during the race.
    #[serde(deserialize_with = "deserialize_possible_no_number")]
    pub number: u32,
    /// Driver's classified position in the race, even if they did not finish.
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    /// Indicates the driver's result in the race, e.g. [`Position::Finished`] containing the
    /// finishing position if the driver finished, or any of other possible outcomes, e.g.
    /// [`Position::Retired`], [`Position::Disqualified`], [`Position::Withdrawn`],etc.
    pub position_text: Position,
    /// Points awarded to the driver for their result in the race, including any fastest lap points.
    #[serde_as(as = "DisplayFromStr")]
    pub points: Points,
    /// The driver that this race result corresponds to.
    #[serde(rename = "Driver")]
    pub driver: Driver,
    /// The constructor/team that the driver was driving for during the race.
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    /// Driver's starting grid position for the race.
    #[serde_as(as = "DisplayFromStr")]
    pub grid: u32,
    /// Number of laps completed by the driver during the race.
    #[serde_as(as = "DisplayFromStr")]
    pub laps: u32,
    /// Driver's status at the end of the race, e.g. `"Finished"`, `"Retired"`, etc.
    pub status: String,
    /// Full race duration for the driver, including possibly a delta to the race leader/P1.
    /// This is only present if a driver finished in the lead lap, if their status is `"Finished"`.
    // @todo If and when the API bug is fixed, this can be changed back to:
    // #[serde(rename = "Time")]
    #[serde(rename = "Time", default, deserialize_with = "deserialize_buggy_race_time")]
    pub time: Option<RaceTime>,
    /// Information about the driver's fastest lap during the race.
    #[serde(rename = "FastestLap")]
    pub fastest_lap: Option<FastestLap>,
}

impl RaceResult {
    /// Represents that no car number was assigned to a race result, set in [`RaceResult::number`].
    /// This only happened for a few entries in two races in the 1960s. As such, it's not worth the
    /// ergonomic cost to have the [`RaceResult::number`] field be [`Option`], and instead this
    /// value will be set for any [`RaceResult`] where the entry was not assigned a car number.
    ///
    /// The historical race results without a car number are:
    ///   - 1962, round 4 (French Grand Prix): P19-22
    ///   - 1963, round 10 (South African Grand Prix): P23
    pub const NO_NUMBER: u32 = u32::MAX;
}

impl Race<Vec<RaceResult>> {
    /// Returns a reference to the field [`Race::payload`], a list of [`RaceResult`]s.
    pub fn race_results(&self) -> &[RaceResult] {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a list of [`RaceResult`]s.
    pub fn into_race_results(self) -> Vec<RaceResult> {
        self.payload
    }
}

impl Race<RaceResult> {
    /// Returns a reference to the field [`Race::payload`], a single [`RaceResult`].
    pub const fn race_result(&self) -> &RaceResult {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a single [`RaceResult`].
    pub fn into_race_result(self) -> RaceResult {
        self.payload
    }
}

impl PayloadInnerList for RaceResult {
    fn try_into_inner_from(payload: Payload) -> Result<InnerList<Self>> {
        payload.into_race_results().map_err(into)
    }
}

/// Deserialize a `u32` from a string, where empty is represented by [`RaceResult::NO_NUMBER`].
fn deserialize_possible_no_number<'de, D>(deserializer: D) -> std::result::Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).and_then(|str| {
        if str == "None" {
            Ok(RaceResult::NO_NUMBER)
        } else {
            str.parse::<u32>().map_err(serde::de::Error::custom)
        }
    })
}

/// Represents a driver's result outcome in a Formula 1 sprint or race session.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Position {
    /// Driver finished the session, with the contained `u32` representing their finishing position.
    Finished(u32),
    /// Driver retired from the session.
    Retired,
    /// Driver was disqualified from the session.
    Disqualified,
    /// Driver was excluded from the session.
    Excluded,
    /// Driver withdrew from the session.
    Withdrawn,
    /// Driver failed to qualify for the session.
    FailedToQualify,
    /// Driver was not classified in the session.
    NotClassified,
}

impl Position {
    /// Shorthand constant for [`Position::Retired`], i.e. [`Position::R`] or [`Self::R`].
    pub const R: Self = Self::Retired;
    /// Shorthand constant for [`Position::Disqualified`], i.e. [`Position::D`] or [`Self::D`].
    pub const D: Self = Self::Disqualified;
    /// Shorthand constant for [`Position::Excluded`], i.e. [`Position::E`] or [`Self::E`].
    pub const E: Self = Self::Excluded;
    /// Shorthand constant for [`Position::Withdrawn`], i.e. [`Position::W`] or [`Self::W`].
    pub const W: Self = Self::Withdrawn;
    /// Shorthand constant for [`Position::FailedToQualify`], i.e. [`Position::F`] or [`Self::F`].
    pub const F: Self = Self::FailedToQualify;
    /// Shorthand constant for [`Position::NotClassified`], i.e. [`Position::N`] or [`Self::N`].
    pub const N: Self = Self::NotClassified;
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        match String::deserialize(deserializer)?.as_str() {
            "R" => Ok(Self::R),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "W" => Ok(Self::W),
            "F" => Ok(Self::F),
            "N" => Ok(Self::N),
            num => Ok(Self::Finished(
                num.parse::<u32>()
                    .map_err(|err| serde::de::Error::custom(err.to_string()))?,
            )),
        }
    }
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

/// Holds information about a single lap in a Formula 1 sprint or race session.
///
/// Requested via [`Resource::LapTimes`] and returned in [`Payload::Laps`].
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Lap {
    /// Lap number within the session, starting from `1` for the first lap.
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    /// List of [`Timing`]s for all drivers for this lap.
    #[serde(rename = "Timings")]
    pub timings: Vec<Timing>,
}

/// Holds timing information for a single driver in a given lap of a sprint or race.
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    /// Unique identifier for the driver that this timing corresponds to.
    pub driver_id: DriverID,
    /// Position of the driver at the end of the lap.
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    /// Lap time for the driver in this lap.
    #[serde(deserialize_with = "deserialize_duration")]
    pub time: Duration,
}

/// Holds information about a single pit stop made by a driver in a Formula 1 sprint or race.
///
/// Requested via [`Resource::PitStops`] and returned in [`Payload::PitStops`].
#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PitStop {
    /// Unique identifier for the driver that made this pit stop.
    pub driver_id: DriverID,
    #[serde_as(as = "DisplayFromStr")]
    /// Lap number during which the pit stop was made.
    pub lap: u32,
    /// Pit stop index for the driver during the session, starting from `1` for their first stop.
    #[serde_as(as = "DisplayFromStr")]
    pub stop: u32,
    /// Time from the start of the race at which the pit stop was made.
    #[serde(deserialize_with = "deserialize_time")]
    pub time: Time,
    /// Duration of the pit stop from pit entry to pit exit.
    // @todo Double-check if it's actually from pit entry to pit exit.
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration: Duration,
}

/// Holds geographical location information, typically about a Formula 1 circuit/track.
#[serde_as]
#[derive(Deserialize, Hash, Eq, PartialEq, Clone, Debug)]
pub struct Location {
    /// Latitude of the location, e.g. `"50.4372"` for 50°26′14″N of Circuit de Spa-Francorchamps.
    #[serde_as(as = "DisplayFromStr")]
    pub lat: OrderedFloat<f64>,
    /// Longitude of the location, e.g. `"5.97139"` for 5°58′17″E of Circuit de Spa-Francorchamps.
    #[serde_as(as = "DisplayFromStr")]
    pub long: OrderedFloat<f64>,
    /// Locality (city/town) of the location, e.g. `"Spa"`, `"Monte-Carlo"`, `"Montreal"`, etc.
    pub locality: String,
    /// Country of the location, e.g. `"Belgium"`, `"Monaco"`, `"Canada"`, `"UK"`, etc.
    pub country: String,
}

/// Holds information about a driver's fastest lap in a Formula 1 sprint or race session.
#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct FastestLap {
    /// The rank of the fastest lap, e.g. `1` for the overall fastest lap in the session.
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub rank: Option<u32>,
    /// The lap number during which the fastest lap was set.
    #[serde_as(as = "DisplayFromStr")]
    pub lap: u32,
    /// The lap time of the fastest lap.
    #[serde(rename = "Time", deserialize_with = "extract_nested_time")]
    pub time: Duration,
    /// The average speed during the fastest lap.
    #[serde(rename = "AverageSpeed")]
    pub average_speed: Option<AverageSpeed>,
}

fn extract_nested_time<'de, D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Duration, D::Error> {
    #[derive(Deserialize)]
    struct Time {
        #[serde(deserialize_with = "deserialize_duration")]
        time: Duration,
    }
    Ok(Time::deserialize(deserializer)?.time)
}

/// Holds information about the average speed during a lap.
#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct AverageSpeed {
    /// The units used for the speed measurement, e.g. kilometers per hour, [`SpeedUnits::Kph`].
    pub units: SpeedUnits,
    /// The average speed value.
    #[serde_as(as = "DisplayFromStr")]
    pub speed: f32,
}

/// Represents the units used for speed measurements.
#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum SpeedUnits {
    /// Kilometers per hour.
    #[serde(rename = "kph")]
    Kph,
}

/// Check that there is exactly one element `T` in a slice `&[T]`, and return a
/// <code>[Result<&\[T\]>]</code> containing the slice if so, [`Error::NotFound`] if it contained no
/// elements, or [`Error::TooMany`] if it contained more than one.
pub(crate) const fn verify_has_one_element<T>(sequence: &[T]) -> Result<&[T]> {
    match sequence.len() {
        0 => Err(Error::NotFound),
        1 => Ok(sequence),
        _ => Err(Error::TooMany),
    }
}

/// Extract a single element `T` from [`Vec<T>`] into [`Result<T>`], enforcing that there is only
/// one element in the vector, returning [`Error::NotFound`] if it contained no elements, or
/// [`Error::TooMany`] if it contained more than one.
pub(crate) fn verify_has_one_element_and_extract<T>(mut sequence: Vec<T>) -> Result<T> {
    match sequence.len() {
        0 => Err(Error::NotFound),
        1 => Ok(sequence.remove(0)),
        _ => Err(Error::TooMany),
    }
}

/// Extract single [`Race`] from a [`Response`], into [`Result<Race>`], enforcing that there is only
/// one race in the [`Response`], returning [`Error::NotFound`] if the it contained no races, or
/// [`Error::TooMany`] if it contained more than one.
pub(crate) fn verify_has_one_race_and_extract(response: Response) -> Result<Race> {
    response
        .table
        .into_races()
        .map_err(into)
        .and_then(verify_has_one_element_and_extract)
}

/// Shorthand for closure `|e| e.into()` and/or `std::convert::Into::into`.
// @todo Replace with an import once `import_trait_associated_functions` is stabilized:
// https://doc.rust-lang.org/nightly/unstable-book/language-features/import-trait-associated-functions.html
fn into<T: Into<U>, U>(t: T) -> U {
    t.into()
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use const_format::formatcp;

    use crate::jolpica::tests::assets::*;
    use crate::tests::asserts::*;
    use shadow_asserts::{assert_eq, assert_ne};

    use super::*;

    #[test]
    fn season_table() {
        let table: Table = serde_json::from_str(SEASON_TABLE_STR).unwrap();
        assert_false!(table.as_seasons().unwrap().is_empty());
        assert_eq!(table, *SEASON_TABLE);
    }

    #[test]
    fn driver_table() {
        let table: Table = serde_json::from_str(DRIVER_TABLE_STR).unwrap();
        assert_false!(table.as_drivers().unwrap().is_empty());
        assert_eq!(table, *DRIVER_TABLE);
    }

    #[test]
    fn constructor_table() {
        let table: Table = serde_json::from_str(CONSTRUCTOR_TABLE_STR).unwrap();
        assert_false!(table.as_constructors().unwrap().is_empty());
        assert_eq!(table, *CONSTRUCTOR_TABLE);
    }

    #[test]
    fn circuit_table() {
        let table: Table = serde_json::from_str(CIRCUIT_TABLE_STR).unwrap();
        assert_false!(table.as_circuits().unwrap().is_empty());
        assert_eq!(table, *CIRCUIT_TABLE);
    }

    #[test]
    fn race_table_schedule() {
        let table: Table = serde_json::from_str(RACE_TABLE_SCHEDULE_STR).unwrap();
        assert_false!(table.as_races().unwrap().is_empty());
        assert_eq!(table, *RACE_TABLE_SCHEDULE);
    }

    #[test]
    fn driver_full_name() {
        assert_eq!(DRIVER_KIMI.full_name(), "Kimi Räikkönen");
        assert_eq!(DRIVER_PEREZ.full_name(), "Sergio Pérez");
        assert_eq!(DRIVER_DE_VRIES.full_name(), "Nyck de Vries");
        assert_eq!(DRIVER_MAX.full_name(), "Max Verstappen");
        assert_eq!(DRIVER_LECLERC.full_name(), "Charles Leclerc");
    }

    #[test]
    fn qualifying_result() {
        let from_str = |result_str| serde_json::from_str::<QualifyingResult>(result_str).unwrap();

        assert_eq!(from_str(QUALIFYING_RESULT_2003_4_P1_STR), *QUALIFYING_RESULT_2003_4_P1);
        assert_eq!(from_str(QUALIFYING_RESULT_2003_4_P2_STR), *QUALIFYING_RESULT_2003_4_P2);
        assert_eq!(from_str(QUALIFYING_RESULT_2003_4_P20_STR), *QUALIFYING_RESULT_2003_4_P20);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_4_P1_STR), *QUALIFYING_RESULT_2023_4_P1);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_4_P2_STR), *QUALIFYING_RESULT_2023_4_P2);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_4_P3_STR), *QUALIFYING_RESULT_2023_4_P3);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_10_P4_STR), *QUALIFYING_RESULT_2023_10_P4);
        assert_eq!(from_str(QUALIFYING_RESULT_2023_12_P2_STR), *QUALIFYING_RESULT_2023_12_P2);
    }

    #[test]
    fn qualifying_results() {
        {
            let race: Race = serde_json::from_str(RACE_2003_4_QUALIFYING_RESULTS_STR).unwrap();
            assert_false!(race.payload.as_qualifying_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2003_4_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_4_QUALIFYING_RESULTS_STR).unwrap();
            assert_false!(race.payload.as_qualifying_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2023_4_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_10_QUALIFYING_RESULTS_STR).unwrap();
            assert_false!(race.payload.as_qualifying_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2023_10_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_12_QUALIFYING_RESULTS_STR).unwrap();
            assert_false!(race.payload.as_qualifying_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2023_12_QUALIFYING_RESULTS);
        }
    }

    #[test]
    fn sprint_result() {
        let from_str = |result_str| serde_json::from_str::<SprintResult>(result_str).unwrap();

        assert_eq!(from_str(SPRINT_RESULT_2023_4_P1_STR), *SPRINT_RESULT_2023_4_P1);
    }

    #[test]
    fn sprint_results() {
        let race: Race = serde_json::from_str(RACE_2023_4_SPRINT_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_sprint_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2023_4_SPRINT_RESULTS);

        let race: Race = serde_json::from_str(RACE_2024_5_SPRINT_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_sprint_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2024_5_SPRINT_RESULTS);
    }

    #[test]
    fn race_result() {
        let from_str = |result_str| serde_json::from_str::<RaceResult>(result_str).unwrap();

        assert_eq!(from_str(RACE_RESULT_1963_10_P23_STR), *RACE_RESULT_1963_10_P23);

        assert_eq!(from_str(RACE_RESULT_2003_4_P1_STR), *RACE_RESULT_2003_4_P1);
        assert_eq!(from_str(RACE_RESULT_2003_4_P2_STR), *RACE_RESULT_2003_4_P2);
        assert_eq!(from_str(RACE_RESULT_2003_4_P19_STR), *RACE_RESULT_2003_4_P19);

        assert_eq!(from_str(RACE_RESULT_2021_12_P1_STR), *RACE_RESULT_2021_12_P1);
        assert_eq!(from_str(RACE_RESULT_2021_12_P2_STR), *RACE_RESULT_2021_12_P2);
        assert_eq!(from_str(RACE_RESULT_2021_12_P3_STR), *RACE_RESULT_2021_12_P3);
        assert_eq!(from_str(RACE_RESULT_2021_12_P10_STR), *RACE_RESULT_2021_12_P10);

        assert_eq!(from_str(RACE_RESULT_2023_4_P1_STR), *RACE_RESULT_2023_4_P1);
        assert_eq!(from_str(RACE_RESULT_2023_4_P2_STR), *RACE_RESULT_2023_4_P2);
        assert_eq!(from_str(RACE_RESULT_2023_4_P20_STR), *RACE_RESULT_2023_4_P20);
    }

    #[test]
    fn race_results() {
        let race: Race = serde_json::from_str(RACE_1950_5_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_1950_5_RACE_RESULTS);

        let race: Race = serde_json::from_str(RACE_1963_10_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_1963_10_RACE_RESULTS);

        let race: Race = serde_json::from_str(RACE_1998_8_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_1998_8_RACE_RESULTS);

        let race: Race = serde_json::from_str(RACE_2003_4_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2003_4_RACE_RESULTS);

        let race: Race = serde_json::from_str(RACE_2020_9_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2020_9_RACE_RESULTS);

        let race: Race = serde_json::from_str(RACE_2021_12_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2021_12_RACE_RESULTS);

        let race: Race = serde_json::from_str(RACE_2023_3_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2023_3_RACE_RESULTS);

        let race: Race = serde_json::from_str(RACE_2023_4_RACE_RESULTS_STR).unwrap();
        assert_false!(race.payload.as_race_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2023_4_RACE_RESULTS);
    }

    #[test]
    fn finishing_status() {
        let table: Table = serde_json::from_str(STATUS_TABLE_2022_STR).unwrap();
        assert_false!(table.as_status().unwrap().is_empty());
        assert_eq!(table, *STATUS_TABLE_2022);
    }

    #[test]
    fn timing() {
        let from_str = |timing_str| serde_json::from_str::<Timing>(timing_str).unwrap();

        assert_eq!(from_str(TIMING_2023_4_L1_P1_STR), *TIMING_2023_4_L1_P1);
        assert_eq!(from_str(TIMING_2023_4_L1_P2_STR), *TIMING_2023_4_L1_P2);
        assert_eq!(from_str(TIMING_2023_4_L2_P1_STR), *TIMING_2023_4_L2_P1);
        assert_eq!(from_str(TIMING_2023_4_L2_P2_STR), *TIMING_2023_4_L2_P2);
    }

    #[test]
    fn lap() {
        let from_str = |lap_str| serde_json::from_str::<Lap>(lap_str).unwrap();

        assert_eq!(from_str(LAP_2023_4_L1_STR), *LAP_2023_4_L1);
        assert_eq!(from_str(LAP_2023_4_L2_STR), *LAP_2023_4_L2);
    }

    #[test]
    fn laps() {
        let race: Race = serde_json::from_str(RACE_2023_4_LAPS_STR).unwrap();

        let laps = race.payload.as_laps().unwrap();
        assert_false!(laps.is_empty());
        laps.iter().for_each(|lap| assert_false!(lap.timings.is_empty()));

        assert_eq!(race, *RACE_2023_4_LAPS);
    }

    #[test]
    fn pit_stop() {
        let from_str = |pit_stop_str| serde_json::from_str::<PitStop>(pit_stop_str).unwrap();

        assert_eq!(from_str(PIT_STOP_2023_4_L10_MAX_STR), *PIT_STOP_2023_4_L10_MAX);
        assert_eq!(from_str(PIT_STOP_2023_4_L11_LECLERC_STR), *PIT_STOP_2023_4_L11_LECLERC);
    }

    #[test]
    fn pit_stops() {
        let race: Race = serde_json::from_str(RACE_2023_4_PIT_STOPS_STR).unwrap();
        assert_false!(race.payload.as_pit_stops().unwrap().is_empty());
        assert_eq!(race, *RACE_2023_4_PIT_STOPS);
    }

    #[test]
    fn pagination_is_last_page() {
        assert_true!(
            Pagination {
                limit: 30,
                offset: 0,
                total: 16
            }
            .is_last_page()
        );

        assert_true!(
            Pagination {
                limit: 10,
                offset: 5,
                total: 15
            }
            .is_last_page()
        );

        assert_false!(
            Pagination {
                limit: 10,
                offset: 4,
                total: 15
            }
            .is_last_page()
        );
    }

    #[test]
    fn pagination_is_single_page() {
        assert_true!(
            Pagination {
                limit: 30,
                offset: 0,
                total: 16
            }
            .is_single_page()
        );

        assert_false!(
            Pagination {
                limit: 10,
                offset: 5,
                total: 15
            }
            .is_single_page()
        )
    }

    #[test]
    fn pagination_next_page() {
        assert_eq!(
            Pagination {
                limit: 10,
                offset: 0,
                total: 15
            }
            .next_page()
            .unwrap(),
            Pagination {
                limit: 10,
                offset: 10,
                total: 15
            }
        );

        assert_true!(
            Pagination {
                limit: 10,
                offset: 10,
                total: 15
            }
            .next_page()
            .is_none()
        );
    }

    #[test]
    fn pagination_deserialize() {
        const REF_PAGINATION: Pagination = Pagination {
            limit: 30,
            offset: 0,
            total: 16,
        };

        assert_eq!(
            serde_json::from_str::<Pagination>(
                r#"{
                "limit": "30",
                "offset": "0",
                "total": "16"
              }"#
            )
            .unwrap(),
            REF_PAGINATION
        );

        assert_eq!(
            serde_json::from_str::<Response>(
                r#"{
                  "MRData": {
                    "xmlns": "",
                    "series": "f1",
                    "url": "https://api.jolpi.ca/ergast/f1/races.json",
                    "limit": "30",
                    "offset": "0",
                    "total": "16",
                    "RaceTable": { "Races": [] }
                  }
                }"#
            )
            .unwrap()
            .pagination,
            REF_PAGINATION
        );
    }

    // Race::as_into() and .to_info()
    // -----------------------------

    fn verify_race_info_compare<T, U, V, F>(lhs: &Race<T>, mut rhs: Race<U>, mut field: F, new_val: V)
    where
        V: Clone + PartialEq + std::fmt::Debug,
        F: FnMut(&mut Race<U>) -> &mut V,
    {
        let old_val = field(&mut rhs).clone();
        assert_ne!(old_val, new_val);

        assert_eq!(lhs.as_info(), rhs.as_info());
        assert_eq!(lhs.to_info(), rhs.to_info());
        field(&mut rhs).clone_from(&new_val);
        assert_ne!(lhs.as_info(), rhs.as_info());
        assert_ne!(lhs.to_info(), rhs.to_info());

        field(&mut rhs).clone_from(&old_val);
        assert_eq!(lhs.as_info(), rhs.as_info());
        assert_eq!(lhs.to_info(), rhs.to_info());
    }

    #[test]
    fn race_as_to_info() {
        let lhs = RACE_2023_4.clone();

        assert_eq!(lhs.as_info(), lhs.as_info());
        assert_eq!(lhs.as_info(), Race::from(lhs.clone(), true).as_info());
        assert_eq!(lhs.to_info(), lhs.to_info());
        assert_eq!(lhs.to_info(), Race::from(lhs.clone(), true).to_info());

        let rhs = lhs.clone();

        verify_race_info_compare(&lhs, rhs.clone(), |r| &mut r.season, RACE_NONE.season);
        verify_race_info_compare(&lhs, rhs.clone(), |r| &mut r.round, RACE_NONE.round);
        verify_race_info_compare(&lhs, rhs.clone(), |r| &mut r.url, RACE_NONE.url.clone());
        verify_race_info_compare(&lhs, rhs.clone(), |r| &mut r.race_name, RACE_NONE.race_name.clone());
        verify_race_info_compare(&lhs, rhs.clone(), |r| &mut r.circuit, RACE_NONE.circuit.clone());
        verify_race_info_compare(&lhs, rhs.clone(), |r| &mut r.date, RACE_NONE.date);
        verify_race_info_compare(&lhs, rhs.clone(), |r| &mut r.time, RACE_NONE.time);
    }

    #[test]
    fn race_info_as_hash_key() {
        let mut map = indexmap::IndexMap::new();

        assert_true!(map.insert(RACE_2003_4.to_info(), RACE_2003_4.clone()).is_none());
        assert_true!(map.contains_key(&RACE_2003_4.to_info()));
        assert_false!(map.contains_key(&RACE_2023_4.to_info()));

        assert_true!(map.insert(RACE_2023_4.to_info(), RACE_2023_4.clone()).is_none());
        assert_true!(map.contains_key(&RACE_2003_4.to_info()));
        assert_true!(map.contains_key(&RACE_2023_4.to_info()));

        assert_ne!(*RACE_2003_4, *RACE_2023_4);
        assert_ne!(&RACE_2003_4.to_info(), &RACE_2023_4.to_info());
        assert_eq!(map[&RACE_2003_4.to_info()], *RACE_2003_4);
        assert_eq!(map[&RACE_2023_4.to_info()], *RACE_2023_4);
    }

    #[test]
    fn race_try_map() {
        let from = Race::from(RACE_2023_4.clone(), true);

        let into = from.clone().try_map::<_, _, Infallible>(|_| Ok(String::from("true")));
        assert_eq!(into.as_ref().unwrap().as_info(), from.as_info());
        assert_eq!(into.unwrap().payload, String::from("true"));

        #[derive(Debug)]
        struct DummyError;

        impl std::fmt::Display for DummyError {
            fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                Ok(())
            }
        }

        impl std::error::Error for DummyError {}

        let into = from.clone().try_map::<Infallible, _, _>(|_| Err(DummyError));
        assert_true!(into.is_err());
    }

    #[test]
    fn race_map() {
        let from = Race::from(RACE_2023_4.clone(), 1);

        let into = from.clone().map(|payload_i32| payload_i32.to_string());
        assert_eq!(into.as_info(), from.as_info());
        assert_eq!(into.payload, String::from("1"));
    }

    #[test]
    fn race_from() {
        let from = RACE_2023_4.clone();

        let into = Race::from(from.clone(), String::from("some"));
        assert_eq!(into.as_info(), from.as_info());
        assert_eq!(into.payload, String::from("some"));
    }

    fn map_race_schedule(race: Race<Payload>) -> Race<Schedule> {
        race.map(|payload| payload.into_schedule().unwrap())
    }

    #[test]
    fn payload_deserialize_helpful_errors() {
        static GOOD_STR: &str = formatcp!(r#"{{"QualifyingResults": [{QUALIFYING_RESULT_2023_4_P1_STR}]}}"#);
        static BAD_STR: &str = formatcp!(r#"{{"QualifyingResults": [{{"key": "data"}}]}}"#);

        let p = serde_json::from_str::<Payload>(GOOD_STR);
        assert_eq!(p.unwrap().as_qualifying_results().unwrap()[0], *QUALIFYING_RESULT_2023_4_P1);

        let p = serde_json::from_str::<Payload>(BAD_STR);
        assert_true!(p.unwrap_err().to_string().contains("missing field `number`"));
    }

    #[test]
    fn race_schedule_accessors() {
        let reference = RACE_2023_4_SCHEDULE.clone();
        let expected = reference.clone().payload.into_schedule().unwrap();

        let actual = map_race_schedule(reference.clone());
        assert_eq!(actual.schedule(), &expected);
        assert_eq!(actual.into_schedule(), expected);
    }

    fn map_race_multi_results<T: PayloadInnerList>(race: Race<Payload>) -> Race<Vec<T>> {
        race.map(|payload| T::try_into_inner_from(payload).unwrap())
    }

    fn map_race_single_result<T: PayloadInnerList>(race: Race<Payload>) -> Race<T> {
        map_race_multi_results(race).map(|payload| payload.into_iter().next().unwrap())
    }

    #[test]
    fn race_qualifying_result_accessors() {
        let reference = RACE_2023_4_QUALIFYING_RESULTS.clone();
        let expected = reference.clone().payload.into_qualifying_results().unwrap();

        let actual = map_race_multi_results(reference.clone());
        assert_eq!(actual.qualifying_results(), &expected);
        assert_eq!(actual.into_qualifying_results(), expected);

        let actual = map_race_single_result(reference.clone());
        assert_eq!(actual.qualifying_result(), &expected[0]);
        assert_eq!(actual.into_qualifying_result(), expected[0]);
    }

    #[test]
    fn race_sprint_result_accessors() {
        let reference = RACE_2023_4_SPRINT_RESULTS.clone();
        let expected = reference.clone().payload.into_sprint_results().unwrap();

        let actual = map_race_multi_results(reference.clone());
        assert_eq!(actual.sprint_results(), &expected);
        assert_eq!(actual.into_sprint_results(), expected);

        let actual = map_race_single_result(reference.clone());
        assert_eq!(actual.sprint_result(), &expected[0]);
        assert_eq!(actual.into_sprint_result(), expected[0]);
    }

    #[test]
    fn race_race_result_accessors() {
        let reference = RACE_2023_4_RACE_RESULTS.clone();
        let expected = reference.clone().payload.into_race_results().unwrap();

        let actual = map_race_multi_results(reference.clone());
        assert_eq!(actual.race_results(), &expected);
        assert_eq!(actual.into_race_results(), expected);

        let actual = map_race_single_result(reference.clone());
        assert_eq!(actual.race_result(), &expected[0]);
        assert_eq!(actual.into_race_result(), expected[0]);
    }

    #[test]
    fn deserialize_possible_no_number() {
        #[derive(Deserialize, Debug)]
        struct Proxy {
            #[serde(deserialize_with = "super::deserialize_possible_no_number")]
            number: u32,
        }

        assert_eq!(serde_json::from_str::<Proxy>(r#"{"number": "10"}"#).unwrap().number, 10);
        assert_eq!(serde_json::from_str::<Proxy>(r#"{"number": "None"}"#).unwrap().number, RaceResult::NO_NUMBER);
    }

    #[test]
    fn position() {
        assert_eq!(Position::Retired, Position::R);
        assert_ne!(Position::Retired, Position::E);

        let pos = Position::Finished(10);
        assert!(matches!(pos, Position::Finished(_)));
        assert_false!(matches!(pos, Position::E));

        match pos {
            Position::Finished(pos) => assert_eq!(pos, 10),
            _ => panic!("Expected Finished variant"),
        };
    }

    #[test]
    fn position_deserialize() {
        assert!(matches!(serde_json::from_str::<Position>("\"R\"").unwrap(), Position::R));
        assert!(matches!(serde_json::from_str::<Position>("\"D\"").unwrap(), Position::D));
        assert!(matches!(serde_json::from_str::<Position>("\"E\"").unwrap(), Position::E));
        assert!(matches!(serde_json::from_str::<Position>("\"W\"").unwrap(), Position::W));
        assert!(matches!(serde_json::from_str::<Position>("\"F\"").unwrap(), Position::F));
        assert!(matches!(serde_json::from_str::<Position>("\"N\"").unwrap(), Position::N));

        let Position::Finished(pos) = serde_json::from_str("\"10\"").unwrap() else {
            panic!("Expected Finished variant")
        };
        assert_eq!(pos, 10);

        assert_true!(serde_json::from_str::<Position>("\"unknown\"").is_err());
    }

    // Response tests
    // --------------

    const RESPONSE_NONE: LazyLock<Response> = LazyLock::new(|| Response {
        xmlns: "".into(),
        series: "f1".into(),
        url: Url::parse("https://api.jolpi.ca/ergast/f1/").unwrap(),
        pagination: Pagination {
            limit: 30,
            offset: 0,
            total: 0,
        },
        table: Table::Seasons { seasons: vec![] },
    });

    fn make_response_with_table(table: Table) -> Response {
        Response {
            table,
            ..RESPONSE_NONE.clone()
        }
    }

    const RESPONSE_SEASONS_NONE: LazyLock<Response> =
        LazyLock::new(|| make_response_with_table(Table::Seasons { seasons: vec![] }));

    const RESPONSE_SEASONS_ONE: LazyLock<Response> = LazyLock::new(|| {
        make_response_with_table(Table::Seasons {
            seasons: vec![SEASON_2000.clone()],
        })
    });

    const RESPONSE_SEASONS_TWO: LazyLock<Response> = LazyLock::new(|| {
        make_response_with_table(Table::Seasons {
            seasons: vec![SEASON_2000.clone(), SEASON_2023.clone()],
        })
    });

    const RESPONSE_DRIVERS_NONE: LazyLock<Response> =
        LazyLock::new(|| make_response_with_table(Table::Drivers { drivers: vec![] }));

    const RESPONSE_DRIVERS_ONE: LazyLock<Response> = LazyLock::new(|| {
        make_response_with_table(Table::Drivers {
            drivers: vec![DRIVER_MAX.clone()],
        })
    });

    const RESPONSE_DRIVERS_TWO: LazyLock<Response> = LazyLock::new(|| {
        make_response_with_table(Table::Drivers {
            drivers: vec![DRIVER_MAX.clone(), DRIVER_LECLERC.clone()],
        })
    });

    // Response::as_into() and .to_info()
    // ----------------------------------

    fn verify_response_info_compare<V, F>(lhs: &Response, mut rhs: Response, mut field: F, new_val: V)
    where
        V: Clone + PartialEq + std::fmt::Debug,
        F: FnMut(&mut Response) -> &mut V,
    {
        let old_val = field(&mut rhs).clone();
        assert_ne!(old_val, new_val);

        assert_eq!(lhs.as_info(), rhs.as_info());
        assert_eq!(lhs.to_info(), rhs.to_info());
        field(&mut rhs).clone_from(&new_val);
        assert_ne!(lhs.as_info(), rhs.as_info());
        assert_ne!(lhs.to_info(), rhs.to_info());

        field(&mut rhs).clone_from(&old_val);
        assert_eq!(lhs.as_info(), rhs.as_info());
        assert_eq!(lhs.to_info(), rhs.to_info());
    }

    #[test]
    fn response_as_to_info() {
        let lhs = RESPONSE_SEASONS_NONE.clone();

        let rhs_diff_pagination = Response {
            pagination: Pagination {
                limit: 30,
                offset: 10,
                total: 0,
            },
            ..lhs.clone()
        };

        let rhs_diff_table = RESPONSE_DRIVERS_NONE.clone();

        assert_eq!(lhs.as_info(), lhs.as_info());
        assert_eq!(lhs.as_info(), rhs_diff_pagination.as_info());
        assert_eq!(lhs.as_info(), rhs_diff_table.as_info());
        assert_eq!(lhs.to_info(), lhs.to_info());
        assert_eq!(lhs.to_info(), rhs_diff_pagination.to_info());
        assert_eq!(lhs.to_info(), rhs_diff_table.to_info());

        let rhs = lhs.clone();

        verify_response_info_compare(&lhs, rhs.clone(), |r| &mut r.xmlns, "other".into());
        verify_response_info_compare(&lhs, rhs.clone(), |r| &mut r.series, "f2".into());
        verify_response_info_compare(&lhs, rhs.clone(), |r| &mut r.url, Url::parse("https://example.com").unwrap());
    }

    // ::into/as_season(s)
    // -------------------

    #[test]
    fn response_into_seasons() {
        let response = RESPONSE_SEASONS_TWO.clone();
        let seasons = response.into_seasons().unwrap();
        assert_eq!(seasons.len(), 2);
        assert_eq!(seasons, vec![SEASON_2000.clone(), SEASON_2023.clone()]);
    }

    #[test]
    fn response_into_seasons_error_bad_table_variant() {
        assert!(matches!(RESPONSE_DRIVERS_NONE.clone().into_seasons(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_into_season() {
        assert_eq!(RESPONSE_SEASONS_ONE.clone().into_season().unwrap(), *SEASON_2000);
    }

    #[test]
    fn response_into_season_error_bad_table_variant() {
        assert!(matches!(RESPONSE_DRIVERS_NONE.clone().into_season(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_into_season_error_not_found() {
        assert!(matches!(RESPONSE_SEASONS_NONE.clone().into_season(), Err(Error::NotFound)));
    }

    #[test]
    fn response_into_season_error_too_many() {
        assert!(matches!(RESPONSE_SEASONS_TWO.clone().into_season(), Err(Error::TooMany)));
    }

    #[test]
    fn response_as_seasons() {
        let response = &*RESPONSE_SEASONS_TWO;
        assert_eq!(response.as_seasons().unwrap().len(), 2);
        assert_eq!(response.as_seasons().unwrap(), &vec![SEASON_2000.clone(), SEASON_2023.clone()]);
    }

    #[test]
    fn response_as_seasons_error_bad_table_variant() {
        assert!(matches!(RESPONSE_DRIVERS_NONE.as_seasons(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_as_season() {
        let response = &*RESPONSE_SEASONS_ONE;
        assert_eq!(response.as_season().unwrap(), &*SEASON_2000);
        assert_eq!(response.as_season().unwrap(), &*SEASON_2000);
    }

    #[test]
    fn response_as_season_error_bad_table_variant() {
        assert!(matches!(RESPONSE_DRIVERS_NONE.as_season(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_as_season_error_not_found() {
        assert!(matches!(RESPONSE_SEASONS_NONE.as_season(), Err(Error::NotFound)));
    }

    #[test]
    fn response_as_season_error_too_many() {
        assert!(matches!(RESPONSE_SEASONS_TWO.as_season(), Err(Error::TooMany)));
    }

    // ::into/as_driver(s)
    // -------------------

    #[test]
    fn response_into_drivers() {
        let response = RESPONSE_DRIVERS_TWO.clone();
        let drivers = response.into_drivers().unwrap();
        assert_eq!(drivers.len(), 2);
        assert_eq!(drivers, vec![DRIVER_MAX.clone(), DRIVER_LECLERC.clone()]);
    }

    #[test]
    fn response_into_drivers_error_bad_table_variant() {
        assert!(matches!(RESPONSE_NONE.clone().into_drivers(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_into_driver() {
        assert_eq!(RESPONSE_DRIVERS_ONE.clone().into_driver().unwrap(), *DRIVER_MAX);
    }

    #[test]
    fn response_into_driver_error_bad_table_variant() {
        assert!(matches!(RESPONSE_NONE.clone().into_driver(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_into_driver_error_not_found() {
        assert!(matches!(RESPONSE_DRIVERS_NONE.clone().into_driver(), Err(Error::NotFound)));
    }

    #[test]
    fn response_into_driver_error_too_many() {
        assert!(matches!(RESPONSE_DRIVERS_TWO.clone().into_driver(), Err(Error::TooMany)));
    }

    #[test]
    fn response_as_drivers() {
        let response = &*RESPONSE_DRIVERS_TWO;
        assert_eq!(response.as_drivers().unwrap().len(), 2);
        assert_eq!(response.as_drivers().unwrap(), &vec![DRIVER_MAX.clone(), DRIVER_LECLERC.clone()]);
    }

    #[test]
    fn response_as_drivers_error_bad_table_variant() {
        assert!(matches!(RESPONSE_NONE.as_drivers(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_as_driver() {
        let response = &*RESPONSE_DRIVERS_ONE;
        assert_eq!(response.as_driver().unwrap(), &*DRIVER_MAX);
        assert_eq!(response.as_driver().unwrap(), &*DRIVER_MAX);
    }

    #[test]
    fn response_as_driver_error_bad_table_variant() {
        assert!(matches!(RESPONSE_NONE.as_driver(), Err(Error::BadTableVariant)));
    }

    #[test]
    fn response_as_driver_error_not_found() {
        assert!(matches!(RESPONSE_DRIVERS_NONE.as_driver(), Err(Error::NotFound)));
    }

    #[test]
    fn response_as_driver_error_too_many() {
        assert!(matches!(RESPONSE_DRIVERS_TWO.as_driver(), Err(Error::TooMany)));
    }
}
