use core::error::Error;
use core::fmt::{Display, Formatter, Result};
use mawu::errors::MawuError;
use nabu::error::NabuError;

/// Result type for Brigid operations
#[allow(clippy::absolute_paths)]
pub type BrigidResult<T> = core::result::Result<T, BrigidError>;

/// Represents errors that can occur during Brigid operations
#[derive(Debug)]
#[allow(clippy::absolute_paths, clippy::module_name_repetitions)]
pub enum BrigidError {
    /// A generic error with a message
    Generic(String),
    /// Error from the Mawu crate
    Mawu(MawuError),
    /// Error from the Nabu crate
    Nabu(NabuError),
    /// Multiple errors occurred
    Many(Vec<BrigidError>),
    /// The specified file was not found
    FileNotFound(String),
    /// An error occurred during CSV processing
    Csv(String),
    /// An error occurred during JSON processing
    Json(String),
    /// An error occurred during XFF processing
    Xff(String),
    /// An I/O error occurred
    Io(std::io::Error),
    /// The root directory can not be deleted
    DeleteRoot,
}

impl Display for BrigidError {
    #[inline]
    #[allow(clippy::use_debug)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BrigidError::Generic(s) => write!(f, "Generic error: {s}"),
            BrigidError::Mawu(e) => write!(f, "Mawu error: {e}"),
            BrigidError::Nabu(e) => write!(f, "Nabu error: {e}"),
            BrigidError::Many(v) => write!(f, "Multiple errors: {v:?}"),
            BrigidError::FileNotFound(s) => write!(f, "File not found: {s}"),
            BrigidError::Csv(s) => write!(f, "CSV error: {s}"),
            BrigidError::Json(s) => write!(f, "JSON error: {s}"),
            BrigidError::Xff(s) => write!(f, "XFF error: {s}"),
            BrigidError::Io(e) => write!(f, "IO error: {e}"),
            BrigidError::DeleteRoot => write!(f, "Root directory can not be deleted"),
        }
    }
}

impl Error for BrigidError {
    #[inline]
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BrigidError::Io(e) => Some(e),
            BrigidError::Mawu(e) => Some(e),
            BrigidError::Nabu(e) => Some(e),
            BrigidError::Generic(_)
            | BrigidError::Many(_)
            | BrigidError::FileNotFound(_)
            | BrigidError::Csv(_)
            | BrigidError::Json(_)
            | BrigidError::Xff(_)
            | BrigidError::DeleteRoot => None,
        }
    }
}

#[allow(clippy::absolute_paths)]
impl From<std::io::Error> for BrigidError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        BrigidError::Io(err)
    }
}

impl From<MawuError> for BrigidError {
    #[inline]
    fn from(err: MawuError) -> Self {
        BrigidError::Mawu(err)
    }
}

impl From<NabuError> for BrigidError {
    #[inline]
    fn from(err: NabuError) -> Self {
        BrigidError::Nabu(err)
    }
}
