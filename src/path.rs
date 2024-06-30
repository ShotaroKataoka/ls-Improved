use anyhow::Result;
use regex::Regex;
use std::cmp::Ordering;
use std::path::{Path, PathBuf};
use unicode_width::UnicodeWidthStr;

/// Represents the types of paths (files or directories).
#[derive(Eq)]
pub enum LsiPathKind {
    Dir,
    File,
}

/// Represents a path along with its metadata and sorting mode.
#[derive(Eq)]
pub struct LsiPath {
    path: PathBuf,
    description: Option<String>,
    pub kind: LsiPathKind,
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
    pub fn new(path: PathBuf, sort_mode: &str) -> LsiPath {
        let flag = path.is_dir();
        LsiPath {
            path,
            description: None,
            kind: if flag {
                LsiPathKind::Dir
            } else {
                LsiPathKind::File
            },
            sort_mode: sort_mode.to_string(),
        }
    }

    /// Determines if a given path is hidden.
    ///
    /// # Arguments
    ///
    /// * `path` - The PathBuf to be checked.
    ///
    /// # Returns
    ///
    /// `true` if the path is hidden, `false` otherwise.
    pub fn is_hidden(path: &Path) -> bool {
        matches!(
            path.file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .chars()
                .take(1)
                .collect::<String>()
                .as_ref(),
            "."
        )
    }

    /// Retrieves the file name of the path as a string slice.
    ///
    /// # Returns
    ///
    /// The file name as a string slice.
    pub fn file_name(&self) -> &str {
        self.get_path().file_name().unwrap().to_str().unwrap()
    }

    /// Gets the absolute path as a String.
    ///
    /// # Returns
    ///
    /// A Result containing the absolute path as a String.
    pub fn absolute_path(&self) -> Result<String> {
        Ok(self
            .get_path()
            .canonicalize()?
            .to_str()
            .unwrap()
            .to_string())
    }

    /// Sets the description of the path.
    ///
    /// # Arguments
    ///
    /// * `_description` - The description to be set.
    pub fn set_description(&mut self, _description: String) {
        self.description = Some(_description);
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
        match &self.description {
            Some(d) => {
                let content = Regex::new(r";.;").unwrap().replace_all(d, "").to_string();
                let content = Regex::new("\\\\033")
                    .unwrap()
                    .replace_all(&content, "")
                    .to_string();
                Some(content)
            }
            None => None,
        }
    }

    /// Gets the sort mode of the path.
    ///
    /// # Returns
    ///
    /// The sort mode as a string slice.
    pub fn get_sort_mode(&self) -> &str {
        &self.sort_mode
    }

    /// Gets the underlying PathBuf of the LsiPath.
    ///
    /// # Returns
    ///
    /// The PathBuf of the path.
    fn get_path(&self) -> &PathBuf {
        &self.path
    }

    /// Gets the length of the file name (in Unicode width).
    ///
    /// # Returns
    ///
    /// The length of the file name.
    pub fn len(&self) -> usize {
        UnicodeWidthStr::width(self.file_name())
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

impl PartialEq<LsiPathKind> for LsiPathKind {
    fn eq(&self, other: &Self) -> bool {
        match self {
            LsiPathKind::Dir => matches!(other, LsiPathKind::Dir),
            LsiPathKind::File => matches!(other, LsiPathKind::File),
        }
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
        LsiPathKind::Dir => "0_".to_string() + path.file_name(),
        LsiPathKind::File => "1_".to_string() + path.file_name(),
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
        Some(d) => match path.kind {
            LsiPathKind::Dir => format!("0_0_{}{}", d, path.file_name()),
            LsiPathKind::File => format!("1_0_{}{}", d, path.file_name()),
        },
        None => match path.kind {
            LsiPathKind::Dir => format!("0_1_{}", path.file_name()),
            LsiPathKind::File => format!("1_1_{}", path.file_name()),
        },
    }
}
