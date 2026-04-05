#[derive(Debug, Clone)]
pub enum SystemWarning {
    PriorityTooHigh(u8),
    UnableToPersistLicenses(String),
    UnableToSetSchedulerPolicy(String),
    UnableToSetIoPolicy(String),
    UnableToSetNiceValue(String),
}
