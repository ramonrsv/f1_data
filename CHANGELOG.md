# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- `/scripts/run_local_test_suite.sh` to run a CI-equivalent test suite locally outside of `act`
- Support for running tests/benchmarks against a local `jolpica-f1` server, via `LOCAL_JOLPICA=1`
- Scripts for setting up and running a local `jolpica-f1` API server instance in a .devcontainer
- [`crates.io`](crates.io) and documentation badges to `README.md`

### Fixed

-Broken documentation links, notably `None` and `Some`, after Rust 1.91 release

### Changed

- Move `*.patch` patches from `/scripts` to new `/batches` directory
- Upgrade dependencies: `serde_with` 3.15.1, `regex` 1.12.2, `indexmap` 2.12.0, `bitflags` 2.10.0

## [0.0.1] - 2025-10-30

### Added

- **`README.md`** - Comprehensive README with an overview of the crate and usage examples.
- **`lib.rs`** - Top-level crate documentation, lint enablement, and internal re-exports.
- **`docs/coding-guidelines.md`** - Coding guidelines that the crate should adhere to.
- **`error` module** - An `Error` type and associated `Result<T>` for all `f1_data` interfaces.
- **`id` module** - ID types for various entities, to be used uniformly across `f1_data` interfaces.
- **`rate_limiter` module** - A simple rate limiter with a minimal interface required by the crate.
- **`tests` module** - Common test utilities useful in tests across the entire crate.
- **`jolpica` module**- Wrapper around the [Jolpica F1](https://github.com/jolpica/jolpica-f1) API.
- **`/agent`** - The main `jolpica` interface, with configuration options, e.g. rate limiting, etc.
- **`/api`** - Information about the jolpica-f1 API, e.g. rate limits, base URL, etc.
- **`/concat`** - Functions to concatenate multi-page `Response`s into a single `Response`.
- **`/get`** - Functions for performing GET requests to the jolpica-f1 API, including multi-page.
- **`/resource`** - Types to identified resources and filters that can be requested from the API.
- **`/response`** - Types that represent and deserialize JSON responses from the jolpica-f1 API.
- **`/time`** - Types, aliases, and parsing of time/duration/date concepts/formats from jolpica-f1.
- **`/tests`** - Utilities and assets used in tests for the `jolpica` module and API.
- **`/tests/known_bugs`** - Tests for known bugs and issues, and the associated workarounds.
- **`fantasy` module** - (Private) module, under `fantasy` feature, for fantasy-related data.
- **CI/CD** - GitHub Actions workflow with build, test, clippy, fmt, code coverage, benchmarks, etc.
- **`scripts/test_no_run_docs.sh`** - Scripts for testing documentation tests marked with ```no_run.
- **`examples/validate_jolpica.rs`** - Example tool that recursively requests the whole jolpica API.
- **Benchmarks** - Criterion benchmarks for GET, deserialize, Resource::to_url, etc. operations.
- **`rustfmt.toml`** - rustfmt with `max_width = 120`, `fn_call_width = 100`.
- **`TODO.md`** - Issue tracking, to be moved to GitHub Issues.

<!-- next-url -->
[Unreleased]: https://github.com/ramonrsv/f1_data/compare/v0.0.1...HEAD
[0.0.1]: https://github.com/ramonrsv/f1_data/releases/tag/v0.0.1
