//! This module handles the configuration of the application. It includes structures for 
//! configuration settings and functions for reading and deserializing configuration files.

use serde::Deserialize;
use std::fs;
use std::io::{BufReader, Read};

/// Struct representing the overall configuration of the application.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Optional color configuration settings.
    pub colors: Option<ColorConf>,
}

/// Struct representing the color configuration settings.
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

/// Reads the content of a file and returns it as a string.
///
/// # Arguments
///
/// * `path` - A string representing the path to the file.
///
/// # Returns
///
/// * A result containing the file content as a string, or an error message.
fn read_file(path: String) -> Result<String, String> {
    let mut file_content = String::new();

    let mut fr = fs::File::open(path)
        .map(BufReader::new)
        .map_err(|e| e.to_string())?;

    fr.read_to_string(&mut file_content)
        .map_err(|e| e.to_string())?;

    Ok(file_content)
}

/// Reads and parses the configuration file from the given path.
///
/// # Arguments
///
/// * `path` - A string representing the path to the configuration file.
///
/// # Returns
///
/// * An option containing the parsed `Config` struct, or `None` if the parsing fails.
pub fn read_config(path: String) -> Option<Config> {
    let s = match read_file(path) {
        Ok(s) => s,
        Err(_) => String::new(),
    };

    toml::from_str(&s).ok()
}

impl ColorConf {
    /// Retrieves the color configuration for a given key.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice representing the color configuration key.
    ///
    /// # Returns
    ///
    /// * A reference to an option containing a vector of strings with ANSI codes, or `None` if the key does not exist.
    pub fn get(&self, key: &str) -> &Option<Vec<String>> {
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