use crate::{
    error::{Error, Result},
    jolpica::{
        resource::{Page, Resource},
        response::Response,
    },
};

#[cfg(doc)]
use crate::jolpica::{agent::Agent, response::Pagination};

/// Performs a GET request to the jolpica-f1 API for a specific page of the specified
/// [`Resource`].
///
/// Performs a GET request to the jolpica-f1 API at `base_url` for a specific page of the specified
/// [`Resource`], and returns a [`Response`] with a single page, parsed from the JSON response,
/// of a possibly multi-page response. [`Response::pagination`] can be used to check for
/// [`Pagination::is_last_page`] and get [`Pagination::next_page`] to request the following page
/// of the response, via another call to this method.
///
/// This method performs no additional processing; it returns the top-level [`Response`] type that
/// is a direct representation of the full JSON response. It is provided here to maximize
/// flexibility and cover edge uses cases, but it is expected that users will use the convenience
/// methods in [`Agent`], e.g. [`Agent::get_seasons`], and/or the extractions methods in
/// [`Response`], e.g. [`Response::into_seasons`].
///
/// <div class="warning">
/// This method does not implement rate limiting or caching; users should be mindful to not violate
/// the jolpica-f1 API's
/// <a href="https://github.com/jolpica/jolpica-f1/blob/main/docs/rate_limits.md">rate limits</a> or
/// any of its <a href="https://github.com/jolpica/jolpica-f1/blob/main/TERMS.md">terms of
/// service</a>.
/// </div>
///
/// # Examples
///
/// ```no_run
/// # use f1_data::jolpica::{
/// #    api::{JOLPICA_API_BASE_URL, JOLPICA_API_PAGINATION},
/// #    get::get_response_page,
/// #    resource::{Filters, Page, Resource}};
/// #
/// let resp = get_response_page(
///     JOLPICA_API_BASE_URL,
///     &Resource::SeasonList(Filters::none()),
///     Some(Page::with_limit(50)),
/// )
/// .unwrap();
///
/// let seasons = resp.table.as_seasons().unwrap();
/// assert_eq!(seasons.len(), 50);
/// assert_eq!(seasons.first().unwrap().season, 1950);
/// assert_eq!(seasons.last().unwrap().season, 1999);
/// assert!(!resp.pagination.is_last_page());
///
/// let resp = get_response_page(
///     JOLPICA_API_BASE_URL,
///     &Resource::SeasonList(Filters::none()),
///     Some(resp.pagination.next_page().unwrap().into()),
/// )
/// .unwrap();
///
/// let seasons = resp.table.as_seasons().unwrap();
/// assert!(seasons.len() <= 50);
/// assert_eq!(seasons.first().unwrap().season, 2000);
/// assert!(resp.pagination.is_last_page());
///
/// let resp = get_response_page(
///     JOLPICA_API_BASE_URL,
///     &Resource::DriverInfo(Filters::new().driver_id("leclerc".into())),
///     None,
/// )
/// .unwrap();
///
/// assert_eq!(resp.pagination.limit, JOLPICA_API_PAGINATION.default_limit);
///
/// let drivers = resp.table.as_drivers().unwrap();
/// assert_eq!(drivers.len(), 1);
/// assert_eq!(drivers.first().unwrap().given_name, "Charles");
/// assert!(resp.pagination.is_last_page());
/// ```
pub fn get_response_page(base_url: &str, resource: &Resource, page: Option<Page>) -> Result<Response> {
    let url = resource.to_url_with_base_and_opt_page(base_url, page);
    let json_str = ureq::get(url.as_str()).call()?.into_body().read_to_string()?;

    // Use `serde_json::from_str::<Resp..>(.read_to_string())` instead of `.read_json::<Response>()`
    // to get better error messages, e.g. to get an [`Error::Parse(serde_json::Error)`] instead of
    // an [`Error::Http(ureq::Error)`] when there are problems parsing the JSON response. Benchmarks
    // also show that this method is slightly more performant - not that it would be significant,
    // since network latency and rate limiting take orders of magnitude longer than JSON parsing.
    serde_json::from_str::<Response>(json_str.as_str()).map_err(Into::into)
}

