mod recipe;
pub use recipe::*;

mod recipe_set;
pub use recipe_set::*;

#[cfg(feature = "api")]
pub mod api;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("ZIP Error: {0}")]
    Zip(#[from] zip::result::ZipError),
}
