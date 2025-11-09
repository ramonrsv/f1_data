use std::sync::LazyLock;

use crate::{
    error::Result,
    jolpica::{
        agent::{Agent, AgentConfigs, MultiPageOption, RateLimiterOption},
        api::{JOLPICA_API_BASE_URL, JOLPICA_API_RATE_LIMIT_QUOTA},
        get::retry_on_http_error,
    },
    rate_limiter::RateLimiter,
};

/// Default maximum number of attempts to retry on HTTP errors, for [`retry_on_http_error`].
pub(crate) const TESTS_DEFAULT_HTTP_RETRIES: usize = 3;

/// Forward to [`retry_on_http_error`] with default retry parameters and rate limiter.
pub(crate) fn retry_http<T>(f: impl Fn() -> Result<T>) -> Result<T> {
    retry_on_http_error(f, get_jolpica_test_rate_limiter(), Some(TESTS_DEFAULT_HTTP_RETRIES))
}

/// Check if tests should use a local jolpica-f1 instance, based on `LOCAL_JOLPICA` env variable.
pub(crate) fn is_using_local_jolpica() -> bool {
    std::env::var("LOCAL_JOLPICA").map_or(false, |v| v == "1" || v == "true")
}

/// Get the jolpica-f1 API base URL for tests, based on `LOCAL_JOLPICA`, etc. environment variables.
///
/// The default base URL is [`JOLPICA_API_BASE_URL`], i.e. the real API base URL. If `LOCAL_JOLPICA`
/// environment variable is set, then it returns the value configured in `LOCAL_JOLPICA_BASE_URL`,
/// or `"http://localhost:8000/ergast/f1"` if it's not set. This is useful for testing against a
/// local server instance, to verify bug fixes, new impls, where rate limiting can be disabled, etc.
pub(crate) fn get_jolpica_test_base_url() -> String {
    if is_using_local_jolpica() {
        if let Ok(url) = std::env::var("LOCAL_JOLPICA_BASE_URL") {
            url
        } else {
            "http://localhost:8000/ergast/f1".to_string()
        }
    } else {
        JOLPICA_API_BASE_URL.to_string()
    }
}

/// Get a rate limiter for jolpica-f1 API tests, if any, depending on `LOCAL_JOLPICA` env variable.
///
/// The default is a shared global rate limiter configured with [`JOLPICA_API_RATE_LIMIT_QUOTA`]. If
/// `LOCAL_JOLPICA` is set, then it returns the same if `LOCAL_JOLPICA_ENABLE_RATE_LIMIT` is set,
/// otherwise it returns [`None`], meaning no rate limiting is applied.
pub(crate) fn get_jolpica_test_rate_limiter() -> Option<&'static RateLimiter> {
    /// Global shared rate limiter for application-wide tests making requests to the jolpica-f1 API.
    static JOLPICA_API_RATE_LIMITER: LazyLock<RateLimiter> =
        LazyLock::new(|| RateLimiter::new(JOLPICA_API_RATE_LIMIT_QUOTA));

    if is_using_local_jolpica() && std::env::var("LOCAL_JOLPICA_ENABLE_RATE_LIMIT").is_err() {
        None
    } else {
        Some(&*JOLPICA_API_RATE_LIMITER)
    }
}

/// Convert [`get_jolpica_test_rate_limiter()`] to a [`RateLimiterOption`], for convenience.
pub(crate) fn get_jolpica_test_rate_limiter_option() -> RateLimiterOption<'static> {
    if let Some(rate_limiter) = get_jolpica_test_rate_limiter() {
        RateLimiterOption::External(rate_limiter)
    } else {
        RateLimiterOption::None
    }
}

/// Shared instance of [`Agent`] for use in tests, with [`MultiPageOption::Disabled`].
///
/// Configured with [`get_jolpica_test_base_url()`] and [`get_jolpica_test_rate_limiter()`].
/// Based on the above, all tests may share a rate limiter, desired when using the real API.
pub(crate) static JOLPICA_SP: LazyLock<Agent<'_>> = LazyLock::new(|| {
    Agent::new(AgentConfigs {
        base_url: get_jolpica_test_base_url(),
        multi_page: MultiPageOption::Disabled,
        http_retries: Some(TESTS_DEFAULT_HTTP_RETRIES),
        rate_limiter: get_jolpica_test_rate_limiter_option(),
    })
});

/// Shared instance of [`Agent`] for use in tests, with [`MultiPageOption::Enabled`].
///
/// Configured with [`get_jolpica_test_base_url()`] and [`get_jolpica_test_rate_limiter()`].
/// Based on the above, all tests may share a rate limiter, desired when using the real API.
pub(crate) static JOLPICA_MP: LazyLock<Agent<'_>> = LazyLock::new(|| {
    Agent::new(AgentConfigs {
        base_url: get_jolpica_test_base_url(),
        multi_page: MultiPageOption::Enabled(None),
        http_retries: Some(TESTS_DEFAULT_HTTP_RETRIES),
        rate_limiter: get_jolpica_test_rate_limiter_option(),
    })
});

/// Get an estimated average duration (in milliseconds) of a request to the jolpica-f1 API.
///
/// This can be used in tests to take request latency into account when asserting time-based
/// conditions, e.g., rate limiting wait times, usually allowing for some margin of error.
///
/// The estimated averages are based on the `jolpica_get.rs/get_race_results/get_race_results`
/// benchmark results. When using a local jolpica-f1 instance the benchmark actually shows ~7ms,
/// but in practice - with other factors, no warming, etc. - we see around 15ms, so we use that.
pub(crate) fn get_request_avg_duration_ms() -> u64 {
    if is_using_local_jolpica() { 15 } else { 350 }
}
