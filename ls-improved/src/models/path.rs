/// Define LsiPath, LsiPathKind models.

use std::cmp::Ordering;
use std::path::PathBuf;
use anyhow::Result;


#[derive(Eq)]
pub enum LsiPathKind {
    Dir,
    File,
}

#[derive(Eq)]
pub struct LsiPath {
    path: PathBuf,
    description: Option<String>,
    pub kind: LsiPathKind,
}

impl LsiPath {
    pub fn new(path: PathBuf) -> LsiPath {
        match path.is_dir() {
            true => LsiPath {
                path: path,
                description: None,
                kind: LsiPathKind::Dir,
            },
            false => LsiPath {
                path: path,
                description: None,
                kind: LsiPathKind::File,
            },
        }
    }

    pub fn is_hidden(path: &PathBuf) -> bool {
        matches!(
            path.file_name().unwrap()
                .to_str().unwrap()
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

    pub fn absolute_path(&self) -> Result<String> {
        Ok(self.get_path()
            .canonicalize()?
            .to_str().unwrap().to_string())
    }

    pub fn set_description(&mut self, _description: String) {
        self.description = Some(_description);
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }
    
    fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn len(&self) -> usize {
        self.file_name().chars().count()
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

impl PartialEq<LsiPathKind> for LsiPathKind {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

fn add_prefix_number_to_name_for_ordering(path1: &LsiPath, path2: &LsiPath) -> (String, String) {
        let name1 = match path1.kind {
            LsiPathKind::Dir => "0_".to_string() + path1.file_name(),
            LsiPathKind::File => "1_".to_string() + path1.file_name(),
        };
        let name2 = match path2.kind {
            LsiPathKind::Dir => "0_".to_string() + path2.file_name(),
            LsiPathKind::File => "1_".to_string() + path2.file_name(),
        };
        (name1, name2)
}
