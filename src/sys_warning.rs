use std::path::PathBuf;

pub enum SystemWarning {
    LicenseFileExists(PathBuf),
}
