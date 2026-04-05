use std::path::PathBuf;

/// Represents non-fatal warnings that can occur during system operations
#[derive(Debug, Clone)]
pub enum SystemWarning {
    /// The process priority (nice value) is too high
    PriorityTooHigh(u8),
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
