use std::fs;
use std::io::{BufReader, Read};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub colors: Option<ColorConf>
}

#[derive(Debug, Deserialize)]
pub struct ColorConf {
    pub red: Option<Vec<String>>,
    pub blue: Option<Vec<String>>,
    pub green: Option<Vec<String>>,
    pub white: Option<Vec<String>>,
    pub purple: Option<Vec<String>>,
    pub yellow: Option<Vec<String>>,
    pub cyan: Option<Vec<String>>,
    pub underline: Option<Vec<String>>,
    pub end: Option<Vec<String>>,
    pub dir: Option<Vec<String>>,
    pub current_dir: Option<Vec<String>>,
    pub file: Option<Vec<String>>,
    pub description: Option<Vec<String>>,
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

pub fn read_config(path: String) -> Option<Config>{
    let s = match read_file(path) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };

    let config: Result<Config, toml::de::Error> = toml::from_str(&s);
    match config {
        Ok(c) => Some(c),
        Err(_) => None,
    }
}

impl ColorConf {
    pub fn get(&self, key: &str) -> &Option<Vec<String>>{
        match key {
            "red" => &self.red,
            "blue" => &self.blue,
            "green" => &self.green,
            "white" => &self.white,
            "purple" => &self.purple,
            "yellow" => &self.yellow,
            "cyan" => &self.cyan,
            "underline" => &self.underline,
            "end" => &self.end,
            "dir" => &self.dir,
            "current_dir" => &self.current_dir,
            "file" => &self.file,
            "description" => &self.description,
            _ => &None,
        }
    }
}
