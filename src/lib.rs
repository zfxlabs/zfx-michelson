#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/zfxlabs")]

pub mod micheline;
pub mod michelson;
pub mod michelson_map;
pub mod michelson_types;

pub use michelson::{install_parser, Parser};
pub use michelson_map::MichelsonMap;
pub use michelson_types::*;

/// Crate's `Error` type
#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ReadNone,
    IdMismatch,
    EncodeError {
        error: serde_json::Value,
    },
    DecodeError {
        error: serde_json::Value,
    },
    JsonError(serde_json::Error),
    /// Associated schema for the type is not  present, see [`JsonWrapped::SCHEMA_STR`]
    NoSchema,
    /// Errors during converting a raw [`serde_json::Value`] to a Rust data type
    EncodingError(String),
}

impl std::error::Error for Error {}

/// Crate's `Result` type
pub type Result<T> = std::result::Result<T, Error>;

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}

impl std::convert::From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonError(error)
    }
}
