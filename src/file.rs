use std::path::PathBuf;

use crate::content::Content;

#[derive(Debug, Clone, Copy)]
pub enum DataType {
    Xff,
    Csv,
    Json,
}

pub struct BrigidFile {
    pub(crate) default_content: Option<Content>,
    pub(crate) data_type: Option<DataType>,
    pub(crate) name: String,
    pub(crate) path: Option<PathBuf>,
    pub(crate) fallback: bool,
}

impl BrigidFile {
    pub fn new(name: &str) -> Self {
        Self {
            default_content: None,
            name: name.to_string(),
            fallback: false,
            data_type: None,
            // If not path is set during build this needs to error
            path: None,
        }
    }
    pub fn with_default_content(&mut self, content: Content) -> &mut Self {
        match content {
            Content::XFF(_) => self.data_type = Some(DataType::Xff),
            Content::CSV(_) => self.data_type = Some(DataType::Csv),
            Content::JSON(_) => self.data_type = Some(DataType::Json),
        };
        self.default_content = Some(content);
        self
    }
    /// Only has an effect if `content` is set using `with_default_content`
    pub fn with_fallback(&mut self) -> &mut Self {
        if self.default_content.is_some() {
            self.fallback = true;
        }
        self
    }
}
