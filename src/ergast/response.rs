use std::convert::Infallible;

use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Deserializer, de::DeserializeOwned};
use serde_with::{DisplayFromStr, serde_as};
use url::Url;

use crate::{
    ergast::{
        resource::{Filters, Resource},
        time::{
            Date, DateTime, Duration, QualifyingTime, RaceTime, Time, deserialize_duration, deserialize_optional_time,
            deserialize_time,
        },
    },
    error::{Error, Result},
    id::{CircuitID, ConstructorID, DriverID, RoundID, SeasonID, StatusID},
};

pub const GRID_PIT_LANE: u32 = 0;

/// Represents a full JSON response from the Ergast API.
///
/// It contains metadata about the API and the response, and a single [`Table`] of data holding a
/// request-dependent variant. Note that, while [`Response`] can be deserialized from a full JSON
/// response, it actually represents the underlying `"MRData"` object, which is flattened in this
/// struct to improve ergonomics.
#[derive(PartialEq, Clone, Debug)]
pub struct Response {
    pub xmlns: String,
    pub series: String,
    pub url: Url,
    pub pagination: Pagination,
    pub table: Table,
}

impl Response {
    // TableLists
    // ----------

    /// Extracts the inner value from the corresponding [`Table`] variant for this [`TableList`].
    ///
    /// For example, [`Response::into_table_list::<Season>()`] extracts from [`Response::table`]
    /// the inner [`Vec<Season>`] of the [`Table::Seasons`] variant.
    ///
    /// Convenience aliases are provided for all implemented [`TableList`] types, e.g.
    /// [`Response::into_seasons()`] is an alias for [`Response::into_table_list::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableList`] type `T`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use f1_data::ergast::{get::get_response_max_limit, resource::{Filters, Resource}};
    /// use f1_data::ergast::response::Season;
    ///
    /// let resp = get_response_max_limit(&Resource::SeasonList(Filters::none())).unwrap();
    ///
    /// let seasons = resp.into_table_list::<Season>().unwrap();
    /// assert!(seasons.len() >= 74);
    /// assert_eq!(seasons[0].season, 1950);
    /// assert_eq!(seasons[73].season, 2023);
    /// ```
    pub fn into_table_list<T: TableList>(self) -> Result<Vec<T>> {
        T::try_into_inner_from(self.table)
    }

    /// Extracts an expected single element from the inner list for the corresponding [`Table`]
    /// variant for this [`TableList`].
    ///
    /// This method is similar to [`Response::into_table_list::<T>()`], but verifies that one and
    /// only one element is present in the extracted list, returning that element directly.For
    /// example, [`Response::into_table_list_single_element::<Season>()`] extracts from
    /// [`Response::table`] the inner [`Vec<Season>`] of the [`Table::Seasons`] variant, verifies
    /// that it contains only one element, then extracts and returns that single [`Season`].
    ///
    /// Convenience aliases are provided for all implemented [`TableList`] types, e.g.
    /// [`Response::into_season()`] is an alias for
    /// [`Response::into_table_list_single_element::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableList`] type `T`. Returns an [`Error::NotFound`] if the extracted list is
    /// empty, or an [`Error::TooMany`] if it contains more than one element.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use f1_data::ergast::{get::get_response, resource::{Filters, Resource}};
    /// use f1_data::ergast::response::Season;
    ///
    /// let resp = get_response(&Resource::SeasonList(Filters::new().season(2023))).unwrap();
    /// let season = resp.into_table_list_single_element::<Season>().unwrap();
    /// assert_eq!(season.season, 2023);
    /// assert_eq!(
    ///     season.url.as_str(),
    ///     "https://en.wikipedia.org/wiki/2023_Formula_One_World_Championship"
    /// );
    /// ```
    pub fn into_table_list_single_element<T: TableList>(self) -> Result<T> {
        self.into_table_list().and_then(verify_has_one_element_and_extract)
    }

    /// Gets a reference to the inner value from the corresponding [`Table`] variant for this
    /// [`TableList`].
    ///
    /// This method is similar to [`Response::into_table_list::<T>()`], but it returns a reference
    /// and does not consume the [`Response`]. For example, [`Response::as_table_list::<Season>()`]
    /// gets a reference to the inner [`Vec<Season>`] of the [`Table::Seasons`] variant in
    /// [`Response::table`].
    ///
    /// Convenience aliases are provided for all implemented [`TableList`] types, e.g.
    /// [`Response::as_season()`] is an alias for [`Response::as_table_list::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableList`] type `T`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use f1_data::ergast::{get::get_response_max_limit, resource::{Filters, Resource}};
    /// use f1_data::ergast::response::Season;
    ///
    /// let resp = get_response_max_limit(&Resource::SeasonList(Filters::none())).unwrap();
    ///
    /// assert!(resp.as_seasons().unwrap().len() >= 74);
    /// assert_eq!(resp.as_seasons().unwrap()[0].season, 1950);
    /// assert_eq!(resp.as_seasons().unwrap()[73].season, 2023);
    /// ```
    pub fn as_table_list<T: TableList>(&self) -> Result<&Vec<T>> {
        T::try_as_inner_from(&self.table)
    }

