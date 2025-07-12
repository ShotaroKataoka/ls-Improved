//! ls-Improved (lsi) is a command-line tool for listing directory structures
//! along with their descriptions. It is particularly useful for managing and
//! accessing experimental results, which can often be buried deep within multiple directories.
//!
//! This library provides the core functionality for the lsi command-line tool.

pub mod colors;
pub mod config;
pub mod decoration;
pub mod errors;
pub mod fs;
pub mod lsi;
pub mod mkdiri;
pub mod path;
pub mod view;

use path::LsiPathKind;

/// This struct encapsulates command-line argument values and configurations for lsi/mkdiri.
pub struct LsiArgs<'a> {
    /// The path to list or manage.
    pub path: &'a str,
    /// Whether to show hidden files or not.
    pub show_hidden: bool,
    /// An optional filter to list only files or directories.
    pub is_only: Option<LsiPathKind>,
    /// The path to the configuration file.
    pub config_path: Option<&'a str>,
    /// An optional description number.
    pub desc_num: Option<usize>,
    /// Whether the current mode is mkdiri or not.
    pub is_mkdiri_mode: bool,
    /// The description to set (if applicable).
    pub set_description: Option<&'a str>,
    /// The description to edit (if applicable).
    pub edit_description: Option<&'a str>,
    /// The mode for sorting entries.
    pub sort_mode: String,
}
