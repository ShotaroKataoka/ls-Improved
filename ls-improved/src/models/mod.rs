pub mod errors;

use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Eq)]
pub struct LsiPath {
    path: PathBuf,
}

pub enum PathType {
    Dir,
    File,
}

impl LsiPath {
    pub fn new(path: PathBuf) -> LsiPath {
        LsiPath { path }
    }

    pub fn is_hidden(&self) -> bool {
        matches!(
            self.file_name()
                .chars()
                .take(1)
                .collect::<String>()
                .as_ref(),
            "."
        )
    }

    pub fn ptype(&self) -> PathType {
        if self.path.is_file() {
            PathType::File
        } else if self.path.is_dir() {
            PathType::Dir
        } else {
            PathType::File
        }
    }

    pub fn file_name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }
}

impl Ord for LsiPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.file_name().cmp(&other.file_name())
    }
}

impl PartialOrd for LsiPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.file_name().cmp(other.file_name()))
    }
}

impl PartialEq for LsiPath {
    fn eq(&self, other: &Self) -> bool {
        self.file_name() == other.file_name()
    }
}
