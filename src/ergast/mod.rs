pub mod get;
pub mod resource;
pub mod response;
pub mod time;

#[cfg(test)]
pub(crate) mod tests;

/// Base URL of endpoints for the [jolpica-f1](https://github.com/jolpica/jolpica-f1) API
pub const JOLPICA_API_BASE_URL: &str = "https://api.jolpi.ca/ergast/f1";
