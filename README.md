# f1_data

[![Build status](https://github.com/ramonrsv/f1_data/actions/workflows/ci.yml/badge.svg)](https://github.com/ramonrsv/f1_data/actions)
[![Crates.io](https://img.shields.io/crates/v/f1_data.svg)](https://crates.io/crates/f1_data)
[![Documentation](https://docs.rs/f1_data/badge.svg)](https://docs.rs/f1_data)
[![codecov](https://codecov.io/github/ramonrsv/f1_data/graph/badge.svg?token=LYPNED8OXF)](https://codecov.io/github/ramonrsv/f1_data)

`f1_data` is a Rust library that provides consolidated access to various sources of Formula 1
information and data, including event schedules, session results, timing and telemetry data, as well
as historical information about drivers, constructors, circuits, etc.

It aims to simplify the process of fetching and processing F1 data for applications by providing
Rust wrappers over different data sources and APIs, and a unified interface that consolidates this
data and abstracts away the different underlying sources.

## Usage

Add `f1_data` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
f1_data = "0.0.2"
```

Then, in your Rust code:

```rust
use f1_data::jolpica::{Agent, Filters};

let jolpica = Agent::default();

let michael_wins = jolpica.get_race_results(
    Filters::new()
        .driver_id("michael_schumacher".into())
        .finish_pos(1)
    ).unwrap();

assert_eq!(michael_wins.len(), 91);
```

## jolpica-f1

[Jolpica F1](https://github.com/jolpica/jolpica-f1) is an open source API for querying Formula 1
data, with backwards compatible endpoints for the now deprecated Ergast API.

The `jolpica` module is a wrapper around the this API, with additional functionality such as rate
limiting to comply with the [Terms of Use](https://github.com/jolpica/jolpica-f1/blob/main/TERMS.md)
, automatic handling of multi-page responses, handling HTTP errors and retries, configurable
alternate servers, etc. A synopsis of the wrapper interface and functionality is provided below.

The main entry point is the `Agent` struct, which provides methods for querying various
[jolpica-f1 API endpoints](https://github.com/jolpica/jolpica-f1/blob/main/docs/README.md#endpoints-and-documentation),
such as `get_drivers()`, `get_race_results()`, etc. Most methods accept an optional `Filters`
parameter to filter the results, which correspond to the jolpica-f1 API's route parameters, e.g. for
[race results](https://github.com/jolpica/jolpica-f1/blob/main/docs/endpoints/results.md#route-parameters).

```rust
let race = jolpica.get_race_result(
        Filters::new().season(2021).round(22).finish_pos(1)
    ).unwrap();

assert_eq!(race.race_result().driver.full_name(), "Max Verstappen");
```

The `AgentConfigs` struct allows configuring various aspects of the `Agent`, such as the base URL of
the jolpica-f1 server, rate limiting options, multi-page response handling, etc.
`AgentConfigs::default()` provides sensible defaults that respect the API's Terms of Use and should
work for most use cases.

```rust
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

The `get_*` methods of the `Agent` are provided for convenience. They do different levels of
post-processing and validation, and return different types. For example, `get_driver()` returns a
`Driver` with information about a single driver, parsed from the API's JSON response. However, all
of these methods build on top of the core lower-level `get_response()`, which accepts a `Resource`
representing the API endpoint to query, a `Filters` with none or any number of filters applied, and
returns a `Response` struct containing the full API response.

```rust
let response = jolpica.get_response(
    &Resource::DriverInfo(
        Filters::new().driver_id("leclerc".into()))
    ).unwrap();

let Table::Drivers { drivers } = response.table else {
    panic!("expected drivers table");
};

let charles = &drivers[0];
assert_eq!(charles.full_name(), "Charles Leclerc");
```

Note that if `AgentConfigs::multi_page` is set to `MultiPageOption::Enabled` - the default in
`AgentConfigs::default()`, then `get_response()`, and by extension many of the `get_*` methods, may
make multiple GET requests to fetch all pages of a multi-page response, transparently concatenating
the results into a single `Response`.

```rust
// Makes 9+ requests to fetch all pages
let all_drivers = jolpica.get_drivers().unwrap();
assert!(all_drivers.len() >= 864);
```