/// Performs GET requests to the jolpica-f1 API for all pages of the specified [`Resource`].
///
/// Performs GET requests to the jolpica-f1 API at `base_url` for all pages of the specified
/// [`Resource`], optionally up to a maximum allowed number of pages, via `max_page_count`. It
/// returns a [`Vec<Response>`] with the [`Response`]s parsed from the JSON responses.
///
/// This function unconditionally makes at least one request for either the optionally specified
/// `initial_page`, or by specifying no page at all. The [`Response::pagination`] field of the first
/// response is then used to determine the subsequent pages to request, if any, via
/// [`Pagination::next_page`]. If a `rate_limiter` is provided, it is used to wait before each
/// request, including the first.
///
/// This method performs no additional processing; it returns the top-level [`Response`]s that
/// are a direct representation of the full JSON responses. It is provided here to maximize
/// flexibility and cover edge uses cases, but it is expected that users will use the convenience
/// methods in [`Agent`], e.g. [`Agent::get_seasons`], and/or the extractions methods in
/// [`Response`], e.g. [`Response::into_seasons`].
///
/// # Errors
///
/// If `max_page_count` is specified, and the total number of pages would exceed it, then an
/// [`Error::ExceededMaxPageCount`] is returned and no requests beyond the first are made.
///
/// # Examples
///
/// ```no_run
/// # use f1_data::{
/// #     jolpica::{
/// #         api::{JOLPICA_API_BASE_URL, JOLPICA_API_RATE_LIMIT_QUOTA},
/// #         get::get_response_multi_pages,
/// #         resource::{Filters, Page, Resource},
/// #     },
/// #     rate_limiter::{Quota, RateLimiter},
/// # };
/// #
/// # let rate_limiter = RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA);
/// #
/// let responses = get_response_multi_pages(
///     JOLPICA_API_BASE_URL,
///     &Resource::SeasonList(Filters::none()),
///     Some(Page::with_limit(50)),
///     Some(10),
///     Some(&rate_limiter),
/// )
/// .unwrap();
///
/// assert_eq!(responses.len(), 2); // 76 / 50 -> 2 pages
/// assert!(!responses.first().unwrap().pagination.is_last_page());
/// assert!(responses.last().unwrap().pagination.is_last_page());
///
/// let seasons = responses.first().unwrap().table.as_seasons().unwrap();
/// assert_eq!(seasons.len(), 50);
/// assert_eq!(seasons.first().unwrap().season, 1950);
///
/// let seasons = responses.last().unwrap().table.as_seasons().unwrap();
/// assert_eq!(seasons.len(), 26);
/// assert_eq!(seasons.first().unwrap().season, 2000);
/// ```
pub fn get_response_multi_pages(
    base_url: &str,
    resource: &Resource,
    initial_page: Option<Page>,
    max_page_count: Option<usize>,
    rate_limiter: Option<&crate::rate_limiter::RateLimiter>,
) -> Result<Vec<Response>> {
    let rate_limiter_wait_until_ready = || {
        if let Some(limiter) = rate_limiter {
            limiter.wait_until_ready();
        }
    };

    rate_limiter_wait_until_ready();
    let mut responses = vec![get_response_page(base_url, resource, initial_page)?];
    let mut pages = vec![responses.last().unwrap().pagination];

    while let Some(next_page) = pages.last().unwrap().next_page() {
        pages.push(next_page);
    }

    if let Some(max_page_count) = max_page_count
        && pages.len() > max_page_count
    {
        return Err(Error::ExceededMaxPageCount(max_page_count));
    }

    for page in &pages[1..] {
        rate_limiter_wait_until_ready();
        responses.push(get_response_page(base_url, resource, Some((*page).into()))?);
    }

    Ok(responses)
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::{
        error::Error,
        jolpica::{
            api::{JOLPICA_API_BASE_URL, JOLPICA_API_PAGINATION, JOLPICA_API_RATE_LIMIT_QUOTA},
            resource::Filters,
            tests::util::GLOBAL_JOLPICA_RATE_LIMITER,
        },
        rate_limiter::RateLimiter,
    };

    use crate::jolpica::tests::{assets::*, util::retry_http};
    use crate::tests::asserts::*;
    use shadow_asserts::assert_eq;

    use super::*;

    fn rate_limited_get_response_page(base_url: &str, resource: &Resource, page: Option<Page>) -> Result<Response> {
        GLOBAL_JOLPICA_RATE_LIMITER.wait_until_ready();
        super::get_response_page(base_url, resource, page)
    }

    #[test]
    #[ignore]
    fn get_response_page() {
        let resp = retry_http(|| {
            rate_limited_get_response_page(
                JOLPICA_API_BASE_URL,
                &Resource::SeasonList(Filters::none()),
                Some(Page::with_limit(50)),
            )
        })
        .unwrap();

        let seasons = resp.table.as_seasons().unwrap();
        assert_eq!(seasons.len(), 50);
        assert_eq!(seasons.first().unwrap().season, 1950);
        assert_eq!(seasons.last().unwrap().season, 1999);
        assert_false!(resp.pagination.is_last_page());

        let resp = retry_http(|| {
            rate_limited_get_response_page(
                JOLPICA_API_BASE_URL,
                &Resource::SeasonList(Filters::none()),
                Some(resp.pagination.next_page().unwrap().into()),
            )
        })
        .unwrap();

        let seasons = resp.table.as_seasons().unwrap();
        assert_le!(seasons.len(), 50);
        assert_eq!(seasons.first().unwrap().season, 2000);
        assert_true!(resp.pagination.is_last_page());

        let resp = retry_http(|| {
            rate_limited_get_response_page(
                JOLPICA_API_BASE_URL,
                &Resource::DriverInfo(Filters::new().driver_id("leclerc".into())),
                None,
            )
        })
        .unwrap();

        assert_eq!(resp.pagination.limit, JOLPICA_API_PAGINATION.default_limit);

        let drivers = resp.table.as_drivers().unwrap();
        assert_eq!(drivers.len(), 1);
        assert_eq!(drivers.first().unwrap().given_name, "Charles");
        assert_true!(resp.pagination.is_last_page());
    }

    #[test]
    #[ignore]
    fn get_response_page_multi_default() {
        let resp = retry_http(|| {
            rate_limited_get_response_page(JOLPICA_API_BASE_URL, &Resource::SeasonList(Filters::none()), None)
        })
        .unwrap();

        let pagination = resp.pagination;
        assert_false!(pagination.is_single_page());
        assert_false!(pagination.is_last_page());
        assert_eq!(pagination.limit, JOLPICA_API_PAGINATION.default_limit);
        assert_eq!(pagination.offset, JOLPICA_API_PAGINATION.default_offset);
        assert_ge!(pagination.total, 76);

        let seasons = resp.table.as_seasons().unwrap();
        assert_eq!(seasons.len(), JOLPICA_API_PAGINATION.default_limit as usize);
        assert_eq!(seasons[0], *SEASON_1950);
    }

    #[test]
    #[ignore]
    fn get_response_page_single_max_limit() {
        let resp = retry_http(|| {
            rate_limited_get_response_page(
                JOLPICA_API_BASE_URL,
                &Resource::SeasonList(Filters::none()),
                Some(Page::with_max_limit()),
            )
        })
        .unwrap();

        let pagination = resp.pagination;
        assert_true!(pagination.is_single_page());
        assert_true!(pagination.is_last_page());
        assert_eq!(pagination.limit, JOLPICA_API_PAGINATION.max_limit);
        assert_eq!(pagination.offset, JOLPICA_API_PAGINATION.default_offset);
        assert_ge!(pagination.total, 76);

        let seasons = resp.table.as_seasons().unwrap();
        assert_ge!(seasons.len(), 76);
        assert_eq!(seasons[0], *SEASON_1950);
        assert_eq!(seasons[29], *SEASON_1979);
        assert_eq!(seasons[50], *SEASON_2000);
        assert_eq!(seasons[73], *SEASON_2023);
    }

    #[test]
    #[ignore]
    fn get_response_page_multi_page() {
        let resource = Resource::SeasonList(Filters::none());
        let page = Page::with_limit(5);

        let mut resp =
            retry_http(|| rate_limited_get_response_page(JOLPICA_API_BASE_URL, &resource, Some(page.clone()))).unwrap();
        assert_false!(resp.pagination.is_last_page());

        let mut current_offset: u32 = 0;

        while !resp.pagination.is_last_page() {
            let pagination = resp.pagination;
            assert_false!(pagination.is_single_page());
            assert_eq!(pagination.limit, page.limit());
            assert_eq!(pagination.offset, current_offset);
            assert_ge!(pagination.total, 76);

            let seasons = resp.table.as_seasons().unwrap();
            assert_eq!(seasons.len(), page.limit() as usize);

            match current_offset {
                0 => assert_eq!(seasons[0], *SEASON_1950),
                25 => assert_eq!(seasons[4], *SEASON_1979),
                50 => assert_eq!(seasons[0], *SEASON_2000),
                70 => assert_eq!(seasons[3], *SEASON_2023),
                _ => (),
            }

            resp = retry_http(|| {
                rate_limited_get_response_page(
                    JOLPICA_API_BASE_URL,
                    &resource,
                    Some(pagination.next_page().unwrap().into()),
                )
            })
            .unwrap();

            current_offset += page.limit();
        }

        let pagination = resp.pagination;
        assert_false!(pagination.is_single_page());
        assert_true!(pagination.is_last_page());
        assert_eq!(pagination.limit, page.limit());
        assert_eq!(pagination.offset, current_offset);
        assert_ge!(pagination.total, 76);

        let seasons = resp.table.as_seasons().unwrap();
        assert_eq!(seasons.last().unwrap().season, 1950 + current_offset + (seasons.len() as u32) - 1);
    }

    #[test]
    #[ignore]
    fn get_response_page_error_wrong_base_url() {
        assert!(matches!(
            super::get_response_page("http://nonexistent.local", &Resource::SeasonList(Filters::none()), None),
            Err(Error::Http(_))
        ));
    }

    #[test]
    #[ignore]
    fn get_response_multi_pages() {
        let resource = Resource::SeasonList(Filters::none());
        let page = Page::with_limit(5);

        let responses = super::get_response_multi_pages(
            JOLPICA_API_BASE_URL,
            &resource,
            Some(page.clone()),
            None,
            Some(&*GLOBAL_JOLPICA_RATE_LIMITER),
        )
        .unwrap();

        assert_false!(responses.first().unwrap().pagination.is_last_page());
        assert_true!(responses.last().unwrap().pagination.is_last_page());
        assert_ge!(responses.len(), 16); // 76 / 5

        let mut current_offset: u32 = 0;

        for resp in &responses {
            let pagination = resp.pagination;
            assert_false!(pagination.is_single_page());
            assert_eq!(pagination.limit, page.limit());
            assert_eq!(pagination.offset, current_offset);
            assert_ge!(pagination.total, 76);

            let seasons = resp.table.as_seasons().unwrap();

            if !resp.pagination.is_last_page() {
                assert_eq!(seasons.len(), page.limit() as usize);
            } else {
                assert_le!(seasons.len(), page.limit() as usize);
            }

            match current_offset {
                0 => assert_eq!(seasons[0], *SEASON_1950),
                25 => assert_eq!(seasons[4], *SEASON_1979),
                50 => assert_eq!(seasons[0], *SEASON_2000),
                70 => assert_eq!(seasons[3], *SEASON_2023),
                _ => (),
            }

            if !resp.pagination.is_last_page() {
                current_offset += page.limit();
            }
        }

        let pagination = responses.last().unwrap().pagination;
        assert_false!(pagination.is_single_page());
        assert_true!(pagination.is_last_page());
        assert_eq!(pagination.limit, page.limit());
        assert_eq!(pagination.offset, current_offset);
        assert_ge!(pagination.total, 76);

        let seasons = responses.last().unwrap().table.as_seasons().unwrap();
        assert_eq!(seasons.last().unwrap().season, 1950 + current_offset + (seasons.len() as u32) - 1);
    }

    #[test]
    #[ignore]
    fn get_response_multi_pages_rate_limiting() {
        // Requests take about ~300ms each without rate limiting
        // 500 requests per hour = 1 request every 7.2 seconds

        let rate_limiter = RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA);

        let start = std::time::Instant::now();
        let _responses = super::get_response_multi_pages(
            JOLPICA_API_BASE_URL,
            &Resource::SeasonList(Filters::none()),
            Some(Page::with_limit(20)),
            None,
            Some(&rate_limiter),
        );
        let elapsed = start.elapsed();
        assert_eq!(_responses.unwrap().len(), 4);

        // First four requests should not wait, ~600ms per request to allow for network latency
        assert_lt!(elapsed, Duration::from_millis(600 * 4));

        // Clear any accumulation from previous requests' latency
        rate_limiter.wait_until_ready();

        let start = std::time::Instant::now();
        let _responses = super::get_response_multi_pages(
            JOLPICA_API_BASE_URL,
            &Resource::SeasonList(Filters::none()),
            Some(Page::with_limit(20)),
            None,
            Some(&rate_limiter),
        );
        let elapsed = start.elapsed();
        assert_eq!(_responses.unwrap().len(), 4);

        // Subsequent requests should wait, at least ~7s each
        assert_ge!(elapsed, Duration::from_secs(7 * 4));
    }

    #[test]
    #[ignore]
    fn get_response_multi_pages_error_exceeded_max_page_count() {
        let rate_limiter = RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA);

        let req = Resource::SeasonList(Filters::none());

        let start = std::time::Instant::now();
        assert!(matches!(
            super::get_response_multi_pages(
                JOLPICA_API_BASE_URL,
                &req,
                Some(Page::with_limit(5)),
                Some(10),
                Some(&rate_limiter)
            ),
            Err(Error::ExceededMaxPageCount(10))
        ));
        let elapsed = start.elapsed();

        // Only the first request should have been made
        assert_lt!(elapsed, Duration::from_millis(600));
    }
}
