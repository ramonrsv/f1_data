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
/// Performs a GET request to the jolpica-f1 API for a specific page of the specified
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
///     Some(Page::with_limit(50))
/// ).unwrap();
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
///   JOLPICA_API_BASE_URL,
///   &Resource::DriverInfo(Filters::new().driver_id("leclerc".into())),
///   None)
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
    ureq::get(resource.to_url_with_base_and_opt_page(base_url, page).as_str())
        .call()?
        .into_body()
        .read_json()
        .map_err(Into::into)
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
    use more_asserts::{assert_ge, assert_le};

    use crate::{
        error::Error,
        jolpica::{
            api::{JOLPICA_API_BASE_URL, JOLPICA_API_PAGINATION},
            resource::Filters,
        },
    };

    use super::*;
    use crate::jolpica::tests::{
        assets::*,
        util::{RATE_LIMIT_SLEEP_DURATION, retry_http},
    };

    fn rate_limited_get_response_page(base_url: &str, resource: &Resource, page: Option<Page>) -> Result<Response> {
        std::thread::sleep(RATE_LIMIT_SLEEP_DURATION);
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
        assert!(!resp.pagination.is_last_page());

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
        assert!(resp.pagination.is_last_page());

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
        assert!(resp.pagination.is_last_page());
    }

    #[test]
    #[ignore]
    fn get_response_page_multi_default() {
        let resp = retry_http(|| {
            rate_limited_get_response_page(JOLPICA_API_BASE_URL, &Resource::SeasonList(Filters::none()), None)
        })
        .unwrap();

        let pagination = resp.pagination;
        assert!(!pagination.is_single_page());
        assert!(!pagination.is_last_page());
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
        assert!(pagination.is_single_page());
        assert!(pagination.is_last_page());
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
        let req = Resource::SeasonList(Filters::none());
        let page = Page::with_limit(5);

        let mut resp =
            retry_http(|| rate_limited_get_response_page(JOLPICA_API_BASE_URL, &req, Some(page.clone()))).unwrap();
        assert!(!resp.pagination.is_last_page());

        let mut current_offset: u32 = 0;

        while !resp.pagination.is_last_page() {
            let pagination = resp.pagination;
            assert!(!pagination.is_single_page());
            assert_eq!(pagination.limit, page.limit());
            assert_eq!(pagination.offset, current_offset);
            assert!(pagination.total >= 74);

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
                rate_limited_get_response_page(JOLPICA_API_BASE_URL, &req, Some(pagination.next_page().unwrap().into()))
            })
            .unwrap();

            current_offset += page.limit();
        }

        let pagination = resp.pagination;
        assert!(!pagination.is_single_page());
        assert!(pagination.is_last_page());
        assert_eq!(pagination.limit, page.limit());
        assert_eq!(pagination.offset, current_offset);
        assert!(pagination.total >= 74);

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
}
