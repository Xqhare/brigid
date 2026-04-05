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
    pub(crate) fallback_path: Option<PathBuf>,
}

impl BrigidFile {
    /// Create a new `BrigidFile` with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the file.
    ///
    /// # Returns
    ///
    /// A new `BrigidFile` instance.
    #[must_use]
    pub fn new(name: &str) -> Self {
        let mut file = Self {
            default_content: None,
            name: name.to_string(),
            fallback: false,
            fallback_path: None,
            data_type: None,
            path: None,
        };
        file.try_infer_data_type();
        file
    }
    fn try_infer_data_type(&mut self) {
        if self.data_type.is_some() {
            return;
        }
        if self.name.ends_with(".json") {
            self.data_type = Some(DataType::Json);
        } else if self.name.ends_with(".csv") {
            self.data_type = Some(DataType::Csv);
        } else if self.name.ends_with(".xff") {
            self.data_type = Some(DataType::Xff);
        }
    }
    /// Set the default content and infer the data type.
    ///
    /// # Arguments
    ///
    /// * `content` - The `Content` to set as default.
    ///
    /// # Returns
    ///
    /// The `BrigidFile` instance.
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
    ///
    /// # Returns
    ///
    /// The `BrigidFile` instance.
    pub fn with_fallback(&mut self) -> &mut Self {
        if self.default_content.is_some() {
            self.fallback = true;
        }
        self
    }
    /// Set a fallback path for this file.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the fallback file.
    ///
    /// # Returns
    ///
    /// The `BrigidFile` instance.
    pub fn with_fallback_path<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        self.fallback_path = Some(path.into());
        self
    }
}
