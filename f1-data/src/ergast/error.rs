use ureq;

use crate::ergast::response::{Payload, Table};

/// An error that may occur while processing a [`Resource`](crate::ergast::resource::Resource)
/// HTTP request from the Ergast API, via the provided family of `get_*` methods. These may be
/// underlying HTTP errors, represented by [`Error::Http`], errors parsing the JSON response,
/// represented by [`Error::Parse`], or errors due to unmet restrictions imposed on the response,
/// e.g. a request by a method supporting only single-page responses resulted in a multi-page
/// response, represented by [`Error::MultiPage`].
#[derive(Debug)]
pub enum Error {
    /// Underlying HTTP error, passing through the [`ureq::Error`] returned by
    /// [`ureq::Request::call`].
    Http(Box<ureq::Error>),

    /// Error parsing the JSON response into a serializable type from
    /// [`response`](crate::ergast::response), presumably an error from [`serde_json`] but passing
    /// through the [`std::io::Error`] returned by [`ureq::Response::into_json`].
    Parse(std::io::Error),

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
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Self::Http(Box::new(error))
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Parse(error)
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

pub type Result<T> = std::result::Result<T, Error>;
