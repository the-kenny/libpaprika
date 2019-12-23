#[macro_use]
extern crate failure;

use serde_json;

mod recipe;
pub use recipe::*;

mod recipe_set;
pub use recipe_set::*;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO Error")]
    Io(std::io::Error),
    #[fail(display = "Serde Error")]
    Serde(serde_json::Error),
    #[fail(display = "ZIP Error")]
    Zip(zip::result::ZipError),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Serde(e)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(e: zip::result::ZipError) -> Error {
        Error::Zip(e)
    }
}
