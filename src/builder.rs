use std::path::PathBuf;

use athena::process::{IoNiceClass, SchedulerPolicy};

use crate::{
    Brigid,
    directory::BrigidDirectory,
    error::{BrigidError, BrigidResult},
    file::BrigidFile,
    sys_warning::SystemWarning,
};

/// Builder for creating and establishing a `Brigid` instance.
pub struct BrigidBuilder {
    /// Path to the root directory
    root_path: PathBuf,
    /// Structure of the root directory
    root_directory: BrigidDirectory,
    /// Optional nice value for the process (0-20, default 19)
    nice_value: Option<u8>,
    /// Optional I/O scheduling policy
    io_policy: Option<IoNiceClass>,
    /// Optional CPU scheduler policy
    scheduler_policy: Option<SchedulerPolicy>,
    /// List of license files to persist (source, target)
    license_info: Vec<(PathBuf, PathBuf)>,
    /// Warnings collected during the build process
    warnings: Vec<SystemWarning>,
}

impl BrigidBuilder {
    /// Create a new `BrigidBuilder` with the specified root path.
    #[must_use]
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        let root_path = root.into();
        Self {
            root_directory: BrigidDirectory::new(
                root_path
                    .to_str()
                    .expect("Failed to convert root path to string"),
            ),
            root_path,
            nice_value: None,
            io_policy: None,
            scheduler_policy: None,
            license_info: Vec::new(),
            warnings: Vec::new(),
        }
    }
    /// Set the nice value for the process (0-20).
    #[must_use]
    pub fn with_priority(mut self, nice_value: u8) -> Self {
        if nice_value > 20 {
            self.warnings
                .push(SystemWarning::PriorityTooHigh(nice_value));
            return self;
        }
        self.nice_value = Some(nice_value);
        self
    }
    /// Set the I/O scheduling policy.
    #[must_use]
    pub fn with_io_policy(mut self, io_policy: IoNiceClass) -> Self {
        self.io_policy = Some(io_policy);
        self
    }
    /// Set the CPU scheduler policy.
    #[must_use]
    pub fn with_scheduler_policy(mut self, scheduler_policy: SchedulerPolicy) -> Self {
        self.scheduler_policy = Some(scheduler_policy);
        self
    }
    /// Set a license file to be copied during establishment.
    ///
    /// # Arguments
    /// * `license_path` - Path to the license file on disk.
    /// * `target_path` - Path where the license file should be copied (e.g., `/usr/share/licenses/myapp/copyright`).
    #[must_use]
    pub fn add_license<P: Into<PathBuf>, T: Into<PathBuf>>(
        mut self,
        license_path: P,
        target_path: T,
    ) -> Self {
        let license_path = license_path.into();
        if !license_path.is_file() {
            self.warnings
                .push(SystemWarning::LicenseSourceNotFound(license_path));
            return self;
        }
        self.license_info.push((license_path, target_path.into()));
        self
    }
    /// Define a file in the root directory.
    #[must_use]
    pub fn file(mut self, name: &str, file: impl FnOnce(&mut BrigidFile)) -> Self {
        self.root_directory.file(name, file);
        self
    }
    /// Define a subdirectory in the root directory.
    #[must_use]
    pub fn directory(mut self, name: &str, dir: impl FnOnce(&mut BrigidDirectory)) -> Self {
        self.root_directory.directory(name, dir);
        self
    }
    /// Establish the directory structure and apply configurations.
    ///
    /// # Errors
    /// Returns a `BrigidError` if directories cannot be created or files cannot be saved.
    pub fn establish(mut self) -> BrigidResult<Brigid> {
        if !self.root_path.exists() {
            std::fs::create_dir_all(&self.root_path).map_err(BrigidError::Io)?;
        }

        self.root_directory.establish(&self.root_path)?;

        for (src, dst) in self.license_info {
            if let Err(err) = persist_license(&src, &dst) {
                self.warnings
                    .push(SystemWarning::UnableToPersistLicenses(err.to_string()));
            }
        }

        self.warnings.extend(process_setup(
            self.io_policy,
            self.scheduler_policy,
            self.nice_value,
        ));

        Ok(Brigid {
            root: self.root_path,
            system_warnings: self.warnings,
            root_directory: self.root_directory,
        })
    }
}

fn process_setup(
    io_policy: Option<IoNiceClass>,
    scheduler_policy: Option<SchedulerPolicy>,
    nice_value: Option<u8>,
) -> Vec<SystemWarning> {
    let mut warnings = Vec::new();
    let nice_value = nice_value.unwrap_or(19);
    if nice_value > 20 {
        warnings.push(SystemWarning::PriorityTooHigh(nice_value));
    }
    if let Some(policy) = scheduler_policy {
        if let Err(err) = athena::process::set_scheduler(policy, nice_value as i32) {
            warnings.push(SystemWarning::UnableToSetSchedulerPolicy(err.to_string()));
        }
    }
    if let Some(policy) = io_policy {
        if let Err(err) = athena::process::set_ionice_value(policy, nice_value as u32) {
            warnings.push(SystemWarning::UnableToSetIoPolicy(err.to_string()));
        }
    }
    if let Err(err) = athena::process::set_nice_value(nice_value as i32) {
        warnings.push(SystemWarning::UnableToSetNiceValue(err.to_string()));
    }
    warnings
}

fn persist_license(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    if let Some(parent) = dst.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }
    std::fs::copy(src, dst)?;
    Ok(())
}
