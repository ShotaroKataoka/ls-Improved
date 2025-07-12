//! This module defines custom error types for use within the application.
//!
//! The `LsiError` enum encapsulates various error conditions that might occur
//! during the execution of the program, providing descriptive messages for each.

use thiserror::Error;

/// Represents the custom error types for the application.
#[derive(Error, Debug)]
pub enum LsiError {
    /// Error indicating that a requested description was not found.
    #[error("Description not found for the specified path")]
    DescriptionNotFound,

    /// Error indicating that a specified path was not found.
    #[error("Path not found: The specified path does not exist or is inaccessible")]
    PathNotFound,

    /// Error indicating that a display operation failed.
    #[error("Failed to display output: {0}")]
    FailedDisplay(String),

    /// Error indicating that launching an external editor failed.
    #[error("Failed to launch editor: {0}")]
    FailedLaunchEditor(String),
    
    /// Error indicating an invalid path format.
    #[error("Invalid path format: The path contains invalid characters or is malformed")]
    InvalidPath,
    
    /// Error indicating a file operation failure.
    #[error("File operation failed: {0}")]
    FileOperationFailed(String),
    
    /// Error indicating a permission issue.
    #[error("Permission denied: Insufficient permissions to access {0}")]
    PermissionDenied(String),
    
    /// Error indicating an I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}
