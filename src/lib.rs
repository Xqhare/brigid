#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![warn(clippy::all)]
#![warn(clippy::restriction)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::print_stdout,
    clippy::implicit_return
)]

mod builder;
pub mod content;
mod directory;
pub mod error;
mod file;
pub mod sys_warning;
use std::path::PathBuf;

use athena::XffValue;
/// Remember: IoNiceClass and SchedulerPolicy need to be exposed publicly, rest not
pub use athena::process::{IoNiceClass, SchedulerPolicy};

use crate::{
    builder::BrigidBuilder,
    directory::BrigidDirectory,
    error::{BrigidError, BrigidResult},
    file::{BrigidFile, DataType},
    sys_warning::SystemWarning,
};

pub struct Brigid {
    /// Path to the root directory
    /// Used as the base for all other, not absolute paths
    root: PathBuf,
    /// List of warnings
    system_warnings: Vec<SystemWarning>,
    /// Contains the root directory structure
    root_directory: BrigidDirectory,
    /// List of all paths, including files and directories.
    all_paths: Vec<PathBuf>,
}

impl Brigid {
    /// Create a new Brigid
    pub fn new<P: Into<PathBuf>>(root: P) -> BrigidBuilder {
        BrigidBuilder::new(root)
    }
    /// Returns true if there are warnings
    pub fn has_warnings(&self) -> bool {
        !self.system_warnings.is_empty()
    }
    /// Returns true if there are no warnings
    pub fn no_warnings(&self) -> bool {
        self.system_warnings.is_empty()
    }
    /// Get all warnings
    ///
    /// Returned Vec is empty if there are no warnings
    pub fn get_warnings(&self) -> &Vec<SystemWarning> {
        &self.system_warnings
    }
    /// Get the XffValue of a file
    pub fn get_file(&self, name: &str) -> BrigidResult<XffValue> {
        if let Ok(file) = self.file_getter(name) {
            let path = self.file_path_getter(file)?;
            let has_fallback = file.has_fallback();
            let mut has_error = false;
            if let Some(data_type) = file.data_type() {
                match data_type {
                    DataType::Xff => {
                        let data =
                            nabu::serde::read(path).map_err(|err| Into::<BrigidError>::into(err));
                        if data.is_err() {
                            has_error = true;
                            if !has_fallback {
                                return Err(BrigidError::Xff(
                                    "File is not a valid Brigid CSV array".to_string(),
                                ));
                            }
                        }
                        if has_fallback && has_error {
                            return Ok(file.get_fallback().expect("Fallback is Some").into_xff());
                        }
                        return Ok(data.unwrap());
                    }
                    DataType::Csv => {
                        let data = mawu::read::csv_headless(path)
                            .map_err(|err| Into::<BrigidError>::into(err))?;
                        let xff = data
                            .to_csv_array()
                            .ok_or(BrigidError::Csv("File is not a CSV array".to_string()));
                        if xff.is_err() {
                            has_error = true;
                            if !has_fallback {
                                return Err(BrigidError::Csv(
                                    "File is not a valid Brigid CSV array".to_string(),
                                ));
                            }
                        }
                        let xff = xff.unwrap();
                        if xff.len() != 1 {
                            has_error = true;
                            if !has_fallback {
                                return Err(BrigidError::Csv(
                                    "File is not a valid Brigid CSV array".to_string(),
                                ));
                            }
                        } else if xff[0].len() != 1 {
                            has_error = true;
                            if !has_fallback {
                                return Err(BrigidError::Csv(
                                    "File is not a valid Brigid CSV array".to_string(),
                                ));
                            }
                        }
                        if has_fallback && has_error {
                            return Ok(file.get_fallback().expect("Fallback is Some").into_xff());
                        }
                        return Ok(xff[0][0].clone());
                    }
                    DataType::Json => {
                        let data =
                            mawu::read::json(path).map_err(|err| Into::<BrigidError>::into(err));
                        if data.is_err() {
                            has_error = true;
                            if !has_fallback {
                                return Err(BrigidError::Json(
                                    "File is not a valid Brigid JSON array".to_string(),
                                ));
                            }
                        }
                        if has_fallback && has_error {
                            return Ok(file.get_fallback().expect("Fallback is Some").into_xff());
                        }
                        return Ok(data.unwrap());
                    }
                }
            }
            if has_fallback && has_error {
                return Ok(file.get_fallback().expect("Fallback is Some").into_xff());
            }
        }
        Err(BrigidError::FileNotFound(name.to_string()))
    }
    /// Get the raw bytes of a file
    pub fn get_raw_file(&self, name: &str) -> BrigidResult<Vec<u8>> {
        let path = self.file_path_getter(self.file_getter(name)?)?;
        return std::fs::read(path).map_err(|err| err.into());
    }
    fn file_path_getter(&self, file: &BrigidFile) -> BrigidResult<PathBuf> {
        if let Some(path) = file.path() {
            return Ok(path.to_path_buf());
        } else {
            return Err(BrigidError::FileNotFound(file.name().to_string()));
        }
    }
    fn file_getter(&self, name: &str) -> BrigidResult<&BrigidFile> {
        self.root_directory
            .get_file(name)
            .ok_or(BrigidError::FileNotFound(name.to_string()))
    }
    /// Delete all files and directories contained in the root of Brigid
    ///
    /// Use with caution - Will attempt to delete all files and directories contained in the root
    /// and the root itself
    pub fn delete_all(&self) -> BrigidResult<()> {
        if let Err(err) = std::fs::remove_dir_all(&self.root) {
            return Err(err.into());
        }
        Ok(())
    }
}
