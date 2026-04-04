use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum SystemWarning {
    LicenseFileExists(PathBuf),
    PriorityTooHigh(u8),
}
