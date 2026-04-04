use mawu::errors::MawuError;
use nabu::error::NabuError;

pub type BrigidResult<T> = Result<T, BrigidError>;

#[derive(Debug)]
pub enum BrigidError {
    Generic(String),
    Mawu(MawuError),
    Nabu(NabuError),
    Io(std::io::Error),
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
