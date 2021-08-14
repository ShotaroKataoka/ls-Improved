pub mod errors;

use std::cmp::Ordering;
use std::path::PathBuf;

#[derive(Eq)]
pub enum LsiPath {
    Dir { path: PathBuf },
    File { path: PathBuf },
}

impl LsiPath {
    pub fn new(path: PathBuf) -> LsiPath {
        match path.is_dir() {
            true => LsiPath::Dir { path },
            false => LsiPath::File { path },
        }
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

    pub fn file_name(&self) -> &str {
        self.get_path()
            .file_name().unwrap()
            .to_str().unwrap()
    }

    fn get_path(&self) -> &PathBuf {
        match self {
            LsiPath::Dir{path} => path,
            LsiPath::File{path} => path,
        }
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
