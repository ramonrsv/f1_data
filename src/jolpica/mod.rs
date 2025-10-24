//! Wrapper module around the [Jolpica F1](https://github.com/jolpica/jolpica-f1) API.
//!
//! [Jolpica F1](https://github.com/jolpica/jolpica-f1) is an open source API for querying Formula 1
//! data, with backwards compatible endpoints for the now deprecated Ergast API. This module is a
//! wrapper around the this API, with additional functionality such as rate limiting to comply with
//! the [Terms of Use](https://github.com/jolpica/jolpica-f1/blob/main/TERMS.md), automatic handling
//! of multi-page responses, handling HTTP errors and retries, configurable alternate servers, etc.

pub mod agent;
pub mod api;
pub mod concat;
pub mod get;
pub mod resource;
pub mod response;
pub mod time;

#[cfg(test)]
pub(crate) mod tests;

pub use agent::{Agent, AgentConfigs, MultiPageOption, RateLimiterOption};
pub use resource::{Filters, LapTimeFilters, PitStopFilters, Resource};
pub use response::{Payload, Table};
