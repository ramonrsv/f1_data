//! Functions for performing GET requests to the [jolpica-f1](https://github.com/jolpica/jolpica-f1)
//! API, including multi-page requests, returning the JSON response(s) parsed into [`Response`]s.

use crate::{
    error::{Error, Result},
    jolpica::{
        resource::{Page, Resource},
        response::Response,
    },
    rate_limiter::RateLimiter,
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
///     Some(2),
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
    rate_limiter: Option<&RateLimiter>,
    http_retries: Option<usize>,
) -> Result<Vec<Response>> {
    let mut responses = vec![retry_on_http_error(
        || get_response_page(base_url, resource, initial_page),
        rate_limiter,
        http_retries,
    )?];

    let mut pages = vec![responses.last().unwrap_or_else(|| unreachable!()).pagination];

    while let Some(next_page) = pages.last().unwrap_or_else(|| unreachable!()).next_page() {
        pages.push(next_page);
    }

    if let Some(max_page_count) = max_page_count
        && pages.len() > max_page_count
    {
        return Err(Error::ExceededMaxPageCount((pages.len(), max_page_count)));
    }

    for page in &pages[1..] {
        responses.push(retry_on_http_error(
            || get_response_page(base_url, resource, Some((*page).into())),
            rate_limiter,
            http_retries,
        )?);
    }

    Ok(responses)
}

