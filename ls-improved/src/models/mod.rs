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
        let (name1, name2) = add_prefix_number_to_name_for_ordering(self, other);
        name1.cmp(&name2)
    }
}

impl PartialOrd for LsiPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (name1, name2) = add_prefix_number_to_name_for_ordering(self, other);
        Some(name1.cmp(&name2))
    }
}

impl PartialEq for LsiPath {
    fn eq(&self, other: &Self) -> bool {
        let (name1, name2) = add_prefix_number_to_name_for_ordering(self, other);
        name1 == name2
    }
}

fn add_prefix_number_to_name_for_ordering(name1: &LsiPath, name2: &LsiPath) -> (String, String) {
        let name1 = match name1 {
            LsiPath::Dir{..} => "0".to_string() + name1.file_name(),
            LsiPath::File{..} => "1".to_string() + name1.file_name(),
        };
        let name2 = match name2 {
            LsiPath::Dir{..} => "0".to_string() + name2.file_name(),
            LsiPath::File{..} => "1".to_string() + name2.file_name(),
        };
        (name1, name2)
}
