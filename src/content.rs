use std::path::Path;

use athena::XffValue;
use mawu::{MawuContents, mawu_value::MawuValue};

use crate::error::BrigidResult;

/// Represents the different content types that Brigid can handle
#[derive(Debug, Clone)]
pub enum Content {
    /// CSV content stored as an XffValue
    CSV(XffValue),
    /// XFF content stored as an XffValue
    XFF(XffValue),
    /// JSON content stored as an XffValue
    JSON(XffValue),
}

impl Content {
    /// Convert the content into an XffValue
    #[must_use]
    pub fn into_xff(self) -> XffValue {
        match self {
            Content::CSV(xff) | Content::XFF(xff) | Content::JSON(xff) => xff,
        }
    }
    /// Save the content to the specified path
    ///
    /// # Errors
    /// Returns a BrigidError if the file cannot be saved
    pub fn save(self, path: &Path) -> BrigidResult<()> {
        match self {
            Content::CSV(xff) => {
                let contents = MawuContents::Csv(MawuValue::CSVArray(vec![vec![xff]]));
                if let Err(err) = mawu::write(path, contents) {
                    return Err(err.into());
                } else {
                    Ok(())
                }
            }
            Content::XFF(xff) => {
                if let Err(err) = nabu::serde::write(path, xff) {
                    return Err(err.into());
                } else {
                    Ok(())
                }
            }
            Content::JSON(xff) => {
                let contents = MawuContents::Json(xff);
                if let Err(err) = mawu::write_pretty(path, contents, 2) {
                    return Err(err.into());
                } else {
                    Ok(())
                }
            }
        }
    }
}
