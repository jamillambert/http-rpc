#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::string::String;

use core::fmt;

#[derive(Debug)]
pub enum Error {
    /// JSON serialization error
    SerializationError,

    /// JSON deserialization error
    DeserializationError,

    /// HTTP error with status code
    HttpError(u16),

    /// Network error
    NetworkError,

    /// Custom error with message
    Custom(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SerializationError => write!(f, "Failed to serialize request"),
            Error::DeserializationError => write!(f, "Failed to deserialize response"),
            Error::HttpError(status) => write!(f, "HTTP error: {}", status),
            Error::NetworkError => write!(f, "Network error"),
            Error::Custom(msg) => write!(f, "{}", msg),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
