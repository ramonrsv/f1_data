/*!
`f1_data` is a Rust library that provides consolidated access to various sources of Formula 1
information and data, including event schedules, session results, timing and telemetry data, as well
as historical information about drivers, constructors, circuits, etc.

It aims to simplify the process of fetching and processing F1 data for applications by providing
Rust wrappers over different data sources and APIs, and a unified interface that consolidates this
data and abstracts away the different underlying sources.

# Usage
Add `f1_data` as a dependency in your `Cargo.toml`:
```toml
[dependencies]
f1_data = "0.0.2"
```

Then, in your Rust code:
```no_run
use f1_data::jolpica::{Agent, Filters};

let jolpica = Agent::default();

let michael_wins = jolpica
    .get_race_results(Filters::new().driver_id("michael_schumacher".into()).finish_pos(1))
    .unwrap();

assert_eq!(michael_wins.len(), 91);
```

# jolpica-f1
[Jolpica F1](https://github.com/jolpica/jolpica-f1) is an open source API for querying Formula 1
data, with backwards compatible endpoints for the now deprecated Ergast API.

The [`jolpica`] module is a wrapper around this API, with additional functionality such as rate
limiting to comply with the [Terms of Use](https://github.com/jolpica/jolpica-f1/blob/main/TERMS.md)
, automatic handling of multi-page responses, handling HTTP errors and retries, configurable
alternate servers, etc. A synopsis of the wrapper interface and functionality is provided below.

The main entry point is the [`Agent`](jolpica::Agent) struct, which provides methods for querying
various [jolpica-f1 API endpoints](https://github.com/jolpica/jolpica-f1/blob/main/docs/README.md#endpoints-and-documentation),
such as [`get_drivers()`](jolpica::Agent::get_drivers),
[`get_race_results()`](jolpica::Agent::get_race_results), etc. Most methods accept an optional
[`Filters`](jolpica::Filters) parameter to filter the results, which correspond to the jolpica-f1
API's route parameters, e.g. for [race results](https://github.com/jolpica/jolpica-f1/blob/main/docs/endpoints/results.md#route-parameters).

```no_run
# use f1_data::jolpica::{Agent, Filters};
# let jolpica = Agent::default();
#
let race = jolpica
    .get_race_result(Filters::new().season(2021).round(22).finish_pos(1))
    .unwrap();

assert_eq!(race.race_result().driver.full_name(), "Max Verstappen");
```

The [`AgentConfigs`](jolpica::AgentConfigs) struct allows configuring various aspects of the
[`Agent`](jolpica::Agent), such as the base URL of the jolpica-f1 server, rate limiting options,
multi-page response handling, etc. [`AgentConfigs::default()`](jolpica::AgentConfigs::default)
provides sensible defaults that respect the API's Terms of Use and should work for most use cases.

```no_run
# use nonzero_ext::nonzero;
#
# use f1_data::{
#     jolpica::{Agent, AgentConfigs, MultiPageOption, RateLimiterOption},
#     rate_limiter::{Quota, RateLimiter},
# };
#
let jolpica = Agent::default();

// The above is equivalent to:
let jolpica = Agent::new(AgentConfigs {
    base_url: "https://api.jolpi.ca/ergast/f1/".into(),
    multi_page: MultiPageOption::Enabled(None),
    http_retries: Some(2),
    rate_limiter: RateLimiterOption::Internal(RateLimiter::new(
        Quota::per_hour(nonzero!(500u32)).allow_burst(nonzero!(4u32)),
    )),
});
```

The `get_*` methods of the [`Agent`](jolpica::Agent) are provided for convenience. They do different
levels of post-processing and validation, and return different types. For example,
[`get_driver()`](jolpica::Agent::get_driver) returns a [`Driver`](jolpica::response::Driver) with
information about a single driver, parsed from the API's JSON response. However, all of these
methods build on top of the core lower-level [`get_response()`](jolpica::Agent::get_response),
which accepts a [`Resource`](jolpica::Resource) representing the API endpoint to query, a
[`Filters`](jolpica::Filters) with none or any number of filters applied, and returns a
[`Response`](jolpica::response::Response) struct containing the full API response.

```no_run
# use f1_data::jolpica::{Agent, Filters, Resource, Table};
# let jolpica = Agent::default();
#
let response = jolpica
    .get_response(&Resource::DriverInfo(Filters::new().driver_id("leclerc".into())))
    .unwrap();

let Table::Drivers { drivers } = response.table else {
    panic!("expected drivers table");
};

let charles = &drivers[0];
assert_eq!(charles.full_name(), "Charles Leclerc");
```

Note that if [`AgentConfigs::multi_page`](jolpica::AgentConfigs::multi_page) is set to
[`MultiPageOption::Enabled`](jolpica::MultiPageOption::Enabled) - the default in
[`AgentConfigs::default()`](jolpica::AgentConfigs::default), then
[`get_response()`](jolpica::Agent::get_response), and by extension many of the `get_*` methods, may
make multiple GET requests to fetch all pages of a multi-page response, transparently concatenating
the results into a single [`Response`](jolpica::response::Response).

```no_run
# use f1_data::jolpica::{Filters, Agent};
# let jolpica = Agent::default();
#
// Makes 9+ requests to fetch all pages
let all_drivers = jolpica.get_drivers(Filters::none()).unwrap();
assert!(all_drivers.len() >= 864);
```
*/

