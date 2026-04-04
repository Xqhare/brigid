use crate::content::Content;

pub struct BrigidFile {
    default_content: Option<Content>,
    name: String,
    fallback: bool,
}

impl BrigidFile {
    pub fn new(name: &str) -> Self {
        Self {
            default_content: None,
            name: name.to_string(),
            fallback: false,
        }
    }
    pub fn with_default_content(&mut self, content: Content) -> &mut Self {
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
