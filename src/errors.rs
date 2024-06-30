//! This module defines custom error types for use within the application.
//!
//! The `LsiError` enum encapsulates various error conditions that might occur
//! during the execution of the program, providing descriptive messages for each.

use thiserror::Error;

/// Represents the custom error types for the application.
#[derive(Error, Debug)]
pub enum LsiError {
    /// Error indicating that a requested description was not found.
    #[error("Description not found")]
    DescriptionNotFound,

    /// Error indicating that a specified path was not found.
    #[error("Path not found")]
    PathNotFound,

    /// Error indicating that a display operation failed.
    #[error("Failed to display")]
    FailedDisplay,

    /// Error indicating that launching an external editor failed.
    #[error("Failed to launch editor")]
    FailedLaunchEditor,
}
