pub type $NAMEResult<T> = Result<T, $NAMEError>;

#[derive(Debug)]
pub enum $NAMEError {
    Generic(String),
    Io(std::io::Error),
}

impl From<std::io::Error> for $NAMEError {
    fn from(err: std::io::Error) -> Self {
        $NAMEError::Io(err)
    }
}