    /// Gets a reference to an expected single element from the inner list for the corresponding
    /// [`Table`] variant for this [`TableList`].
    ///
    /// This method is similar to [`Response::as_table_list::<T>()`], but verifies that one and
    /// only one element is present in the list, returning a reference to that element directly.
    /// It's also similar to [`Response::into_table_list_single_element::<T>()`], but it returns a
    /// reference instead of consuming the [`Response`]. For example,
    /// [`Response::as_table_list_single_element::<Season>()`] gets a reference to the
    /// [`Vec<Season>`] from the [`Table::Seasons`] variant in [`Response::table`], verifies that it
    /// contains only one element, then returns a reference to that single [`Season`].
    ///
    /// Convenience aliases are provided for all implemented [`TableList`] types, e.g.
    /// [`Response::as_season()`] is an alias for
    /// [`Response::as_table_list_single_element::<Season>()`].
    ///
    /// # Errors
    ///
    /// Returns an [`Error::BadTableVariant`] if the contained [`Table`] variant does not match the
    /// requested [`TableList`] type `T`. Returns an [`Error::NotFound`] if the extracted list is
    /// empty, or an [`Error::TooMany`] if it contains more than one element.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use f1_data::ergast::{get::get_response, resource::{Filters, Resource}};
    /// use f1_data::ergast::response::Season;
    ///
    /// let resp = get_response(&Resource::SeasonList(Filters::new().season(2023))).unwrap();
    /// assert_eq!(resp.as_season().unwrap().season, 2023);
    /// assert_eq!(
    ///     resp.as_season().unwrap().url.as_str(),
    ///     "https://en.wikipedia.org/wiki/2023_Formula_One_World_Championship"
    /// );
    /// ```
    pub fn as_table_list_single_element<T: TableList>(&self) -> Result<&T> {
        self.as_table_list()
            .map(Vec::as_slice)
            .and_then(verify_has_one_element)
            .map(|s| s.first().unwrap())
    }

    // Races and SessionResults
    // ------------------------

    pub fn into_race_schedules(self) -> Result<Vec<Race<Schedule>>> {
        self.table
            .into_races()?
            .into_iter()
            .map(|race| race.try_map(|payload| payload.into_schedule().map_err(into)))
            .collect()
    }

    pub fn into_session_results<T: SessionResult>(self) -> Result<Vec<Race<Vec<T>>>> {
        self.table
            .into_races()?
            .into_iter()
            .map(|race| race.try_map(|payload| T::try_into_inner_from(payload)))
            .collect()
    }

    pub fn into_session_result_for_events<T: SessionResult>(self) -> Result<Vec<Race<T>>> {
        self.into_session_results()?
            .into_iter()
            .map(|race| race.try_map(verify_has_one_element_and_extract))
            .collect()
    }

    // Laps, Timings, PitStops
    // -----------------------

