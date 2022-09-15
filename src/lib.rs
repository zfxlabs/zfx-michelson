#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/zfxlabs")]

pub mod micheline;
pub mod michelson;
mod michelson_map;
mod michelson_types;
pub mod prelude;

pub use michelson::{install_parser, Parser};
pub use michelson_map::MichelsonMap;
pub use michelson_types::*;

use serde_json::value::Value; // FIXME: IDK if this is a good idea here

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    ReadNone,
    IdMismatch,
    EncodeError { error: Value },
    DecodeError { error: Value },
}

impl std::error::Error for Error {}

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
