use bitflags::bitflags;
use url::Url;

use crate::{
    error::{Error, Result},
    id::{RoundID, SeasonID},
    jolpica::{
        response::{Circuit, Pagination, Payload, Race, Response, Table},
        time::{Date, Time},
    },
};

#[cfg(doc)]
use crate::jolpica::{get::get_response_multi_pages, resource::Page};

bitflags! {
/// Bitflags to control verification of response pages when concatenating multi-page responses.
#[derive(Copy, Clone, Debug)]
pub struct PageVerify: u32 {
    /// Do not perform any verification of the response pages.
    const NONE = 0b0000;
    /// Verify that the response pages are contiguous, i.e. that there are no gaps or overlaps.
    const CONTIGUOUS = 0b0001;
    /// Verify that the first response page starts at offset 0.
    const START_AT_FIRST_PAGE = 0b0010;
    /// Verify that the last response page is the last page, i.e. `offset + limit >= total`.
    const FINISH_AT_LAST_PAGE = 0b0100;
    /// Perform all verifications above.
    const ALL = Self::CONTIGUOUS.bits() | Self::START_AT_FIRST_PAGE.bits() | Self::FINISH_AT_LAST_PAGE.bits();
}
}

/// Concatenate a sequence of [`Response`]s representing pages of a multi-page response into a
/// single [`Response`], concatenating underlying [`Table`]s, [`Payload`]s, [`Race`]s, etc.
///
/// `page_verify` controls the verification of the pagination sequence, see [`PageVerify`] for
/// details. If [`PageVerify::CONTIGUOUS`] is set, the [`Response::pagination`] field will be
/// updated to reflect the concatenated [`Pagination::limit`], otherwise it will be left as-is from
/// the first [`Response`] in the sequence. A sequence returned by [`get_response_multi_pages`]
/// should satisfy [`PageVerify::ALL`] if requested with a [`Page`] starting at offset 0.
///
/// All [`Table`] variants, except [`Table::Races`], are concatenated by simply concatenating the
/// underlying [`Vec<T>`]s, e.g. [`Vec<Season>`]s for [`Table::Seasons`]. For [`Table::Races`],
/// the underlying [`Race`]s are first grouped by their [`Race::as_info`], and then their underlying
/// [`Payload`]s are concatenated, e.g. [`Vec<RaceResult>`]s for [`Payload::RaceResults`], etc.
///
/// # Errors
///
/// If `responses` is empty, an [`Error::EmptyResponseList`] is returned. If all
/// [`Response::as_info`] do not match, an [`Error::BadResponseInfo`] is returned. If the [`Table`]
/// variants do not match, an [`Error::BadTableVariant`] is returned. If the [`Payload`] variants of
/// all [`Race`]s with the same [`Race::as_info`] do not match, an [`Error::BadPayloadVariant`] is
/// returned. If any of the verification specified by `page_verify` fail, an
/// [`Error::BadPagination`] is returned.
///
/// # Examples
///
/// ```no_run
/// # use f1_data::{
/// #     jolpica::{
/// #         api::JOLPICA_API_BASE_URL,
/// #         get::get_response_multi_pages,
/// #         resource::{Filters, Resource},
/// #         concat::{concat_response_multi_pages, PageVerify},
/// #     }};
/// let responses = get_response_multi_pages(
///     JOLPICA_API_BASE_URL,
///     &Resource::SeasonList(Filters::none()),
///     None,
///     None,
///     None,
/// )
/// .unwrap();
///
/// assert_eq!(responses.len(), 3); // 76 / 30 -> 3 pages
/// assert_eq!(responses[0].as_seasons().unwrap().len(), 30);
/// assert_eq!(responses[1].as_seasons().unwrap().len(), 30);
/// assert_eq!(responses[2].as_seasons().unwrap().len(), 16);
///
/// let response = concat_response_multi_pages(responses, PageVerify::ALL).unwrap();
/// assert_eq!(response.as_seasons().unwrap().len(), 76);
/// assert!(response.pagination.is_last_page());
/// assert_eq!(response.pagination.offset, 0);
/// assert_eq!(response.pagination.limit, 90);
/// assert_eq!(response.pagination.total, 76);
///
/// assert_eq!(response.as_seasons().unwrap()[0].season, 1950);
/// assert_eq!(response.as_seasons().unwrap()[75].season, 2025);
/// ```
pub fn concat_response_multi_pages(mut responses: Vec<Response>, page_verify: PageVerify) -> Result<Response> {
    if responses.is_empty() {
        return Err(Error::EmptyResponseList);
    }

    let mut lhs_resp = responses.remove(0);

    if page_verify.contains(PageVerify::START_AT_FIRST_PAGE) && lhs_resp.pagination.offset != 0 {
        return Err(Error::BadPagination(format!(
            "First response page offset {} is not 0",
            lhs_resp.pagination.offset
        )));
    }

    if let Some(last_page) = responses.last()
        && page_verify.contains(PageVerify::FINISH_AT_LAST_PAGE)
        && !last_page.pagination.is_last_page()
    {
        return Err(Error::BadPagination(format!(
            "Last response in sequence is not last page: {:?}",
            last_page.pagination
        )));
    }

    for rhs_response in responses {
        if lhs_resp.as_info() != rhs_response.as_info() {
            return Err(Error::BadResponseInfo(format!(
                "Inconsistent response info: {:?} != {:?}",
                lhs_resp.as_info(),
                rhs_response.as_info()
            )));
        }

        lhs_resp.pagination = concat_pagination(lhs_resp.pagination, rhs_response.pagination, page_verify)?;
        lhs_resp.table = concat_tables(lhs_resp.table, rhs_response.table)?;
    }

    if lhs_resp.table.is_races() {
        lhs_resp.table = Table::Races {
            races: concat_races(lhs_resp.table.into_races()?)?,
        }
    }

    Ok(lhs_resp)
}

