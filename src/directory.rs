use crate::{
    error::{BrigidError, BrigidResult},
    file::BrigidFile,
};

/// Represents a directory in the Brigid structure.
pub struct BrigidDirectory {
    pub(crate) name: String,
    pub(crate) files: Vec<BrigidFile>,
    pub(crate) directories: Vec<BrigidDirectory>,
}

impl BrigidDirectory {
    /// Create a new `BrigidDirectory` with the specified name.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
    /// Define a file in this directory.
    pub fn file(&mut self, name: &str, file: impl FnOnce(&mut BrigidFile)) -> &mut Self {
        let mut b_file = BrigidFile::new(name);
        file(&mut b_file);
        self.files.push(b_file);
        self
    }
    /// Define a subdirectory in this directory.
    pub fn directory(&mut self, name: &str, dir: impl FnOnce(&mut BrigidDirectory)) -> &mut Self {
        let mut directory = BrigidDirectory::new(name);
        dir(&mut directory);
        self.directories.push(directory);
        self
    }
    /// Find a file by name or path within this directory (recursive).
    #[must_use]
    pub fn get_file(&self, name: &str) -> Option<&BrigidFile> {
        if let Some((dir_name, rest)) = name.split_once('/') {
            if let Some(dir) = self.directories.iter().find(|d| d.name == dir_name) {
                if let Some(file) = dir.get_file(rest) {
                    return Some(file);
                }
            }
        }

        if let Some(file) = self.files.iter().find(|f| f.name == name) {
            return Some(file);
        }

        for dir in &self.directories {
            if let Some(file) = dir.get_file(name) {
                return Some(file);
            }
        }

        None
    }
    pub(crate) fn establish(&mut self, current_path: &std::path::Path) -> BrigidResult<()> {
        for file in &mut self.files {
            let file_path = current_path.join(&file.name);
            file.path = Some(file_path.clone());
            if let Some(content) = &file.default_content {
                if !file_path.exists() {
                    content.clone().save(&file_path)?;
                }
            }
        }

        for dir in &mut self.directories {
            let dir_path = current_path.join(&dir.name);
            if !dir_path.exists() {
                std::fs::create_dir_all(&dir_path).map_err(BrigidError::Io)?;
            }
            dir.establish(&dir_path)?;
        }

        Ok(())
    }
}
