use thiserror::Error;

#[derive(Error, Debug)]
pub enum LsiError {
    #[error("Description not found")]
    DescriptionNotFound,
    #[error("Path not found")]
    PathNotFound,
    #[error("Failed to display")]
    FailedDisplay,
    #[error("Failed to launch editor")]
    FailedLaunchEditor,
}
