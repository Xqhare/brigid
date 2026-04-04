use athena::XffValue;

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
}
