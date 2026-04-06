use std::{fs::create_dir_all, path::Path};

use crate::{
    error::{BrigidError, BrigidResult},
    file::BrigidFile,
};

/// Represents a directory in the Brigid structure.
#[expect(
    clippy::field_scoped_visibility_modifiers,
    reason = "Intended for internal use"
)]
pub struct BrigidDirectory {
    pub(crate) name: String,
    pub(crate) files: Vec<BrigidFile>,
    pub(crate) directories: Vec<BrigidDirectory>,
}

impl BrigidDirectory {
    /// Create a new `BrigidDirectory` with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the directory.
    ///
    /// # Returns
    ///
    /// A new `BrigidDirectory` instance.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
    /// Define a file in this directory.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the file.
    /// * `file` - A closure to configure the `BrigidFile`.
    ///
    /// # Returns
    ///
    /// The `BrigidDirectory` instance.
    pub fn file(&mut self, name: &str, file: impl FnOnce(&mut BrigidFile)) -> &mut Self {
        let mut b_file = BrigidFile::new(name);
        file(&mut b_file);
        self.files.push(b_file);
        self
    }
    /// Define a subdirectory in this directory.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the subdirectory.
    /// * `dir` - A closure to configure the `BrigidDirectory`.
    ///
    /// # Returns
    ///
    /// The `BrigidDirectory` instance.
    pub fn directory(&mut self, name: &str, dir: impl FnOnce(&mut BrigidDirectory)) -> &mut Self {
        let mut directory = BrigidDirectory::new(name);
        dir(&mut directory);
        self.directories.push(directory);
        self
    }
    /// Find a file by name or path within this directory (recursive).
    ///
    /// # Arguments
    ///
    /// * `name` - The name or path of the file to find.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `BrigidFile` if found.
    #[must_use]
    #[expect(clippy::else_if_without_else, reason = "Implicit continue")]
    pub fn get_file(&self, name: &str) -> Option<&BrigidFile> {
        // 1. Try exact match in current directory (including subpaths if name has '/')
        if let Some((dir_name, rest)) = name.split_once('/')
            && let Some(dir) = self.directories.iter().find(|d| d.name == dir_name)
            && let Some(file) = dir.get_file(rest)
        {
            return Some(file);
        } else if let Some(file) = self.files.iter().find(|f| f.name == name) {
            return Some(file);
        }

        // 2. Fallback to recursive search ONLY if it's just a filename (no '/')
        if !name.contains('/') {
            for dir in &self.directories {
                if let Some(file) = dir.get_file(name) {
                    return Some(file);
                }
            }
        }

        None
    }
    pub(crate) fn establish(&mut self, current_path: &Path) -> BrigidResult<()> {
        for file in &mut self.files {
            let file_path = current_path.join(&file.name);
            file.path = Some(file_path.clone());
            if !file_path.exists()
                && let Some(content) = &file.default_content
            {
                content.clone().save(&file_path)?;
            }
        }

        for dir in &mut self.directories {
            let dir_path = current_path.join(&dir.name);
            if !dir_path.exists() {
                create_dir_all(&dir_path).map_err(BrigidError::Io)?;
            }
            dir.establish(&dir_path)?;
        }

        Ok(())
    }
}
