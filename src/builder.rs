use std::path::PathBuf;

use athena::process::{IoNiceClass, SchedulerPolicy};

use crate::{
    Brigid, directory::BrigidDirectory, error::BrigidResult, file::BrigidFile,
    sys_warning::SystemWarning,
};

pub struct BrigidBuilder {
    root: PathBuf,
    nice_value: Option<u8>,
    io_policy: Option<IoNiceClass>,
    scheduler_policy: Option<SchedulerPolicy>,
    /// Path to license files in the form of `(path, app_name)`
    license_paths: Vec<(PathBuf, String)>,
    all_paths: Vec<PathBuf>,
    warnings: Vec<SystemWarning>,
}

impl BrigidBuilder {
    #[must_use]
    pub fn new<P: Into<PathBuf>>(root: P) -> Self {
        Self {
            root: root.into(),
            nice_value: None,
            io_policy: None,
            scheduler_policy: None,
            license_paths: Vec::new(),
            all_paths: Vec::new(),
            warnings: Vec::new(),
        }
    }
    #[must_use]
    pub fn with_priority(mut self, nice_value: u8) -> Self {
        // All checking is done inside `.establish()`
        self.nice_value = Some(nice_value);
        self
    }
    #[must_use]
    pub fn with_io_policy(mut self, io_policy: IoNiceClass) -> Self {
        self.io_policy = Some(io_policy);
        self
    }
    #[must_use]
    pub fn with_scheduler_policy(mut self, scheduler_policy: SchedulerPolicy) -> Self {
        self.scheduler_policy = Some(scheduler_policy);
        self
    }
    /// If supplying more than one License file, make sure that the filenames themselves are unique; If the same filename is used, the later usage will be dropped
    /// All Paths are copied to `/usr/share/licenses/<app_name>/copyright` (Failure only warns)
    #[must_use]
    pub fn add_license<P: Into<PathBuf>>(mut self, license_path: P, app_name: &str) -> Self {
        let license_path = license_path.into();
        if self.all_paths.contains(&license_path) {
            // Already added; Warn during build
            self.warnings
                .push(SystemWarning::LicenseFileExists(license_path));
            return self;
        }
        let license_name = if let Some(name) = license_path.file_name() {
            name
        } else {
            return self;
        };
        // If paths are the same or if the filenames are the same
        if self.license_paths.iter().any(|(path, name)| {
            if name == app_name {
                return true;
            }
            if let Some(existing_name) = path.file_name() {
                if existing_name == license_name {
                    return true;
                }
            }
            false
        }) {
            // Warn during build
            self.warnings
                .push(SystemWarning::LicenseFileExists(license_path));
            return self;
        }

        self.all_paths.push(license_path.clone());
        self.license_paths
            .push((license_path, app_name.to_string()));
        self
    }
    #[must_use]
    pub fn file(mut self, name: &str, closure: impl FnOnce(&mut BrigidFile)) -> Self {
        todo!()
    }
    #[must_use]
    pub fn directory(mut self, name: &str, closure: impl FnOnce(&mut BrigidDirectory)) -> Self {
        todo!()
    }
    pub fn establish(self) -> BrigidResult<Brigid> {
        todo!()
    }
}
