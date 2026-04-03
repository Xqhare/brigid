pub type BrigidResult<T> = Result<T, BrigidError>;

#[derive(Debug)]
pub enum BrigidError {
    Generic(String),
    Io(std::io::Error),
}

impl From<std::io::Error> for BrigidError {
    fn from(err: std::io::Error) -> Self {
        BrigidError::Io(err)
    }
}
