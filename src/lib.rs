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
    builder::BrigidBuilder, directory::BrigidDirectory, error::BrigidResult,
    sys_warning::SystemWarning,
};

pub struct Brigid {
    root: PathBuf,
    system_warnings: Vec<SystemWarning>,
    root_directory: BrigidDirectory,
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
        todo!()
    }
    /// Get the raw bytes of a file
    pub fn get_raw_file(&self, name: &str) -> BrigidResult<Vec<u8>> {
        todo!()
    }
    /// Delete all files and directories contained in the root of Brigid
    pub fn delete_all(&self) -> BrigidResult<()> {
        todo!()
    }
}
