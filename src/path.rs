use anyhow::Result;
use regex::Regex;
/// Define LsiPath, LsiPathKind models.
use std::cmp::Ordering;
use std::path::PathBuf;
use unicode_width::UnicodeWidthStr;

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
    sort_mode: String,
}

impl LsiPath {
    pub fn new(path: PathBuf, sort_mode: &str) -> LsiPath {
        let flag = path.is_dir();
        LsiPath {
            path: path,
            description: None,
            kind: if flag {
                LsiPathKind::Dir
            } else {
                LsiPathKind::File
            },
            sort_mode: sort_mode.to_string(),
        }
    }

    pub fn is_hidden(path: &PathBuf) -> bool {
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

    pub fn file_name(&self) -> &str {
        self.get_path().file_name().unwrap().to_str().unwrap()
    }

    pub fn absolute_path(&self) -> Result<String> {
        Ok(self
            .get_path()
            .canonicalize()?
            .to_str()
            .unwrap()
            .to_string())
    }

    pub fn set_description(&mut self, _description: String) {
        self.description = Some(_description);
    }

    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }

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

    pub fn get_sort_mode(&self) -> &str {
        &self.sort_mode
    }

    fn get_path(&self) -> &PathBuf {
        &self.path
    }

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
        let (name1, name2) = format_for_eq(self, other);
        Some(name1.cmp(&name2))
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
        self == other
    }
}

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

fn format_name(path: &LsiPath) -> String {
    match path.kind {
        LsiPathKind::Dir => "0_".to_string() + path.file_name(),
        LsiPathKind::File => "1_".to_string() + path.file_name(),
    }
}

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
