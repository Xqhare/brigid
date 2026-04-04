use std::path::Path;

use athena::XffValue;
use mawu::{MawuContents, mawu_value::MawuValue};

use crate::error::BrigidResult;

#[derive(Debug, Clone)]
pub enum Content {
    CSV(XffValue),
    XFF(XffValue),
    JSON(XffValue),
}

impl Content {
    pub fn into_xff(self) -> XffValue {
        match self {
            Content::CSV(xff) => xff,
            Content::XFF(xff) => xff,
            Content::JSON(xff) => xff,
        }
    }
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
