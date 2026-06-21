use core::error::Error;
use core::fmt::{Display, Formatter, Result};

/// Result type for Brigid operations
pub type BrigidResult<T> = core::result::Result<T, nemesis::NemesisError>;

/// Represents errors that can occur during Brigid operations
#[derive(Debug)]
#[expect(
    clippy::module_name_repetitions,
    reason = "Compiler Type inference"
)]
pub enum BrigidError {
    /// A generic error with a message
    Generic(String),
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            BrigidError::Generic(s) => write!(f, "Generic error: {s}"),
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
            BrigidError::Generic(_)
            | BrigidError::FileNotFound(_)
            | BrigidError::Csv(_)
            | BrigidError::Json(_)
            | BrigidError::Xff(_)
            | BrigidError::DeleteRoot => None,
        }
    }
}

impl From<std::io::Error> for BrigidError {
    #[inline]
    fn from(err: std::io::Error) -> Self {
        BrigidError::Io(err)
    }
}

impl From<BrigidError> for nemesis::NemesisError {
    #[inline]
    fn from(err: BrigidError) -> Self {
        nemesis::NemesisError::new("brigid", err)
    }
}
