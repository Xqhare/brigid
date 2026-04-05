use std::path::PathBuf;

/// Represents non-fatal warnings that can occur during system operations
#[derive(Debug, Clone)]
pub enum SystemWarning {
    /// The process priority (nice value) is too high
    PriorityTooHigh(i8),
    /// The process priority (nice value) is too low
    PriorityTooLow(i8),
    /// An error occurred while persisting a license file
    UnableToPersistLicenses(String),
    /// An error occurred while setting the scheduler policy
    UnableToSetSchedulerPolicy(String),
    /// An error occurred while setting the I/O policy
    UnableToSetIoPolicy(String),
    /// An error occurred while setting the nice value
    UnableToSetNiceValue(String),
    /// The source license file was not found
    LicenseSourceNotFound(PathBuf),
}

impl std::fmt::Display for SystemWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemWarning::PriorityTooHigh(val) => write!(f, "Priority too high: {val}. Max is 19"),
            SystemWarning::PriorityTooLow(val) => write!(f, "Priority too low: {val}. Min is -20"),
            SystemWarning::UnableToPersistLicenses(msg) => {
                write!(f, "Unable to persist licenses: {msg}")
            }
            SystemWarning::UnableToSetSchedulerPolicy(msg) => {
                write!(f, "Unable to set scheduler policy: {msg}")
            }
            SystemWarning::UnableToSetIoPolicy(msg) => write!(f, "Unable to set I/O policy: {msg}"),
            SystemWarning::UnableToSetNiceValue(msg) => {
                write!(f, "Unable to set nice value: {msg}")
            }
            SystemWarning::LicenseSourceNotFound(path) => {
                write!(f, "License source not found: {path:?}")
            }
        }
    }
}
