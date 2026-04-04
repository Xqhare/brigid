use crate::file::BrigidFile;

pub struct BrigidDirectory {
    name: String,
    contents: Vec<BrigidFile>,
}

impl BrigidDirectory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            contents: Vec::new(),
        }
    }
    pub fn file(&mut self, name: &str, closure: impl FnOnce(&mut BrigidFile)) -> &mut Self {
        todo!()
    }
}
