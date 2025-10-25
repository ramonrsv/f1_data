//! Contains constants and information about the [jolpica-f1](https://github.com/jolpica/jolpica-f1)
//! API, e.g. base URL, rate limits, pagination, special values, etc.

use nonzero_ext::nonzero;

use crate::rate_limiter::Quota;

#[cfg(doc)]
use crate::{
    jolpica::{
        resource::{Filters, Page, Resource},
        response::{RaceResult, Response, SprintResult},
    },
    rate_limiter::RateLimiter,
};

/// Base URL of endpoints for the [jolpica-f1](https://github.com/jolpica/jolpica-f1) API
pub const JOLPICA_API_BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";

/// Represents a rate limit as expressed by the jolpica-f1 API.
#[derive(Copy, Clone, Debug)]
pub struct RateLimit {
    /// The burst limit, in requests per second.
    pub burst_limit_per_sec: std::num::NonZeroU32,
    /// The sustained limit, in requests per hour.
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

/// The above rate limit expressed as a [`Quota`], to be used with a [`RateLimiter`].
pub const JOLPICA_API_RATE_LIMIT_QUOTA: Quota = Quota::per_hour(JOLPICA_API_RATE_LIMIT.sustained_limit_per_hour)
    .allow_burst(JOLPICA_API_RATE_LIMIT.burst_limit_per_sec);

/// Represents pagination parameters for the jolpica-f1 API, e.g. max and default limits.
#[derive(Clone, Copy, Debug)]
pub struct Pagination {
    /// Default limit for a page, i.e. the number of items per page. This value is meant to match
    /// the default limit of the jolpica-f1 API, but that is not required for operation correctness.
    pub default_limit: u32,

    /// Default offset for a page, i.e. the number of items to skip before the first item.
    pub default_offset: u32,

    /// Maximum limit for a page. This value is meant to match the maximum limit of the jolpica-f1
    /// API, but that is not required for operation correctness. Note, however, that [`Page`]'s
    /// interface will enforce this maximum, e.g. [`Page::with_limit`] will panic if a value greater
    /// than this is passed. The actual limit returned in a [`Response`] may be lower than this max.
    pub max_limit: u32,
}

/// The pagination parameters for the [jolpica-f1](https://github.com/jolpica/jolpica-f1) API.
pub const JOLPICA_API_PAGINATION: Pagination = Pagination {
    default_limit: 30,
    default_offset: 0,
    max_limit: 100,
};

/// This value, as a grid position, indicates that a driver started the event from the pit lane.
///
/// It can be set in the [`Filters::grid_pos`] field, and may be returned in [`SprintResult::grid`]
/// and [`RaceResult::grid`]. See [`Resource::RaceResults`] for more information.
pub const GRID_PIT_LANE: u32 = 0;
