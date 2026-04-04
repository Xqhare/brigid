use crate::file::BrigidFile;

pub struct BrigidDirectory {
    name: String,
    files: Vec<BrigidFile>,
    directories: Vec<BrigidDirectory>,
}

impl BrigidDirectory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
    pub fn file(&mut self, name: &str, file: impl FnOnce(&mut BrigidFile)) -> &mut Self {
        let mut b_file = BrigidFile::new(name);
        file(&mut b_file);
        self.files.push(b_file);
        self
    }
    pub fn directory(&mut self, name: &str, dir: impl FnOnce(&mut BrigidDirectory)) -> &mut Self {
        let mut directory = BrigidDirectory::new(name);
        dir(&mut directory);
        self.directories.push(directory);
        self
    }
    pub fn get_file(&self, name: &str) -> Option<&BrigidFile> {
        if let Some(file) = self.files.iter().find(|f| f.name() == name) {
            return Some(file);
        } else {
            if let Some(file) = self.directories.iter().find_map(|d| d.get_file(name)) {
                return Some(file);
            }
        }
        None
    }
}