/// Concatenate two [`Pagination`]s, updating the `limit` field to reflect the total number of items
/// in the concatenated pages if `page_verify` contains [`PageVerify::CONTIGUOUS`].
///
/// # Errors
///
/// If `page_verify` contains [`PageVerify::CONTIGUOUS`] and the two pages are not contiguous,
/// an [`Error::BadPagination`] is returned.
fn concat_pagination(mut lhs: Pagination, rhs: Pagination, page_verify: PageVerify) -> Result<Pagination> {
    if page_verify.contains(PageVerify::CONTIGUOUS)
        && !((lhs.total == rhs.total) && (lhs.offset + lhs.limit == rhs.offset))
    {
        return Err(Error::BadPagination(format!("Response pages are not contiguous: {lhs:?} and {rhs:?}")));
    }

    lhs.limit += rhs.limit;

    Ok(lhs)
}

/// Concatenate two [`Table`]s of the same variant by concatenating their underlying [`Vec<T>`]s.
///
/// # Errors
///
/// If the two [`Table`]s are not of the same variant, an [`Error::BadTableVariant`] is returned.
//
// #[rstfmt::skip] can be applied only to the match statement if/when feature(stmt_expr_attributes)
// feature is stabilized, see the tracking issue at https://github.com/rust-lang/rust/issues/15701
#[rustfmt::skip]
fn concat_tables(lhs_table: Table, rhs_table: Table) -> Result<Table> {
    #[allow(clippy::enum_glob_use)]
    use Table::*;

    match (lhs_table, rhs_table) {
        (Seasons { seasons: lhs }, Seasons { seasons: rhs }) => Ok(Seasons { seasons: [lhs, rhs].concat() }),
        (Drivers { drivers: lhs }, Drivers { drivers: rhs }) => Ok(Drivers { drivers: [lhs, rhs].concat() }),
        (Constructors { constructors: lhs }, Constructors { constructors: rhs }) => Ok(Constructors { constructors: [lhs, rhs].concat() }),
        (Circuits { circuits: lhs }, Circuits { circuits: rhs }) => Ok(Circuits { circuits: [lhs, rhs].concat() }),
        (Races { races: lhs }, Races { races: rhs }) => Ok(Races { races: [lhs, rhs].concat() }),
        (Status { status: lhs }, Status { status: rhs }) => Ok(Status { status: [lhs, rhs].concat() }),
        _ => Err(Error::BadTableVariant),
    }
}

