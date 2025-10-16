use std::sync::LazyLock;
use std::time::Duration;

use crate::{
    error::Result,
    jolpica::{api::JOLPICA_API_RATE_LIMIT_QUOTA, get::retry_on_http_error},
    rate_limiter::RateLimiter,
};

#[cfg(doc)]
use crate::jolpica;

/// Duration to wait between GET calls to avoid exceeding the jolpica-f1 API rate limits.
///
/// This is meant to to be used in crude rate limiting tests and benchmarks where the rate limiting
/// in [`jolpica::agent::Agent`] is not available, e.g. testing [`jolpica::get::get_response_page`].
///
/// In accordance with jolpica-f1 API's rate limit [`jolpica::api::JOLPICA_API_RATE_LIMIT`], 500
/// requests per hour translates to about 7.2 seconds between requests, so 8s to add some margin.
pub(crate) static RATE_LIMIT_SLEEP_DURATION: Duration = Duration::from_secs(8);

/// Default maximum number of attempts to retry on HTTP errors, for [`retry_on_http_error`].
const DEFAULT_HTTP_RETRY_MAX_ATTEMPT_COUNT: usize = 3;

/// Default sleep duration between attempts to retry on HTTP errors, for [`retry_on_http_error`]
const DEFAULT_HTTP_RETRY_SLEEP: Duration = RATE_LIMIT_SLEEP_DURATION;

/// Forward to [`retry_on_http_error`] with default retry parameters.
pub(crate) fn retry_http<T>(f: impl Fn() -> Result<T>) -> Result<T> {
    retry_on_http_error(f, DEFAULT_HTTP_RETRY_MAX_ATTEMPT_COUNT, DEFAULT_HTTP_RETRY_SLEEP)
}

/// Global shared rate limiter for application-wide tests making requests to the jolpica-f1 API.
pub(crate) static GLOBAL_JOLPICA_RATE_LIMITER: LazyLock<RateLimiter> =
    LazyLock::new(|| RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA));
