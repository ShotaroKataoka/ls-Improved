use std::fs;
use std::io::{BufReader, Read};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    colors: Option<ColorConf>
}

#[derive(Debug, Deserialize)]
struct ColorConf {
    pub red: Option<String>,
    pub blue: Option<String>,
    pub green: Option<String>,
    pub white: Option<String>,
    pub purple: Option<String>,
    pub yellow: Option<String>,
    pub cyan: Option<String>,
    pub underline: Option<String>,
    pub end: Option<String>,
    pub dir: Option<String>,
    pub current_dir: Option<String>,
    pub file: Option<String>,
    pub description: Option<String>,
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

pub fn read_config(path: String) {
    let s = match read_file(path) {
        Ok(s) => s,
        Err(e) => panic!("fail to read file: {}", e),
    };

    let config: Result<Config, toml::de::Error> = toml::from_str(&s);
    match config {
        Ok(c) => println!("{:#?}", c),
        Err(e) => panic!("fail to parse toml: {}", e),
    };
}


