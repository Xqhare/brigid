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
mod content;
mod directory;
mod error;
mod file;
mod sys_warning;
use std::path::PathBuf;

use athena::XffValue;
/// Remember: IoNiceClass and SchedulerPolicy need to be exposed publicly, rest not
pub use athena::process::{IoNiceClass, SchedulerPolicy};

use crate::{builder::BrigidBuilder, error::BrigidResult, sys_warning::SystemWarning};

pub struct Brigid {
    root: PathBuf,
    system_warnings: Vec<SystemWarning>,
}

impl Brigid {
    pub fn new<P: Into<PathBuf>>(root: P) -> BrigidBuilder {
        BrigidBuilder::new(root)
    }
    pub fn has_warnings(&self) -> bool {
        !self.system_warnings.is_empty()
    }
    pub fn no_warnings(&self) -> bool {
        self.system_warnings.is_empty()
    }
    pub fn get_warnings(&self) -> &Vec<SystemWarning> {
        &self.system_warnings
    }
    pub fn get_file(&self, name: &str) -> BrigidResult<XffValue> {
        todo!()
    }
    pub fn get_raw_file(&self, name: &str) -> BrigidResult<Vec<u8>> {
        todo!()
    }
}
