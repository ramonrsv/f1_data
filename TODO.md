# TODO

- [ ] Look into replacing **Note:** with [^note]:
- [ ] Create ergast::api module to hold jolpica-f1 API base URL, rate limit, max request size, etc.
- [ ] Add support to ergast::JolpicaF1 to automatically make and merge multiple GET requests for
      multi-page responses, likely supporting a maximum number of allowed auto-requests, etc.
- [ ] Add support to ergast::JolpicaF1 to use jolpica-f1 database dumps as a source. May also need
      to add ergast::Resource functionality for
      https://github.com/jolpica/jolpica-f1/blob/main/docs/dumps.md#endpoint, and to
      ergast::JolpicaF1 to automatically download the latest database dump.
- [ ] Look into the full jolpica-f1 database schema
      https://dbdocs.io/jolpica/jolpica-f1?view=relationships, there may be a lot more there than is
      supported by the API
