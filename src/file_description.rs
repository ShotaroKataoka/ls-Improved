use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::io::{BufReader, Read};

#[derive(Debug, Deserialize)]
pub struct FileDescriptions {
    pub files: Option<HashMap<String, String>>,
}

fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(|f| BufReader::new(f))
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

pub fn read_file_description(path: String) -> Option<FileDescriptions> {
    let s = match read_file(path) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };

    let file_description: Result<FileDescriptions, toml::de::Error> = toml::from_str(&s);
    match file_description {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}
