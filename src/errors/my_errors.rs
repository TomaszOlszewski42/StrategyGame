use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyErrors {
    #[error("Inconsistent Data: {0}")]
    InconsistentData(String),
    #[error("No save folder")]
    NoSaveFolder,
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Deserialization error")]
    Deserialization,
    #[error("Serialization")]
    Serialization,
    #[error("Reading from file: {0}")]
    FileReading(String),
}