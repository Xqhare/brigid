use std::path::PathBuf;

use crate::content::Content;

/// Represents the type of data stored in a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    /// XFF format
    Xff,
    /// CSV format
    Csv,
    /// JSON format
    Json,
}

/// Represents a file in the Brigid structure.
pub struct BrigidFile {
    pub(crate) default_content: Option<Content>,
    pub(crate) data_type: Option<DataType>,
    pub(crate) name: String,
    pub(crate) path: Option<PathBuf>,
    pub(crate) fallback: bool,
}

impl BrigidFile {
    /// Create a new `BrigidFile` with the specified name.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            default_content: None,
            name: name.to_string(),
            fallback: false,
            data_type: None,
            path: None,
        }
    }
    /// Set the default content and infer the data type.
    pub fn with_default_content(&mut self, content: Content) -> &mut Self {
        match content {
            Content::XFF(_) => self.data_type = Some(DataType::Xff),
            Content::CSV(_) => self.data_type = Some(DataType::Csv),
            Content::JSON(_) => self.data_type = Some(DataType::Json),
        };
        self.default_content = Some(content);
        self
    }
    /// Enable fallback to default content if the file is missing or corrupted.
    ///
    /// Only has an effect if `content` was set using `with_default_content`.
    pub fn with_fallback(&mut self) -> &mut Self {
        if self.default_content.is_some() {
            self.fallback = true;
        }
        self
    }
}