/// Call the provided function, retrying on HTTP errors, and forwarding anything else.
///
/// The function `f` is unconditionally called at least once. If it returns [`Ok`], any error that
/// isn't [`Error::Http`], or if `max_retries` is [`None`] or [`Some(0)`](Some), then the result is
/// returned as-is. Otherwise, if it returns an [`Error::Http`] error, it calls the function again
/// up to `max_retries` times, returning the first [`Ok`] result or the first [`Error`] that isn't
/// [`Error::Http`]. If all attempts result in [`Error::Http`], then an [`Error::HttpRetries`] is
/// returned, holding the number of retries attempted and the last encountered [`ureq::Error`].
/// If a `rate_limiter` is provided, it is used to wait before each attempt, including the first.
pub fn retry_on_http_error<T>(
    f: impl Fn() -> Result<T>,
    rate_limiter: Option<&RateLimiter>,
    max_retries: Option<usize>,
) -> Result<T> {
    let max_retries = max_retries.unwrap_or(0);

    let rate_limited_call = || {
        if let Some(limiter) = rate_limiter {
            limiter.wait_until_ready();
        }
        f()
    };

    let mut result = rate_limited_call();

    if max_retries == 0 || !matches!(result, Err(Error::Http(_))) {
        return result;
    }

    for _ in 0..max_retries {
        result = rate_limited_call();

        if !matches!(result, Err(Error::Http(_))) {
            return result;
        }
    }

    let Err(Error::Http(ureq_err)) = result else {
        unreachable!()
    };
    Err(Error::HttpRetries((max_retries, ureq_err)))
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::time::Duration;

    use crate::{
        error::Error,
        jolpica::{
            api::{JOLPICA_API_BASE_URL, JOLPICA_API_PAGINATION, JOLPICA_API_RATE_LIMIT_QUOTA},
            resource::Filters,
            tests::util::GLOBAL_JOLPICA_RATE_LIMITER,
        },
        rate_limiter::{Quota, RateLimiter, nonzero},
    };

    use crate::jolpica::tests::{
        assets::*,
        util::{TESTS_DEFAULT_HTTP_RETRIES, retry_http},
    };
    use crate::tests::asserts::*;
    use shadow_asserts::assert_eq;

    use super::*;

    fn get_response_rate_limited_with_http_retries(
        base_url: &str,
        resource: &Resource,
        page: Option<Page>,
    ) -> Result<Response> {
        retry_http(|| super::get_response_page(base_url, resource, page))
    }

    #[test]
    #[ignore]
    fn get_response_page() {
        let resp = get_response_rate_limited_with_http_retries(
            JOLPICA_API_BASE_URL,
            &Resource::SeasonList(Filters::none()),
            Some(Page::with_limit(50)),
        )
        .unwrap();

        let seasons = resp.table.as_seasons().unwrap();
        assert_eq!(seasons.len(), 50);
        assert_eq!(seasons.first().unwrap().season, 1950);
        assert_eq!(seasons.last().unwrap().season, 1999);
        assert_false!(resp.pagination.is_last_page());

        let resp = get_response_rate_limited_with_http_retries(
            JOLPICA_API_BASE_URL,
            &Resource::SeasonList(Filters::none()),
            Some(resp.pagination.next_page().unwrap().into()),
        )
        .unwrap();

        let seasons = resp.table.as_seasons().unwrap();
        assert_le!(seasons.len(), 50);
        assert_eq!(seasons.first().unwrap().season, 2000);
        assert_true!(resp.pagination.is_last_page());

        let resp = get_response_rate_limited_with_http_retries(
            JOLPICA_API_BASE_URL,
            &Resource::DriverInfo(Filters::new().driver_id("leclerc".into())),
            None,
        )
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
        let resp = get_response_rate_limited_with_http_retries(
            JOLPICA_API_BASE_URL,
            &Resource::SeasonList(Filters::none()),
            None,
        )
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
        let resp = get_response_rate_limited_with_http_retries(
            JOLPICA_API_BASE_URL,
            &Resource::SeasonList(Filters::none()),
            Some(Page::with_max_limit()),
        )
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
            get_response_rate_limited_with_http_retries(JOLPICA_API_BASE_URL, &resource, Some(page.clone())).unwrap();
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

            resp = get_response_rate_limited_with_http_retries(
                JOLPICA_API_BASE_URL,
                &resource,
                Some(pagination.next_page().unwrap().into()),
            )
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
            Some(TESTS_DEFAULT_HTTP_RETRIES),
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
            None,
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
            None,
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
                Some(&rate_limiter),
                None
            ),
            // 76 / 5 -> 16 pages > 10 max
            Err(Error::ExceededMaxPageCount((16, 10)))
        ));
        let elapsed = start.elapsed();

        // Only the first request should have been made
        assert_lt!(elapsed, Duration::from_millis(600));
    }

    // Helper function to create a closure that counts how many times it has been called.
    // The counter is reset to zero whenever this function is called to make a new closure.
    fn make_counter_f<T>(count: &RefCell<u32>, f: impl Fn() -> Result<T>) -> impl Fn() -> Result<T> {
        *count.borrow_mut() = 0;

        move || {
            *count.borrow_mut() += 1;
            f()
        }
    }

    #[test]
    fn counter_closure() {
        let count = RefCell::<u32>::new(0);
        let f = make_counter_f(&count, || Ok(()));

        assert_eq!(*count.borrow(), 0);
        let _unused = f();
        assert_eq!(*count.borrow(), 1);
        let _unused = f();
        assert_eq!(*count.borrow(), 2);

        let f = make_counter_f(&count, || Ok(()));
        assert_eq!(*count.borrow(), 0);
        let _unused = f();
        assert_eq!(*count.borrow(), 1);
    }

    #[test]
    fn retry_on_http_error() {
        let count = RefCell::<u32>::new(0);

        let f_ok = || Ok(42);
        let f_err_http = || Err(Error::Http(ureq::Error::ConnectionFailed));
        let f_err_non_http = || Err(Error::NotFound);

        // let compiler deduce the type of the closures
        let _unused: Result<u32> = f_err_http();
        let _unused: Result<u32> = f_err_non_http();

        // No retries, forwards everything
        let result = super::retry_on_http_error(make_counter_f(&count, f_ok), None, None);
        assert_eq!(result.unwrap(), 42);
        assert_eq!(*count.borrow(), 1);

        let result = super::retry_on_http_error(make_counter_f(&count, f_err_http), None, None);
        assert!(matches!(result, Err(Error::Http(_))));
        assert_eq!(*count.borrow(), 1);

        let result = super::retry_on_http_error(make_counter_f(&count, f_err_non_http), None, Some(0));
        assert!(matches!(result, Err(Error::NotFound)));
        assert_eq!(*count.borrow(), 1);

        // Succeeds on first try
        let result = super::retry_on_http_error(make_counter_f(&count, f_ok), None, Some(3));
        assert_true!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(*count.borrow(), 1);

        // Fails with non-HTTP error
        let result = super::retry_on_http_error(make_counter_f(&count, f_err_non_http), None, Some(3));
        assert!(matches!(result, Err(Error::NotFound)));
        assert_eq!(*count.borrow(), 1);

        // Fails twice with HTTP error, then succeeds
        let result = super::retry_on_http_error(
            make_counter_f(&count, || if *count.borrow() < 3 { f_err_http() } else { f_ok() }),
            None,
            Some(3),
        );
        assert_eq!(result.unwrap(), 42);
        assert_eq!(*count.borrow(), 3);

        // Fails twice with HTTP error, then with non-HTTP error
        let result = super::retry_on_http_error(
            make_counter_f(&count, || {
                if *count.borrow() < 3 {
                    f_err_http()
                } else {
                    f_err_non_http()
                }
            }),
            None,
            Some(3),
        );
        assert!(matches!(result, Err(Error::NotFound)));
        assert_eq!(*count.borrow(), 3);

        // Fails with HTTP error exceeding max retries
        let result = super::retry_on_http_error(make_counter_f(&count, f_err_http), None, Some(3));
        assert!(matches!(result, Err(Error::HttpRetries((3, _)))));
        assert_eq!(*count.borrow(), 4);

        // Rate limiting, 100ms per call
        let rate_limiter = RateLimiter::new(Quota::per_second(nonzero!(10u32)).allow_burst(nonzero!(1u32)));
        rate_limiter.wait_until_ready(); // Clear the starting burst cell

        let start = std::time::Instant::now();
        let result = super::retry_on_http_error(make_counter_f(&count, f_err_http), Some(&rate_limiter), Some(3));
        let elapsed = start.elapsed();

        assert!(matches!(result, Err(Error::HttpRetries((3, _)))));
        assert_eq!(*count.borrow(), 4);
        assert_ge!(elapsed, Duration::from_millis(100 * 4));
        assert_lt!(elapsed, Duration::from_millis(100 * 5));
    }
}