// These are all the lint groups or allowed-by-default lints that are enabled as `deny` in f1_data.
// Lints that are warn-by-default, deny-by-default, or in one of the groups are not included, for
// brevity. As such, it is expected that `cargo build/test/clippy/doc` will be run with
// `-D warnings` in CI workflows. `#![deny(warnings)]` is not included in this list as that is
// an anti-pattern: https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html.
// @todo Move these to Cargo.toml once stabilized: https://github.com/rust-lang/cargo/issues/12115
#![deny(
    // rustc groups
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    rust_2024_compatibility,
    unused,

    // rustc
    macro_use_extern_crate,
    meta_variable_misuse,
    missing_abi,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    non_ascii_idents,
    noop_method_call,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unstable_features,
    unused_crate_dependencies,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_results,
    variant_size_differences,

    // clippy
    clippy::all,
    clippy::cargo,
    clippy::suspicious,

    // rustdoc
    rustdoc::all,
)]
//
// Clippy lints from the `pedantic`, `nursery`, and `restriction` groups are more finicky and
// require a combination of opt-in and opt-out strategies to minimize false positives and verbosity.
// For ease of new development these are set to `warn`, but note that CI will treat them as `deny`.
#![warn(
    clippy::pedantic,
    clippy::nursery,

    // restriction, opt-in
    clippy::dbg_macro,
    clippy::print_stdout,
    clippy::unimplemented,
    clippy::todo,
)]
#![allow(
    // pedantic, opt-out
    clippy::must_use_candidate,

    // nursery, opt-out
)]
//
// These lints are temporarily allowed while fixes for associated violations are being worked on.
// Developers can locally change to `warn` to see the warnings - CI would fail due to `-D warnings`.
// @todo Fix the associated violations and remove these lints - list should normally be empty.
#![allow(clippy::missing_errors_doc)]

// Silence unused_crate_dependencies lint for [dev-dependencies] used in /benches and /examples.
// While clippy detects uses in unit tests, it doesn't seem to capture these particular uses.
#[cfg(test)]
mod _lint {
    use anyhow as _;
    use colored as _;
    use criterion as _;
    use env_logger as _;
    use log as _;
}

pub mod error;
pub mod id;
pub mod jolpica;
pub mod rate_limiter;

// @todo Make this public if and when a solution for sourcing the data is found.
#[allow(unreachable_pub, unused, rustdoc::private_doc_tests)]
#[cfg(feature = "fantasy")]
mod fantasy;

#[cfg(test)]
pub(crate) mod tests;
