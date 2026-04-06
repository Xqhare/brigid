#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::print_stdout,
    clippy::implicit_return,
    clippy::single_call_fn,
    clippy::str_to_string,
    clippy::question_mark_used,
    clippy::indexing_slicing,
    clippy::pattern_type_mismatch,
    clippy::arbitrary_source_item_ordering,
    clippy::doc_paragraphs_missing_punctuation,
    clippy::exhaustive_enums
)]

mod builder;
/// Content handling for Brigid files
pub mod content;
mod directory;
/// Error types for Brigid operations
pub mod error;
mod file;
/// System-level warnings and configurations
pub mod sys_warning;
use std::{
    fs::{read, remove_dir_all, remove_file},
    path::PathBuf,
};

use athena::XffValue;
/// Reexporting `IoNiceClass` and `SchedulerPolicy`.
/// Needed if we want to set the scheduler policy of the current process.
pub use athena::process::{IoNiceClass, SchedulerPolicy};
use mawu::read::{csv_headless, json};

use crate::{
    builder::BrigidBuilder,
    content::Content,
    directory::BrigidDirectory,
    error::{BrigidError, BrigidResult},
    file::{BrigidFile, DataType},
    sys_warning::SystemWarning,
};

/// The main entry point for managing local file and directory structures.
///
/// `Brigid` provides a builder-based API to define and establish a directory structure
/// with default file contents (CSV, JSON, XFF) and system-level configurations.
pub struct Brigid {
    /// Path to the root directory
    /// Used as the base for all other, not absolute paths
    root: PathBuf,
    /// List of warnings
    system_warnings: Vec<SystemWarning>,
    /// Contains the root directory structure
    root_directory: BrigidDirectory,
}

impl Brigid {
    /// Create a new `BrigidBuilder` starting at the given root path.
    ///
    /// # Arguments
    ///
    /// * `root` - The root path of the project.
    ///
    /// # Returns
    ///
    /// A `BrigidBuilder` instance to configure the environment.
    ///
    /// # Example
    ///
    /// ```rust
    /// use brigid::Brigid;
    /// let builder = Brigid::new("my_app");
    /// ```
    #[allow(clippy::new_ret_no_self)]
    #[inline]
    pub fn new<P: Into<PathBuf>>(root: P) -> BrigidBuilder {
        BrigidBuilder::new(root)
    }
    /// Returns true if there are warnings generated during establishment.
    ///
    /// # Returns
    ///
    /// `true` if warnings were generated, `false` otherwise.
    #[must_use]
    #[inline]
    pub fn has_warnings(&self) -> bool {
        !self.system_warnings.is_empty()
    }
    /// Returns true if there are no warnings.
    ///
    /// # Returns
    ///
    /// `true` if no warnings were generated, `false` otherwise.
    #[must_use]
    #[inline]
    pub fn no_warnings(&self) -> bool {
        self.system_warnings.is_empty()
    }
    /// Get all warnings generated during establishment.
    ///
    /// # Returns
    ///
    /// A slice of `SystemWarning` generated during the build process.
    #[must_use]
    #[inline]
    pub fn get_warnings(&self) -> &[SystemWarning] {
        &self.system_warnings
    }
    /// Delete a file from disk.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or path of the file (e.g., "config.json" or "data/db.xff")
    ///
    /// # Returns
    ///
    /// A `BrigidResult` containing `()`.
    ///
    /// # Errors
    ///
    /// Returns a `BrigidError::FileNotFound` if the file-path is not found.
    /// Returns `BrigidError::Io` if the file cannot be deleted from disk.
    #[inline]
    pub fn delete_file(&self, name: &str) -> BrigidResult<()> {
        let path = self.get_file_path(name)?;
        remove_file(path).map_err(BrigidError::Io)
    }
    /// Update the content of a file.
    ///
    /// This only updates the file on disk. Default content is not updated.
    /// New file is created if it does not exist.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or path of the file (e.g., "config.json" or "data/db.xff")
    /// * `new_content` - The new content to be saved in the file.
    ///
    /// # Returns
    ///
    /// A `BrigidResult` containing `()`.
    ///
    /// # Errors
    ///
    /// Returns a `BrigidError::FileNotFound` if the file-path is not found.
    /// Returns `BrigidError::Io` if the file cannot be written to disk.
    #[inline]
    pub fn update_file(&self, name: &str, new_content: Content) -> BrigidResult<()> {
        let path = self.get_file_path(name)?;
        new_content.save(&path)
    }
    /// Get the path of a file.
    ///
    /// To read the contents of a file, use `get_file`.
    /// To update the contents of a file, use `update_file`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or path of the file (e.g., "config.json" or "data/db.xff")
    ///
    /// # Returns
    ///
    /// A `BrigidResult` containing the `PathBuf` of the file.
    ///
    /// # Errors
    ///
    /// Returns a `BrigidError::FileNotFound` if the file is not found.
    #[inline]
    pub fn get_file_path(&self, name: &str) -> BrigidResult<PathBuf> {
        file_path_getter(self.file_getter(name)?)
    }
    /// Get the content of a file as an `XffValue`.
    ///
    /// This method will attempt to read the file from disk using the inferred or
    /// explicitly set data type. If the file is missing or corrupted, it will
    /// attempt to fall back to a specified path or the default content.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or path of the file (e.g., "config.json" or "data/db.xff")
    ///
    /// # Returns
    ///
    /// A `BrigidResult` containing the `XffValue`.
    ///
    /// # Errors
    ///
    /// Returns a `BrigidError::FileNotFound` if the file is not found and no fallback is available.
    /// Returns `BrigidError::Mawu` or `BrigidError::Nabu` if parsing fails.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use brigid::Brigid;
    /// # use athena::XffValue;
    /// # let root = "test_get_file_doc";
    /// # let brigid = Brigid::new(root).establish().unwrap();
    /// // Assuming "config.json" was defined and exists
    /// let val = brigid.get_file("config.json");
    /// # brigid.delete_all().unwrap();
    /// ```
    #[inline]
    pub fn get_file(&self, name: &str) -> BrigidResult<XffValue> {
        let file = self.file_getter(name)?;
        let path = file_path_getter(file)?;

        let result = try_read_file(path, file.data_type);

        match result {
            Ok(val) => Ok(val),
            Err(err) => {
                // Try fallback path if provided
                if let Some(fallback_path) = &file.fallback_path
                    && let Ok(val) = try_read_file(fallback_path.clone(), file.data_type)
                {
                    return Ok(val);
                }

                // Try default content fallback
                if file.fallback
                    && let Some(default_content) = &file.default_content
                {
                    return Ok(default_content.clone().into_xff());
                }
                Err(err)
            }
        }
    }

