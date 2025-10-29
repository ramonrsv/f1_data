# TODO

- [ ] Look into replacing **Note:** with [^note]:
- [ ] Add support to jolpica::Agent to use jolpica-f1 database dumps as a source. May also need
      to add to jolpica::Resource functionality for
      https://github.com/jolpica/jolpica-f1/blob/main/docs/endpoints/data/dumps.md, and to
      jolpica::Agent to automatically download the latest database dump.
- [ ] Look into the full jolpica-f1 database schema
      https://dbdocs.io/jolpica/jolpica-f1?view=relationships, there may be a lot more there than is
      supported by the API
- [ ] Add support for jolpica-f1 [driver](https://api.jolpi.ca/ergast/f1/2025/driverstandings/) and
      [constructor](https://api.jolpi.ca/ergast/f1/2025/constructorstandings/) standings
- [ ] Use [github-action-benchmark](https://github.com/benchmark-action/github-action-benchmark) to
      track benchmark results in CI.
- [ ] Look into using `newtype` idiom for ID types, and into implementing `From<>` for common
      sources, e.g. `&str` and `u32`. Consider using https://docs.rs/derive_more/latest/derive_more/
- [ ] Clean up and improve the `get_*` and `into_*` lap timings and pit stops interface.
- [ ] Rename `get_session_result(s)_for_event(s)`, maybe into:
      `get_event_session_results` - many results, one event
      `get_individual_session_results` - one result, many events
- [ ] Refactor `get` to introduce a `Request` that holds all the configurations and has
      `get_page` and `get_multi_pages` methods, similar to `ureq::Request`.
- [ ] Introduce `::HtpRetries` to clarify HTTP retries configs; `Option<uszie>` can be confusing.
- [ ] Consider unifying `sprint_shootout` and `sprint_qualifying` into one field.
- [ ] Investigate qualifying results missing for several race weekends, e.g. 2000, R2. Maybe add a
      check to `validate_jolpica` to catch these.
- [ ] Look into adding support for new jolpica alpha API: https://api.jolpi.ca/docs/#/,
      https://github.com/jolpica/jolpica-f1/discussions
