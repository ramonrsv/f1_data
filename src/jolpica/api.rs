use nonzero_ext::nonzero;

/// Base URL of endpoints for the [jolpica-f1](https://github.com/jolpica/jolpica-f1) API
pub const JOLPICA_API_BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";

#[derive(Clone, Copy, Debug)]
pub struct RateLimit {
    pub burst_limit_per_sec: std::num::NonZeroU32,
    pub sustained_limit_per_hour: std::num::NonZeroU32,
}

/// The rate limit for the [jolpica-f1](https://github.com/jolpica/jolpica-f1) API
///
/// As per <https://github.com/jolpica/jolpica-f1/blob/main/docs/rate_limits.md>
///     Burst limit: 4 request per second
///     Sustained limit: 500 requests per hour
pub const JOLPICA_API_RATE_LIMIT: RateLimit = RateLimit {
    burst_limit_per_sec: nonzero!(4u32),
    sustained_limit_per_hour: nonzero!(500u32),
};