    /// Get the raw bytes of a file.
    ///
    /// # Arguments
    ///
    /// * `name` - The name or path of the file.
    ///
    /// # Returns
    ///
    /// A `BrigidResult` containing a `Vec<u8>` of the file's content.
    ///
    /// # Errors
    ///
    /// Returns `BrigidError::Io` if the file cannot be read from disk.
    #[allow(clippy::absolute_paths)]
    #[inline]
    pub fn get_raw_file(&self, name: &str) -> BrigidResult<Vec<u8>> {
        let path = file_path_getter(self.file_getter(name)?)?;
        read(path).map_err(Into::into)
    }
    #[inline]
    fn file_getter(&self, name: &str) -> BrigidResult<&BrigidFile> {
        self.root_directory
            .get_file(name)
            .ok_or(BrigidError::FileNotFound(name.to_string()))
    }
    /// Delete all files and directories contained in the root of Brigid.
    ///
    /// Use with caution - Will attempt to delete all files and directories contained in the root
    /// and the root itself.
    ///
    /// # Returns
    ///
    /// A `BrigidResult` indicating success or failure.
    ///
    /// # Errors
    ///
    /// Returns `BrigidError::DeleteRoot` if the root is the filesystem root ("/").
    /// Returns `BrigidError::Io` if deletion fails.
    #[inline]
    pub fn delete_all(&self) -> BrigidResult<()> {
        if self.root == *"/" {
            return Err(BrigidError::DeleteRoot);
        }
        if let Err(err) = remove_dir_all(&self.root) {
            return Err(err.into());
        }
        Ok(())
    }
}
#[allow(clippy::absolute_paths)]
fn try_read_file(path: PathBuf, data_type: Option<DataType>) -> BrigidResult<XffValue> {
    match data_type {
        Some(DataType::Xff) => nabu::serde::read(path).map_err(Into::into),
        Some(DataType::Csv) => match csv_headless(path) {
            Ok(data) => {
                let xff = data
                    .to_csv_array()
                    .ok_or_else(|| BrigidError::Csv("File is not a CSV array".to_string()))?;

                // If it's 1x1, return the single value, otherwise return the whole array
                if xff.len() == 1 && xff[0].len() == 1 {
                    return Ok(xff[0][0].clone());
                }
                let rows = xff
                    .into_iter()
                    .map(|row| XffValue::Array(row.into()))
                    .collect::<Vec<XffValue>>();
                Ok(XffValue::Array(rows.into()))
            }
            Err(err) => Err(err.into()),
        },
        Some(DataType::Json) => json(path).map_err(Into::into),
        None => Err(BrigidError::FileNotFound(
            path.to_string_lossy().to_string(),
        )),
    }
}
#[inline]
fn file_path_getter(file: &BrigidFile) -> BrigidResult<PathBuf> {
    if let Some(path) = file.path.as_ref() {
        Ok(path.clone())
    } else {
        Err(BrigidError::FileNotFound(file.name.clone()))
    }
}
