use std::sync::LazyLock;

use crate::{
    error::Result,
    jolpica::{api::JOLPICA_API_RATE_LIMIT_QUOTA, get::retry_on_http_error},
    rate_limiter::RateLimiter,
};

#[cfg(doc)]
use crate::jolpica;

/// Global shared rate limiter for application-wide tests making requests to the jolpica-f1 API.
pub(crate) static GLOBAL_JOLPICA_RATE_LIMITER: LazyLock<RateLimiter> =
    LazyLock::new(|| RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA));

/// Default maximum number of attempts to retry on HTTP errors, for [`retry_on_http_error`].
pub(crate) const TESTS_DEFAULT_HTTP_RETRIES: usize = 3;

/// Forward to [`retry_on_http_error`] with default retry parameters and rate limiter.
pub(crate) fn retry_http<T>(f: impl Fn() -> Result<T>) -> Result<T> {
    retry_on_http_error(f, Some(&*GLOBAL_JOLPICA_RATE_LIMITER), Some(TESTS_DEFAULT_HTTP_RETRIES))
}