/// Concatenate a sequence of [`Race`]s by grouping them by their [`Race::as_info`] and
/// concatenating their underlying [`Payload`]s.
///
/// # Errors
///
/// If the [`Payload`] variants of all [`Race`]s with the same [`Race::as_info`] do not match,
/// an [`Error::BadPayloadVariant`] is returned.
fn concat_races(races: Vec<Race>) -> Result<Vec<Race>> {
    #[allow(clippy::enum_glob_use)]
    use Payload::*;

    type RaceInfo = (SeasonID, RoundID, Url, String, Circuit, Date, Option<Time>);
    let mut indexed_races: indexmap::IndexMap<RaceInfo, Race> = indexmap::IndexMap::new();

    for race in races {
        if indexed_races.contains_key(&race.to_info()) {
            match (&mut indexed_races[&race.to_info()].payload, race.payload) {
                // Payload::Schedule is the absence of a payload, so we do nothing
                (Schedule(_), Schedule(_)) => {}
                // For all others, we extend the inner Vec<T> of the Payload variant
                (QualifyingResults(lhs), QualifyingResults(rhs)) => lhs.extend(rhs),
                (SprintResults(lhs), SprintResults(rhs)) => lhs.extend(rhs),
                (RaceResults(lhs), RaceResults(rhs)) => lhs.extend(rhs),
                (Laps(lhs), Laps(rhs)) => lhs.extend(rhs),
                (PitStops(lhs), PitStops(rhs)) => lhs.extend(rhs),
                _ => return Err(Error::BadPayloadVariant),
            }
        } else {
            let _unused = indexed_races.insert(race.to_info(), race);
        }
    }

    Ok(indexed_races.into_values().collect())
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::{
        jolpica::{
            api::{JOLPICA_API_BASE_URL, JOLPICA_API_RATE_LIMIT_QUOTA},
            get::{get_response_multi_pages, get_response_page},
            resource::{Filters, Page, Resource},
            response::Pagination,
        },
        rate_limiter::RateLimiter,
    };

    use super::*;
    use crate::jolpica::tests::assets::*;

    const fn make_pagination(limit: u32, offset: u32, total: u32) -> Pagination {
        Pagination { limit, offset, total }
    }

    static RATE_LIMITER: LazyLock<RateLimiter> = LazyLock::new(|| RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA));

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

    static RESPONSES_SEASONS: LazyLock<Vec<Response>> = LazyLock::new(|| {
        vec![
            Response {
                pagination: make_pagination(2, 0, 6),
                table: Table::Seasons {
                    seasons: SEASON_TABLE.as_seasons().unwrap()[0..2].to_vec(),
                },
                ..RESPONSE_NONE.clone()
            },
            Response {
                pagination: make_pagination(2, 2, 6),
                table: Table::Seasons {
                    seasons: SEASON_TABLE.as_seasons().unwrap()[2..4].to_vec(),
                },
                ..RESPONSE_NONE.clone()
            },
            Response {
                pagination: make_pagination(2, 4, 6),
                table: Table::Seasons {
                    seasons: SEASON_TABLE.as_seasons().unwrap()[4..6].to_vec(),
                },
                ..RESPONSE_NONE.clone()
            },
        ]
    });

    static RESPONSES_DRIVERS: LazyLock<Vec<Response>> = LazyLock::new(|| {
        vec![
            Response {
                pagination: make_pagination(5, 0, 15),
                table: Table::Drivers {
                    drivers: DRIVER_TABLE.as_drivers().unwrap()[0..5].to_vec(),
                },
                ..RESPONSE_NONE.clone()
            },
            Response {
                pagination: make_pagination(3, 5, 15),
                table: Table::Drivers {
                    drivers: DRIVER_TABLE.as_drivers().unwrap()[5..8].to_vec(),
                },
                ..RESPONSE_NONE.clone()
            },
            Response {
                // `limit` purposely goes > `total` to test that case
                pagination: make_pagination(30, 8, 15),
                table: Table::Drivers {
                    drivers: DRIVER_TABLE.as_drivers().unwrap()[8..15].to_vec(),
                },
                ..RESPONSE_NONE.clone()
            },
        ]
    });

    static RESPONSE_RACES_CONCATENATED: LazyLock<Response> = LazyLock::new(|| Response {
        pagination: make_pagination(10, 0, 10),
        table: Table::Races {
            races: vec![
                Race {
                    payload: Payload::RaceResults(vec![
                        RACE_RESULT_2003_4_P1.clone(),
                        RACE_RESULT_2003_4_P2.clone(),
                        RACE_RESULT_2003_4_P19.clone(),
                    ]),
                    ..RACE_2003_4.clone()
                },
                Race {
                    payload: Payload::RaceResults(vec![
                        RACE_RESULT_2021_12_P1.clone(),
                        RACE_RESULT_2021_12_P2.clone(),
                        RACE_RESULT_2021_12_P3.clone(),
                        RACE_RESULT_2021_12_P10.clone(),
                    ]),
                    ..RACE_2021_12.clone()
                },
                Race {
                    payload: Payload::RaceResults(vec![
                        RACE_RESULT_2023_4_P1.clone(),
                        RACE_RESULT_2023_4_P2.clone(),
                        RACE_RESULT_2023_4_P20.clone(),
                    ]),
                    ..RACE_2023_4.clone()
                },
            ],
        },
        ..RESPONSE_NONE.clone()
    });

    static RESPONSE_RACES_RACE_RESULTS: LazyLock<Vec<Response>> = LazyLock::new(|| {
        vec![
            Response {
                pagination: make_pagination(5, 0, 10),
                table: Table::Races {
                    races: vec![
                        RESPONSE_RACES_CONCATENATED.clone().into_races().unwrap()[0].clone(),
                        Race {
                            payload: Payload::RaceResults(
                                RESPONSE_RACES_CONCATENATED.clone().into_races().unwrap()[1]
                                    .clone()
                                    .payload
                                    .into_race_results()
                                    .unwrap()[0..2]
                                    .to_vec(),
                            ),
                            ..RACE_2021_12.clone()
                        },
                    ],
                },
                ..RESPONSE_NONE.clone()
            },
            Response {
                pagination: make_pagination(5, 5, 10),
                table: Table::Races {
                    races: vec![
                        Race {
                            payload: Payload::RaceResults(
                                RESPONSE_RACES_CONCATENATED.clone().into_races().unwrap()[1]
                                    .clone()
                                    .payload
                                    .into_race_results()
                                    .unwrap()[2..4]
                                    .to_vec(),
                            ),
                            ..RACE_2021_12.clone()
                        },
                        RESPONSE_RACES_CONCATENATED.clone().into_races().unwrap()[2].clone(),
                    ],
                },
                ..RESPONSE_NONE.clone()
            },
        ]
    });

    #[test]
    fn concat_responses_seasons() {
        let response = concat_response_multi_pages(RESPONSES_SEASONS.clone(), PageVerify::ALL).unwrap();
        assert_eq!(response.as_info(), RESPONSE_NONE.as_info());
        assert_eq!(response.as_seasons().unwrap().len(), 6);
        assert_eq!(response.as_seasons().unwrap(), &SEASON_TABLE.as_seasons().unwrap()[..]);
        assert_eq!(response.pagination, make_pagination(6, 0, 6));

        let response = concat_response_multi_pages(
            RESPONSES_SEASONS[..2].to_vec(),
            PageVerify::CONTIGUOUS | PageVerify::START_AT_FIRST_PAGE,
        )
        .unwrap();
        assert_eq!(response.as_info(), RESPONSE_NONE.as_info());
        assert_eq!(response.as_seasons().unwrap().len(), 4);
        assert_eq!(response.as_seasons().unwrap(), &SEASON_TABLE.as_seasons().unwrap()[..4]);
        assert_eq!(response.pagination, make_pagination(4, 0, 6));

        let response = concat_response_multi_pages(
            vec![RESPONSES_SEASONS[0].clone()],
            PageVerify::CONTIGUOUS | PageVerify::START_AT_FIRST_PAGE,
        )
        .unwrap();
        assert_eq!(response.as_info(), RESPONSE_NONE.as_info());
        assert_eq!(response.as_seasons().unwrap().len(), 2);
        assert_eq!(response.as_seasons().unwrap(), &SEASON_TABLE.as_seasons().unwrap()[..2]);
        assert_eq!(response.pagination, make_pagination(2, 0, 6));
    }

    #[test]
    fn concat_responses_drivers() {
        let response = concat_response_multi_pages(RESPONSES_DRIVERS.clone(), PageVerify::ALL).unwrap();
        assert_eq!(response.as_info(), RESPONSE_NONE.as_info());
        assert_eq!(response.as_drivers().unwrap().len(), 15);
        assert_eq!(response.as_drivers().unwrap(), &DRIVER_TABLE.as_drivers().unwrap()[..]);
        // Note the accumulated `limit` > `total` to test that case
        assert_eq!(response.pagination, make_pagination(38, 0, 15));

        let response = concat_response_multi_pages(
            RESPONSES_DRIVERS[..2].to_vec(),
            PageVerify::CONTIGUOUS | PageVerify::START_AT_FIRST_PAGE,
        )
        .unwrap();
        assert_eq!(response.as_info(), RESPONSE_NONE.as_info());
        assert_eq!(response.as_drivers().unwrap().len(), 8);
        assert_eq!(response.as_drivers().unwrap(), &DRIVER_TABLE.as_drivers().unwrap()[..8]);
        assert_eq!(response.pagination, make_pagination(8, 0, 15));

        let response = concat_response_multi_pages(
            vec![RESPONSES_DRIVERS[0].clone()],
            PageVerify::CONTIGUOUS | PageVerify::START_AT_FIRST_PAGE,
        )
        .unwrap();
        assert_eq!(response.as_info(), RESPONSE_NONE.as_info());
        assert_eq!(response.as_drivers().unwrap().len(), 5);
        assert_eq!(response.as_drivers().unwrap(), &DRIVER_TABLE.as_drivers().unwrap()[..5]);
        assert_eq!(response.pagination, make_pagination(5, 0, 15));
    }

    #[test]
    fn concat_responses_races_race_results() {
        assert_eq!(
            concat_response_multi_pages(RESPONSE_RACES_RACE_RESULTS.clone(), PageVerify::ALL).unwrap(),
            *RESPONSE_RACES_CONCATENATED
        );
    }

    #[test]
    fn concat_responses_error_empty_list() {
        assert!(matches!(concat_response_multi_pages(vec![], PageVerify::NONE), Err(Error::EmptyResponseList)));
    }

    #[test]
    fn concat_responses_error_different_info() {
        let mut responses = RESPONSES_SEASONS.clone();
        responses[1].series = "f2".into();
        assert!(matches!(concat_response_multi_pages(responses, PageVerify::NONE), Err(Error::BadResponseInfo(_))));
    }

    #[test]
    fn concat_responses_error_page_verify_contiguous() {
        let responses = |page_verify| {
            concat_response_multi_pages(vec![RESPONSES_SEASONS[0].clone(), RESPONSES_SEASONS[2].clone()], page_verify)
        };

        let enforce_expect_err = vec![
            (PageVerify::NONE, false),
            (PageVerify::CONTIGUOUS, true),
            (PageVerify::START_AT_FIRST_PAGE, false),
            (PageVerify::FINISH_AT_LAST_PAGE, false),
            (PageVerify::ALL, true),
        ];

        for (page_verify, expect_err) in enforce_expect_err {
            assert!(expect_err == matches!(responses(page_verify), Err(Error::BadPagination(_))));
        }
    }

    #[test]
    fn concat_responses_error_page_verify_start_at_first_page() {
        let responses = |page_verify| concat_response_multi_pages(RESPONSES_SEASONS[1..].to_vec(), page_verify);

        let enforce_expect_err = vec![
            (PageVerify::NONE, false),
            (PageVerify::CONTIGUOUS, false),
            (PageVerify::START_AT_FIRST_PAGE, true),
            (PageVerify::FINISH_AT_LAST_PAGE, false),
            (PageVerify::ALL, true),
        ];

        for (page_verify, expect_err) in enforce_expect_err {
            assert!(expect_err == matches!(responses(page_verify), Err(Error::BadPagination(_))));
        }
    }

    #[test]
    fn concat_responses_error_page_verify_finish_at_last_page() {
        let responses = |page_verify| concat_response_multi_pages(RESPONSES_SEASONS[..2].to_vec(), page_verify);

        let enforce_expect_err = vec![
            (PageVerify::NONE, false),
            (PageVerify::CONTIGUOUS, false),
            (PageVerify::START_AT_FIRST_PAGE, false),
            (PageVerify::FINISH_AT_LAST_PAGE, true),
            (PageVerify::ALL, true),
        ];

        for (page_verify, expect_err) in enforce_expect_err {
            assert!(expect_err == matches!(responses(page_verify), Err(Error::BadPagination(_))));
        }
    }

    #[test]
    #[ignore]
    fn concat_responses_seasons_get_response_multi_pages() {
        let responses = get_response_multi_pages(
            JOLPICA_API_BASE_URL,
            &Resource::SeasonList(Filters::none()),
            None,
            None,
            Some(&RATE_LIMITER),
        )
        .unwrap();

        assert_eq!(responses.len(), 3); // 76 / 30 -> 3 pages
        assert_eq!(responses[0].as_seasons().unwrap().len(), 30);
        assert_eq!(responses[1].as_seasons().unwrap().len(), 30);
        assert_eq!(responses[2].as_seasons().unwrap().len(), 16);

        let response = concat_response_multi_pages(responses, PageVerify::ALL).unwrap();
        assert_eq!(response.as_seasons().unwrap().len(), 76);
        assert!(response.pagination.is_last_page());
        assert_eq!(response.pagination.offset, 0);
        assert_eq!(response.pagination.limit, 90);
        assert_eq!(response.pagination.total, 76);

        assert_eq!(response.as_seasons().unwrap()[0].season, 1950);
        assert_eq!(response.as_seasons().unwrap()[75].season, 2025);
    }

    #[test]
    #[ignore]
    fn concat_responses_races_race_results_get_response_multi_pages() {
        let get_race_results = |page| {
            RATE_LIMITER.wait_until_ready();
            get_response_page(JOLPICA_API_BASE_URL, &Resource::RaceResults(Filters::new().season(2019)), page).unwrap()
        };

        let response_r1_to_r3 = get_race_results(Some(Page::with_limit(60)));
        let races = response_r1_to_r3.table.as_races().unwrap();
        assert_eq!(
            response_r1_to_r3.pagination,
            Pagination {
                limit: 60,
                offset: 0,
                total: 420
            }
        );
        assert_eq!(races.len(), 3);
        assert_eq!(races[0].payload.as_race_results().unwrap().len(), 20);
        assert_eq!(races[1].payload.as_race_results().unwrap().len(), 20);
        assert_eq!(races[2].payload.as_race_results().unwrap().len(), 20);

        let responses = vec![
            // r1 [0..15]
            get_race_results(Some(Page::with(15, 0))),
            // r1 [15..20], r2 [0..5]
            get_race_results(Some(Page::with(10, 15))),
            // r2 [5..15]
            get_race_results(Some(Page::with(10, 25))),
            // r2 [15..20], r3 [0..10]
            get_race_results(Some(Page::with(15, 35))),
            // r3 [10..20]
            get_race_results(Some(Page::with(10, 50))),
        ];

        let concatenated_response =
            concat_response_multi_pages(responses, PageVerify::CONTIGUOUS | PageVerify::START_AT_FIRST_PAGE).unwrap();

        assert_eq!(response_r1_to_r3, concatenated_response);
    }
}
