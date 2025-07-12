//! This module provides types and functions for handling file paths and their metadata.
//! It includes functionality for path comparison, sorting, and description management.

use anyhow::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use unicode_width::UnicodeWidthStr;

/// Represents the types of paths (files or directories).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LsiPathKind {
    /// Represents a directory path
    Dir,
    /// Represents a file path
    File,
}

/// Represents a path along with its metadata and sorting mode.
#[derive(Debug, Eq)]
pub struct LsiPath {
    /// The underlying path
    path: PathBuf,
    /// Optional description for the path
    description: Option<String>,
    /// The kind of path (file or directory)
    pub kind: LsiPathKind,
    /// The mode used for sorting
    sort_mode: String,
}

impl LsiPath {
    /// Creates a new `LsiPath` instance.
    ///
    /// # Arguments
    ///
    /// * `path` - The PathBuf representing the file or directory path.
    /// * `sort_mode` - The mode used for sorting.
    ///
    /// # Returns
    ///
    /// A `LsiPath` instance.
    pub fn new(path: PathBuf, sort_mode: &str) -> Self {
        let kind = if path.is_dir() {
            LsiPathKind::Dir
        } else {
            LsiPathKind::File
        };
        
        Self {
            path,
            description: None,
            kind,
            sort_mode: sort_mode.to_string(),
        }
    }

    /// Determines if a given path is hidden.
    ///
    /// # Arguments
    ///
    /// * `path` - The Path to be checked.
    ///
    /// # Returns
    ///
    /// `true` if the path is hidden, `false` otherwise.
    pub fn is_hidden(path: &Path) -> bool {
        path.file_name()
            .and_then(|name| name.to_str())
            .map(|name| name.starts_with('.'))
            .unwrap_or(false)
    }

    /// Retrieves the file name of the path as a string slice.
    ///
    /// # Returns
    ///
    /// The file name as a string slice, or an empty string if the file name is invalid.
    pub fn file_name(&self) -> &str {
        self.path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("")
    }

    /// Gets the absolute path as a String.
    ///
    /// # Returns
    ///
    /// A Result containing the absolute path as a String.
    pub fn absolute_path(&self) -> Result<String> {
        Ok(self.path
            .canonicalize()?
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid path encoding"))?
            .to_string())
    }

    /// Sets the description of the path.
    ///
    /// # Arguments
    ///
    /// * `description` - The description to be set.
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }

    /// Gets the description of the path.
    ///
    /// # Returns
    ///
    /// The description as an Option containing a String.
    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

    /// Gets the plain description (without special characters) of the path.
    ///
    /// # Returns
    ///
    /// The plain description as an Option containing a String.
    pub fn get_plain_description(&self) -> Option<String> {
        self.description.as_ref().map(|d| {
            let content = Regex::new(r";.;")
                .unwrap_or_else(|_| Regex::new(r"").unwrap())
                .replace_all(d, "")
                .to_string();
            
            Regex::new("\\\\033")
                .unwrap_or_else(|_| Regex::new(r"").unwrap())
                .replace_all(&content, "")
                .to_string()
        })
    }

    /// Gets the sort mode of the path.
    ///
    /// # Returns
    ///
    /// The sort mode as a string slice.
    pub fn get_sort_mode(&self) -> &str {
        &self.sort_mode
    }

    /// Gets the length of the file name (in Unicode width).
    ///
    /// # Returns
    ///
    /// The length of the file name.
    pub fn len(&self) -> usize {
        UnicodeWidthStr::width(self.file_name())
    }
    
    /// Returns whether the path is empty.
    ///
    /// # Returns
    ///
    /// `true` if the path's file name has zero length, `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Ord for LsiPath {
    fn cmp(&self, other: &Self) -> Ordering {
        let (name1, name2) = format_for_eq(self, other);
        name1.cmp(&name2)
    }
}

impl PartialOrd for LsiPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for LsiPath {
    fn eq(&self, other: &Self) -> bool {
        let (name1, name2) = format_for_eq(self, other);
        name1 == name2
    }
}

/// Formats the paths for comparison.
///
/// # Arguments
///
/// * `path1` - The first LsiPath to be compared.
/// * `path2` - The second LsiPath to be compared.
///
/// # Returns
///
/// A tuple containing the formatted names for comparison.
fn format_for_eq(path1: &LsiPath, path2: &LsiPath) -> (String, String) {
    match path1.get_sort_mode() {
        "d" => {
            let name1 = format_description(path1);
            let name2 = format_description(path2);
            (name1, name2)
        }
        _ => {
            let name1 = format_name(path1);
            let name2 = format_name(path2);
            (name1, name2)
        }
    }
}

/// Formats the name of the path for comparison.
///
/// # Arguments
///
/// * `path` - The LsiPath to be formatted.
///
/// # Returns
///
/// The formatted name as a String.
fn format_name(path: &LsiPath) -> String {
    match path.kind {
        LsiPathKind::Dir => format!("0_{}", path.file_name()),
        LsiPathKind::File => format!("1_{}", path.file_name()),
    }
}

/// Formats the description of the path for comparison.
///
/// # Arguments
///
/// * `path` - The LsiPath to be formatted.
///
/// # Returns
///
/// The formatted description as a String.
fn format_description(path: &LsiPath) -> String {
    match path.get_plain_description() {
        Some(desc) => match path.kind {
            LsiPathKind::Dir => format!("0_0_{}{}", desc, path.file_name()),
            LsiPathKind::File => format!("1_0_{}{}", desc, path.file_name()),
        },
        None => match path.kind {
            LsiPathKind::Dir => format!("0_1_{}", path.file_name()),
            LsiPathKind::File => format!("1_1_{}", path.file_name()),
        },
    }
}