    pub fn into_driver_laps(self, driver_id: &DriverID) -> Result<Vec<DriverLap>> {
        Ok(self)
            .and_then(verify_has_one_race_and_extract)?
            .payload
            .into_laps()
            .map_err(into)
            .map(into_iter)
            .and_then(|laps| laps.map(|lap| DriverLap::try_from(lap, driver_id)).collect())
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

    // Convenience aliases for into/as_table_list(s)::<T> and into/as_table_list_single_element::<T>
    // Aliases implemented for TableList types: Seasons, Drivers, Constructors, Circuits, Statuses
    // ---------------------------------------------------------------------------------------------

    /// Convenience alias for `into_table_list::<Season>()`.
    pub fn into_seasons(self) -> Result<Vec<Season>> {
        self.into_table_list::<Season>()
    }

    /// Convenience alias for `into_table_list_single_element::<Season>()`.
    pub fn into_season(self) -> Result<Season> {
        self.into_table_list_single_element::<Season>()
    }

    /// Convenience alias for `as_table_list::<Season>()`.
    pub fn as_seasons(&self) -> Result<&Vec<Season>> {
        self.as_table_list::<Season>()
    }

    /// Convenience alias for `as_table_list_single_element::<Season>()`.
    pub fn as_season(&self) -> Result<&Season> {
        self.as_table_list_single_element::<Season>()
    }

    /// Convenience alias for `into_table_list::<Driver>()`.
    pub fn into_drivers(self) -> Result<Vec<Driver>> {
        self.into_table_list::<Driver>()
    }

    /// Convenience alias for `into_table_list_single_element::<Driver>()`.
    pub fn into_driver(self) -> Result<Driver> {
        self.into_table_list_single_element::<Driver>()
    }

    /// Convenience alias for `as_table_list::<Driver>()`.
    pub fn as_drivers(&self) -> Result<&Vec<Driver>> {
        self.as_table_list::<Driver>()
    }

    /// Convenience alias for `as_table_list_single_element::<Driver>()`.
    pub fn as_driver(&self) -> Result<&Driver> {
        self.as_table_list_single_element::<Driver>()
    }

    /// Convenience alias for `into_table_list::<Constructor>()`.
    pub fn into_constructors(self) -> Result<Vec<Constructor>> {
        self.into_table_list::<Constructor>()
    }

    /// Convenience alias for `into_table_list_single_element::<Constructor>()`.
    pub fn into_constructor(self) -> Result<Constructor> {
        self.into_table_list_single_element::<Constructor>()
    }

    /// Convenience alias for `as_table_list::<Constructor>()`.
    pub fn as_constructors(&self) -> Result<&Vec<Constructor>> {
        self.as_table_list::<Constructor>()
    }

    /// Convenience alias for `as_table_list_single_element::<Constructor>()`.
    pub fn as_constructor(&self) -> Result<&Constructor> {
        self.as_table_list_single_element::<Constructor>()
    }

    /// Convenience alias for `as_table_list::<Constructor>()`.
    pub fn into_circuits(self) -> Result<Vec<Circuit>> {
        self.into_table_list::<Circuit>()
    }

    /// Convenience alias for `into_table_list_single_element::<Constructor>()`.
    pub fn into_circuit(self) -> Result<Circuit> {
        self.into_table_list_single_element::<Circuit>()
    }

    /// Convenience alias for `as_table_list::<Constructor>()`.
    pub fn as_circuits(&self) -> Result<&Vec<Circuit>> {
        self.as_table_list::<Circuit>()
    }

    /// Convenience alias for `as_table_list_single_element::<Constructor>()`.
    pub fn as_circuit(&self) -> Result<&Circuit> {
        self.as_table_list_single_element::<Circuit>()
    }

    /// Convenience alias for `as_table_list::<Constructor>()`.
    pub fn into_statuses(self) -> Result<Vec<Status>> {
        self.into_table_list::<Status>()
    }

    /// Convenience alias for `into_table_list_single_element::<Constructor>()`.
    pub fn into_status(self) -> Result<Status> {
        self.into_table_list_single_element::<Status>()
    }

    /// Convenience alias for `as_table_list::<Constructor>()`.
    pub fn as_statuses(&self) -> Result<&Vec<Status>> {
        self.as_table_list::<Status>()
    }

    /// Convenience alias for `as_table_list_single_element::<Constructor>()`.
    pub fn as_status(&self) -> Result<&Status> {
        self.as_table_list_single_element::<Status>()
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

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Pagination {
    #[serde_as(as = "DisplayFromStr")]
    pub limit: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub offset: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub total: u32,
}

impl Pagination {
    pub fn is_last_page(&self) -> bool {
        (self.offset + self.limit) >= self.total
    }

    pub fn is_single_page(&self) -> bool {
        (self.offset == 0) && self.is_last_page()
    }

    pub fn next_page(&self) -> Option<Self> {
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

/// Represents all the possible different lists of data that may be returned in a
/// [`Response`] from the Ergast API.
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
/// use f1_data::ergast::response::{Season, Table};
///
/// let table = Table::Seasons {
///     seasons: vec![Season {
///         season: 2022,
///         url: Url::parse("http://empty.org").unwrap(),
///     }],
/// };
///
/// let Table::Seasons { ref seasons } = table else { panic!("Expected Seasons variant") };
/// assert_eq!(seasons[0].season, 2022);
///
/// assert_eq!(table.as_seasons().unwrap()[0].season, 2022);
/// ```
#[derive(Deserialize, EnumAsInner, PartialEq, Clone, Debug)]
pub enum Table {
    /// Contains a list of [`Season`]s, and corresponds to the `"SeasonTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "SeasonTable")]
    Seasons {
        /// List of [`Season`]s, corresponding to the `"Seasons"` property key in the JSON response.
        #[serde(rename = "Seasons")]
        seasons: Vec<Season>,
    },
    /// Contains a list of [`Driver`]s, and corresponds to the `"DriverTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "DriverTable")]
    Drivers {
        /// List of [`Driver`]s, corresponding to the `"Drivers"` property key in the JSON response.
        #[serde(rename = "Drivers")]
        drivers: Vec<Driver>,
    },
    /// Contains a list of [`Constructor`]s, and corresponds to the `"ConstructorTable"` property
    /// key in the JSON response from the Ergast API.
    #[serde(rename = "ConstructorTable")]
    Constructors {
        /// List of [`Constructor`]s, corresponding to the `"Constructors"` property key in the JSON
        /// response.
        #[serde(rename = "Constructors")]
        constructors: Vec<Constructor>,
    },
    /// Contains a list of [`Circuit`]s, and corresponds to the `"CircuitTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "CircuitTable")]
    Circuits {
        /// List of [`Circuit`]s, corresponding to the `"Circuits"` property key in the JSON
        /// response.
        #[serde(rename = "Circuits")]
        circuits: Vec<Circuit>,
    },
    /// Contains a list of [`Race`]s, and corresponds to the `"RaceTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "RaceTable")]
    Races {
        /// List of [`Race`]s, corresponding to the `"Races"` property key in the JSON response.
        #[serde(rename = "Races")]
        races: Vec<Race>,
    },
    /// Contains a list of [`Status`]es, and corresponds to the `"StatusTable"` property key in the
    /// JSON response from the Ergast API.
    #[serde(rename = "StatusTable")]
    Status {
        /// List of [`Status`]es, corresponding to the `"Status"` property key in the JSON response.
        #[serde(rename = "Status")]
        status: Vec<Status>,
    },
}

/// Inner type of a [`Payload`] variant for a [`SessionResult`] type, and of a [`Table`] variant
/// for a [`TableList`] type.
///
/// For example, the inner type of the [`Payload::RaceResults`] variant is [`Vec<RaceResult>`], and
/// the inner type of the [`Table::Seasons`] variant is [`Vec<Season>`].
type Inner<T> = Vec<T>;

/// The [`TableList`] trait allows the generic handling of all [`Table`] list inner types,
/// associated [`Resource`] requests, and the extraction of the corresponding variants.
///
/// For example, [`Season`]s, which are requested via [`Resource::SeasonList`], can be extracted
/// from a [`Response`] via [`Response::table`], from the [`Table::Seasons`] variant.
///
/// The trait is implemented for [`Season`], [`Driver`], [`Constructor`], [`Circuit`], [`Status`].
pub trait TableList
where
    Self: Sized,
{
    /// The type of the [`Filters`] ID for this [`TableList`], e.g. [`SeasonID`] for [`Season`].
    type ID;

    /// Wrap a [`Filters`] with the corresponding [`Resource`] variant for this [`TableList`],
    /// e.g. [`Resource::SeasonList`] for [`Season`].
    fn to_resource(filters: Filters) -> Resource;

    /// Wrap a [`Filters`] with the corresponding ID filter and [`Resource`] for this [`TableList`],
    /// e.g. a [`Filters::season`] filter for [`Season`], to be passed to [`Resource::SeasonList`].
    fn to_resource_with_id_filter(id: Self::ID) -> Resource;

    /// Extract the inner value from the corresponding [`Table`] variant for this [`TableList`],
    /// e.g. a [`Vec<Season>`] from the [`Table::Seasons`] variant for [`Season`].
    fn try_into_inner_from(table: Table) -> Result<Inner<Self>>;

    /// Get a reference to the inner value from the corresponding [`Table`] variant for this
    /// [`TableList`], e.g. a <code>&[`Vec<Season>`]</code> from the [`Table::Seasons`] variant.
    fn try_as_inner_from(table: &Table) -> Result<&Inner<Self>>;
}

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Season {
    #[serde_as(as = "DisplayFromStr")]
    pub season: SeasonID,
    pub url: Url,
}

impl TableList for Season {
    type ID = SeasonID;

    fn to_resource(filters: Filters) -> Resource {
        Resource::SeasonList(filters)
    }

    fn to_resource_with_id_filter(season: Self::ID) -> Resource {
        Self::to_resource(Filters::new().season(season))
    }

    fn try_into_inner_from(table: Table) -> Result<Inner<Self>> {
        table.into_seasons().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&Inner<Self>> {
        table.as_seasons().ok_or(Error::BadTableVariant)
    }
}

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Driver {
    pub driver_id: DriverID,
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub permanent_number: Option<u32>,
    pub code: Option<String>,
    pub url: Url,
    pub given_name: String,
    pub family_name: String,
    pub date_of_birth: Date,
    pub nationality: String,
}

impl TableList for Driver {
    type ID = DriverID;

    fn to_resource(filters: Filters) -> Resource {
        Resource::DriverInfo(filters)
    }

    fn to_resource_with_id_filter(id: Self::ID) -> Resource {
        Self::to_resource(Filters::new().driver_id(id))
    }

    fn try_into_inner_from(table: Table) -> Result<Inner<Self>> {
        table.into_drivers().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&Inner<Self>> {
        table.as_drivers().ok_or(Error::BadTableVariant)
    }
}

#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Constructor {
    pub constructor_id: ConstructorID,
    pub url: Url,
    pub name: String,
    pub nationality: String,
}

impl TableList for Constructor {
    type ID = ConstructorID;

    fn to_resource(filters: Filters) -> Resource {
        Resource::ConstructorInfo(filters)
    }

    fn to_resource_with_id_filter(id: Self::ID) -> Resource {
        Self::to_resource(Filters::new().constructor_id(id))
    }

    fn try_into_inner_from(table: Table) -> Result<Inner<Self>> {
        table.into_constructors().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&Inner<Self>> {
        table.as_constructors().ok_or(Error::BadTableVariant)
    }
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Circuit {
    pub circuit_id: CircuitID,
    pub url: Url,
    pub circuit_name: String,
    #[serde(rename = "Location")]
    pub location: Location,
}

impl TableList for Circuit {
    type ID = CircuitID;

    fn to_resource(filters: Filters) -> Resource {
        Resource::CircuitInfo(filters)
    }

    fn to_resource_with_id_filter(id: Self::ID) -> Resource {
        Self::to_resource(Filters::new().circuit_id(id))
    }

    fn try_into_inner_from(table: Table) -> Result<Inner<Self>> {
        table.into_circuits().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&Inner<Self>> {
        table.as_circuits().ok_or(Error::BadTableVariant)
    }
}

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    #[serde_as(as = "DisplayFromStr")]
    pub status_id: StatusID,
    #[serde_as(as = "DisplayFromStr")]
    pub count: u32,
    pub status: String,
}

impl TableList for Status {
    type ID = StatusID;

    fn to_resource(filters: Filters) -> Resource {
        Resource::FinishingStatus(filters)
    }

    fn to_resource_with_id_filter(id: Self::ID) -> Resource {
        Self::to_resource(Filters::new().finishing_status(id))
    }

    fn try_into_inner_from(table: Table) -> Result<Inner<Self>> {
        table.into_status().map_err(into)
    }

    fn try_as_inner_from(table: &Table) -> Result<&Inner<Self>> {
        table.as_status().ok_or(Error::BadTableVariant)
    }
}

/// This generic struct represents a race weekend event, corresponding to the list element type
/// under the `"RaceTable.Races"` property key in the JSON response from the Ergast API. The generic
/// type parameter `T` represents the type of payload that may be returned, depending on the
/// requested [`Resource`]. The default <code>T = [Payload]</code> accepts all possible payload
/// types, but the `T` parameter may be specified during postprocessing to restrict the payload
/// type, e.g. by `get_*` API functions that know the expected payload variant.
#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Race<T = Payload> {
    #[serde_as(as = "DisplayFromStr")]
    pub season: SeasonID,
    #[serde_as(as = "DisplayFromStr")]
    pub round: RoundID,
    pub url: Url,
    pub race_name: String,
    #[serde(rename = "Circuit")]
    pub circuit: Circuit,
    pub date: Date,
    #[serde(default, deserialize_with = "deserialize_optional_time")]
    pub time: Option<Time>,
    #[serde(flatten)]
    pub payload: T,
}

impl<T> Race<T> {
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

/// Compares two [`Race`]s for equality, ignoring the payload.
// @todo If a new field is added to [`Race`], and this implementation isn't updated accordingly,
// it will silently fail - unit tests won't catch it. I haven't figured out a way to solve this
// problem without a lot of inefficient cloning to discard payload and compare [`Race<Void>`]s.
pub fn eq_race_info<T, U>(lhs: &Race<T>, rhs: &Race<U>) -> bool {
    (lhs.season == rhs.season)
        && (lhs.round == rhs.round)
        && (lhs.url == rhs.url)
        && (lhs.race_name == rhs.race_name)
        && (lhs.circuit == rhs.circuit)
        && (lhs.date == rhs.date)
        && (lhs.time == rhs.time)
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Schedule {
    #[serde(rename = "FirstPractice")]
    pub first_practice: Option<DateTime>,
    #[serde(rename = "SecondPractice")]
    pub second_practice: Option<DateTime>,
    #[serde(rename = "ThirdPractice")]
    pub third_practice: Option<DateTime>,
    #[serde(rename = "Qualifying")]
    pub qualifying: Option<DateTime>,
    #[serde(rename = "Sprint")]
    pub sprint: Option<DateTime>,
}

impl Race<Schedule> {
    /// Returns a reference to the field [`Race::payload`], a [`Schedule`].
    pub fn schedule(&self) -> &Schedule {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a [`Schedule`].
    pub fn into_schedule(self) -> Schedule {
        self.payload
    }
}

/// [`Payload`] represents all the possible different data elements that be me returned as part of
/// a [`Race`] in a [`Response`] from the Ergast API.
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
/// use f1_data::ergast::response::{Payload, SprintResult};
///
/// let payload = Payload::Laps(vec![]);
///
/// let Payload::Laps(laps) = &payload else { panic!("Expected Laps variant"); };
/// assert!(laps.is_empty());
///
/// assert!(payload.as_laps().unwrap().is_empty());
/// ```
#[derive(EnumAsInner, PartialEq, Clone, Debug)]
pub enum Payload {
    /// Contains a list of [`QualifyingResult`]s, and corresponds to the `"QualifyingResults"`
    /// property key in the JSON response from the Ergast API.
    QualifyingResults(Vec<QualifyingResult>),

    /// Contains a list of [`SprintResult`]s, and corresponds to the `"SprintResults"` property key
    /// in the JSON response from the Ergast API.
    SprintResults(Vec<SprintResult>),

    /// Contains a list of [`RaceResult`]s, and corresponds to the `"Results"` property key in the
    /// JSON response from the Ergast API.
    RaceResults(Vec<RaceResult>),

    /// Contains a list of [`Lap`]s, and corresponds to the `"Laps"` property key in the JSON
    /// response from the Ergast API.
    Laps(Vec<Lap>),

    /// Contains a list of [`PitStop`]s, and corresponds to the `"PitStops"` property key in the
    /// JSON response from the Ergast API.
    PitStops(Vec<PitStop>),

    /// Contains a [`Schedule`] object, and corresponds to the absence of a tag property key in the
    /// JSON response from the Ergast API. That is, all the elements of a schedule are flattened
    /// directly into the [`Race`] object in JSON.
    ///
    /// **Note:** Because of the untagged nature of this variant, and because all of the fields of
    /// [`Schedule`] are optional, it no payload is returned this variant will be the one being set.
    /// This is also a valid response from the Ergast API, e.g. for races prior to 2022, where
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

/// The [`SessionResult`] trait allows the generic handling of all [`Payload`] session result inner
/// types, associated [`Resource`] requests, and extraction of the corresponding variants.
///
/// For example, [`RaceResult`]s are requested via [`Resource::RaceResults`], and the response
/// [`Vec<RaceResult>`] can be extracted from the [`Payload::RaceResults`] variant.
///
/// The trait is implemented for [`QualifyingResult`], [`SprintResult`], and [`RaceResult`].
pub trait SessionResult
where
    Self: Sized,
{
    /// Wrap a [`Filters`] with the corresponding [`Resource`] variant for this [`SessionResult`],
    /// e.g. [`Resource::RaceResults`] for [`RaceResult`].
    fn to_resource(filters: Filters) -> Resource;

    /// Extract the inner value from the corresponding [`Payload`] variant for this
    /// [`SessionResult`], e.g. a [`Vec<RaceResult>`] from the [`Payload::RaceResults`] variant.
    fn try_into_inner_from(payload: Payload) -> Result<Inner<Self>>;
}

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct QualifyingResult {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde(rename = "Q1")]
    pub q1: Option<QualifyingTime>,
    #[serde(rename = "Q2")]
    pub q2: Option<QualifyingTime>,
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
    pub fn qualifying_result(&self) -> &QualifyingResult {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a single [`QualifyingResult`].
    pub fn into_qualifying_result(self) -> QualifyingResult {
        self.payload
    }
}

impl SessionResult for QualifyingResult {
    fn to_resource(filters: Filters) -> Resource {
        Resource::QualifyingResults(filters)
    }

    fn try_into_inner_from(payload: Payload) -> Result<Inner<Self>> {
        payload.into_qualifying_results().map_err(into)
    }
}

/// Represents points awarded, e.g. for a sprint/race finish, fastest lap, etc.
///
/// These are represented as floating point because some events may award fractional points, e.g.
/// the 2021 Belgian GP only awarded half points, meaning P1, P3, and P10 received `x.5` points.
pub type Points = f32;

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SprintResult {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    pub position_text: Position,
    #[serde_as(as = "DisplayFromStr")]
    pub points: Points,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde_as(as = "DisplayFromStr")]
    pub grid: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub laps: u32,
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<RaceTime>,
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
    pub fn sprint_result(&self) -> &SprintResult {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a single [`SprintResult`].
    pub fn into_sprint_result(self) -> SprintResult {
        self.payload
    }
}

impl SessionResult for SprintResult {
    fn to_resource(filters: Filters) -> Resource {
        Resource::SprintResults(filters)
    }

    fn try_into_inner_from(payload: Payload) -> Result<Inner<Self>> {
        payload.into_sprint_results().map_err(into)
    }
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RaceResult {
    #[serde(deserialize_with = "deserialize_possible_no_number")]
    pub number: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    pub position_text: Position,
    #[serde_as(as = "DisplayFromStr")]
    pub points: Points,
    #[serde(rename = "Driver")]
    pub driver: Driver,
    #[serde(rename = "Constructor")]
    pub constructor: Constructor,
    #[serde_as(as = "DisplayFromStr")]
    pub grid: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub laps: u32,
    pub status: String,
    #[serde(rename = "Time")]
    pub time: Option<RaceTime>,
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
    pub fn race_result(&self) -> &RaceResult {
        &self.payload
    }

    /// Extracts and returns the field [`Race::payload`], a single [`RaceResult`].
    pub fn into_race_result(self) -> RaceResult {
        self.payload
    }
}

impl SessionResult for RaceResult {
    fn to_resource(filters: Filters) -> Resource {
        Resource::RaceResults(filters)
    }

    fn try_into_inner_from(payload: Payload) -> Result<Inner<Self>> {
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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Position {
    Finished(u32),
    Retired,
    Disqualified,
    Excluded,
    Withdrawn,
    FailedToQualify,
    NotClassified,
}

impl Position {
    pub const R: Self = Self::Retired;
    pub const D: Self = Self::Disqualified;
    pub const E: Self = Self::Excluded;
    pub const W: Self = Self::Withdrawn;
    pub const F: Self = Self::FailedToQualify;
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

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct Lap {
    #[serde_as(as = "DisplayFromStr")]
    pub number: u32,
    #[serde(rename = "Timings")]
    pub timings: Vec<Timing>,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Timing {
    pub driver_id: DriverID,
    #[serde_as(as = "DisplayFromStr")]
    pub position: u32,
    #[serde(deserialize_with = "deserialize_duration")]
    pub time: Duration,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PitStop {
    pub driver_id: DriverID,
    #[serde_as(as = "DisplayFromStr")]
    pub lap: u32,
    #[serde_as(as = "DisplayFromStr")]
    pub stop: u32,
    #[serde(deserialize_with = "deserialize_time")]
    pub time: Time,
    #[serde(deserialize_with = "deserialize_duration")]
    pub duration: Duration,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct Location {
    #[serde_as(as = "DisplayFromStr")]
    pub lat: f64,
    #[serde_as(as = "DisplayFromStr")]
    pub long: f64,
    pub locality: String,
    pub country: String,
}

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct FastestLap {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub rank: Option<u32>,
    #[serde_as(as = "DisplayFromStr")]
    pub lap: u32,
    #[serde(rename = "Time", deserialize_with = "extract_nested_time")]
    pub time: Duration,
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

#[serde_as]
#[derive(Deserialize, PartialEq, Clone, Copy, Debug)]
pub struct AverageSpeed {
    pub units: SpeedUnits,
    #[serde_as(as = "DisplayFromStr")]
    pub speed: f32,
}

#[derive(Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub enum SpeedUnits {
    #[serde(rename = "kph")]
    Kph,
}

/// Check that there is exactly one element `T` in a slice `&[T]`, and return a
/// <code>[Result<&\[T\]>]</code> containing the slice if so, [`Error::NotFound`] if it contained no
/// elements, or [`Error::TooMany`] if it contained more than one.
pub(crate) fn verify_has_one_element<T>(sequence: &[T]) -> Result<&[T]> {
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

/// Shorthand for closure `|v| v.into_iter()` and/or `std::iter::IntoIterator::into_iter`.
// @todo Replace with an import once `import_trait_associated_functions` is stabilized:
// https://doc.rust-lang.org/nightly/unstable-book/language-features/import-trait-associated-functions.html
fn into_iter<T: IntoIterator>(t: T) -> T::IntoIter {
    t.into_iter()
}

#[cfg(test)]
mod tests {
    use const_format::formatcp;
    use pretty_assertions::{assert_eq, assert_ne};
    use std::sync::LazyLock;

    use super::*;
    use crate::ergast::tests::assets::*;

    #[test]
    fn season_table() {
        let table: Table = serde_json::from_str(SEASON_TABLE_STR).unwrap();
        assert!(!table.as_seasons().unwrap().is_empty());
        assert_eq!(table, *SEASON_TABLE);
    }

    #[test]
    fn driver_table() {
        let table: Table = serde_json::from_str(DRIVER_TABLE_STR).unwrap();
        assert!(!table.as_drivers().unwrap().is_empty());
        assert_eq!(table, *DRIVER_TABLE);
    }

    #[test]
    fn constructor_table() {
        let table: Table = serde_json::from_str(CONSTRUCTOR_TABLE_STR).unwrap();
        assert!(!table.as_constructors().unwrap().is_empty());
        assert_eq!(table, *CONSTRUCTOR_TABLE);
    }

    #[test]
    fn circuit_table() {
        let table: Table = serde_json::from_str(CIRCUIT_TABLE_STR).unwrap();
        assert!(!table.as_circuits().unwrap().is_empty());
        assert_eq!(table, *CIRCUIT_TABLE);
    }

    #[test]
    fn race_table_schedule() {
        let table: Table = serde_json::from_str(RACE_TABLE_SCHEDULE_STR).unwrap();
        assert!(!table.as_races().unwrap().is_empty());
        assert_eq!(table, *RACE_TABLE_SCHEDULE);
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
            assert!(!race.payload.as_qualifying_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2003_4_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_4_QUALIFYING_RESULTS_STR).unwrap();
            assert!(!race.payload.as_qualifying_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2023_4_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_10_QUALIFYING_RESULTS_STR).unwrap();
            assert!(!race.payload.as_qualifying_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2023_10_QUALIFYING_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_12_QUALIFYING_RESULTS_STR).unwrap();
            assert!(!race.payload.as_qualifying_results().unwrap().is_empty());
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
        assert!(!race.payload.as_sprint_results().unwrap().is_empty());
        assert_eq!(race, *RACE_2023_4_SPRINT_RESULTS);
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
        {
            let race: Race = serde_json::from_str(RACE_1963_10_RACE_RESULTS_STR).unwrap();
            assert!(!race.payload.as_race_results().unwrap().is_empty());
            assert_eq!(race, *RACE_1963_10_RACE_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2003_4_RACE_RESULTS_STR).unwrap();
            assert!(!race.payload.as_race_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2003_4_RACE_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2021_12_RACE_RESULTS_STR).unwrap();
            assert!(!race.payload.as_race_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2021_12_RACE_RESULTS);
        }

        {
            let race: Race = serde_json::from_str(RACE_2023_4_RACE_RESULTS_STR).unwrap();
            assert!(!race.payload.as_race_results().unwrap().is_empty());
            assert_eq!(race, *RACE_2023_4_RACE_RESULTS);
        }
    }

    #[test]
    fn finishing_status() {
        let table: Table = serde_json::from_str(STATUS_TABLE_2022_STR).unwrap();
        assert!(!table.as_status().unwrap().is_empty());
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
        assert!(!laps.is_empty());
        laps.iter().for_each(|lap| assert!(!lap.timings.is_empty()));

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
        assert!(!race.payload.as_pit_stops().unwrap().is_empty());
        assert_eq!(race, *RACE_2023_4_PIT_STOPS);
    }

    #[test]
    fn pagination_is_last_page() {
        assert!(
            Pagination {
                limit: 30,
                offset: 0,
                total: 16
            }
            .is_last_page()
        );

        assert!(
            Pagination {
                limit: 10,
                offset: 5,
                total: 15
            }
            .is_last_page()
        );

        assert!(
            !Pagination {
                limit: 10,
                offset: 4,
                total: 15
            }
            .is_last_page()
        );
    }

    #[test]
    fn pagination_is_single_page() {
        assert!(
            Pagination {
                limit: 30,
                offset: 0,
                total: 16
            }
            .is_single_page()
        );

        assert!(
            !Pagination {
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

        assert!(
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

    fn verify_eq_race_info<T, U, V, F>(lhs: &Race<T>, mut rhs: Race<U>, mut field: F, new_val: V)
    where
        V: Clone + PartialEq + std::fmt::Debug,
        F: FnMut(&mut Race<U>) -> &mut V,
    {
        let old_val = field(&mut rhs).clone();
        assert_ne!(old_val, new_val);

        assert!(eq_race_info(lhs, &rhs));
        field(&mut rhs).clone_from(&new_val);
        assert!(!eq_race_info(lhs, &rhs));

        field(&mut rhs).clone_from(&old_val);
        assert!(eq_race_info(lhs, &rhs));
    }

    #[test]
    fn race_eq_race_info() {
        let lhs = RACE_2023_4.clone();

        assert!(eq_race_info(&lhs, &lhs));
        assert!(eq_race_info(&lhs, &Race::from(lhs.clone(), true)));

        let rhs = lhs.clone();

        verify_eq_race_info(&lhs, rhs.clone(), |r| &mut r.season, RACE_NONE.season);
        verify_eq_race_info(&lhs, rhs.clone(), |r| &mut r.round, RACE_NONE.round);
        verify_eq_race_info(&lhs, rhs.clone(), |r| &mut r.url, RACE_NONE.url.clone());
        verify_eq_race_info(&lhs, rhs.clone(), |r| &mut r.race_name, RACE_NONE.race_name.clone());
        verify_eq_race_info(&lhs, rhs.clone(), |r| &mut r.circuit, RACE_NONE.circuit.clone());
        verify_eq_race_info(&lhs, rhs.clone(), |r| &mut r.date, RACE_NONE.date);
        verify_eq_race_info(&lhs, rhs.clone(), |r| &mut r.time, RACE_NONE.time);
    }

    #[test]
    fn race_try_map() {
        let from = Race::from(RACE_2023_4.clone(), true);

        let into = from.clone().try_map::<_, _, Infallible>(|_| Ok(String::from("true")));
        assert!(eq_race_info(&into.as_ref().unwrap(), &from));
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
        assert!(into.is_err());
    }

    #[test]
    fn race_map() {
        let from = Race::from(RACE_2023_4.clone(), 1);

        let into = from.clone().map(|payload_i32| payload_i32.to_string());
        assert!(eq_race_info(&into, &from));
        assert_eq!(into.payload, String::from("1"));
    }

    #[test]
    fn race_from() {
        let from = RACE_2023_4.clone();

        let into = Race::from(from.clone(), String::from("some"));
        assert!(eq_race_info(&into, &from));
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
        assert!(p.unwrap_err().to_string().contains("missing field `number`"));
    }

    #[test]
    fn race_schedule_accessors() {
        let reference = RACE_2023_4_SCHEDULE.clone();
        let expected = reference.clone().payload.into_schedule().unwrap();

        let actual = map_race_schedule(reference.clone());
        assert_eq!(actual.schedule(), &expected);
        assert_eq!(actual.into_schedule(), expected);
    }

    fn map_race_multi_results<T: SessionResult>(race: Race<Payload>) -> Race<Vec<T>> {
        race.map(|payload| T::try_into_inner_from(payload).unwrap())
    }

    fn map_race_single_result<T: SessionResult>(race: Race<Payload>) -> Race<T> {
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
        assert!(!matches!(pos, Position::E));

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

        assert!(serde_json::from_str::<Position>("\"unknown\"").is_err());
    }

    // Response::into_* and Response::as_* tests
    // -----------------------------------------

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
