use serde_json;
use ureq;

#[cfg(feature = "fantasy")]
use serde_yaml;

use crate::jolpica::response::{Payload, Table};

#[cfg(doc)]
use crate::jolpica::{resource::Resource, response};

/// An error that may occur while processing a [`Resource`] HTTP request from the jolpica-f1 API,
/// via the provided family of `get_*` methods.
///
/// These may be underlying HTTP errors, represented by [`Error::Http`], errors parsing the JSON
/// response, represented by [`Error::Parse`], or errors due to unmet restrictions imposed on the
/// response, e.g. a request by a method supporting only single-page responses resulted in a
/// multi-page response, represented by [`Error::MultiPage`].
#[derive(Debug)]
pub enum Error {
    /// Underlying HTTP error, passing through [`ureq::Error`] from [`ureq::RequestBuilder::call`].
    Http(ureq::Error),

    /// Forwarded [`std::io::Error`] that may be returned by various underlying functions, e.g.
    /// [`ureq::Body::read_json`], [`ureq::Body::read_to_string`], or [`std::fs::read_to_string`].
    Io(std::io::Error),

    /// Error parsing the JSON response into a serializable type from [`response`], passing through
    /// the [`serde_json::Error`] from [`serde_json::from_str`], or similar [`serde_json`] methods.
    Parse(serde_json::Error),

    /// Error parsing a YAML data file into a serializable type, passing through the
    /// [`serde_yaml::Error`] from [`serde_yaml::from_str`], or similar [`serde_yaml`] methods.
    #[cfg(feature = "fantasy")]
    YamlParse(serde_yaml::Error),

    /// A request by a method supporting only single-page responses resulted in a multi-page one.
    MultiPage,
    /// A request resulted in a response that did not contain the expected [`Table`] variant.
    BadTableVariant,
    /// A request resulted in a response that did not contain the expected [`Payload`] variant.
    BadPayloadVariant,
    /// A request resulted in a response that did not contain any of the expected elements.
    NotFound,
    /// A request resulted in a response that contained more than the expected number of elements.
    TooMany,
    /// A generic error for when unexpected data was found during processing of a response.
    UnexpectedData(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Self::Http(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Parse(error)
    }
}

#[cfg(feature = "fantasy")]
impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Self::YamlParse(error)
    }
}

impl From<Table> for Error {
    fn from(_: Table) -> Self {
        Self::BadTableVariant
    }
}

impl From<Payload> for Error {
    fn from(_: Payload) -> Self {
        Self::BadPayloadVariant
    }
}

/// Convenience type alias for [`Result<T, f1_data::error::Error>`].
pub type Result<T> = std::result::Result<T, Error>;
