use mawu::errors::MawuError;
use nabu::error::NabuError;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

pub type BrigidResult<T> = Result<T, BrigidError>;

#[derive(Debug)]
pub enum BrigidError {
    Generic(String),
    Mawu(MawuError),
    Nabu(NabuError),
    Many(Vec<BrigidError>),
    FileNotFound(String),
    Csv(String),
    Json(String),
    Xff(String),
    Io(std::io::Error),
}

impl Display for BrigidError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
        }
    }
}

impl Error for BrigidError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            BrigidError::Io(e) => Some(e),
            BrigidError::Mawu(e) => Some(e),
            BrigidError::Nabu(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for BrigidError {
    fn from(err: std::io::Error) -> Self {
        BrigidError::Io(err)
    }
}

impl From<MawuError> for BrigidError {
    fn from(err: MawuError) -> Self {
        BrigidError::Mawu(err)
    }
}

impl From<NabuError> for BrigidError {
    fn from(err: NabuError) -> Self {
        BrigidError::Nabu(err)
    }
}
