//! This module defines the `Colors` struct and its associated methods, which are used
//! to manage and apply ANSI color codes based on configuration settings or default values.

use crate::config::ColorConf;
use std::collections::HashMap;

/// The `Colors` struct holds ANSI color codes for various UI elements.
pub struct Colors {
    /// ANSI color code for red.
    pub red: String,
    /// ANSI color code for blue.
    pub blue: String,
    /// ANSI color code for green.
    pub green: String,
    /// ANSI color code for white.
    pub white: String,
    /// ANSI color code for purple.
    pub purple: String,
    /// ANSI color code for yellow.
    pub yellow: String,
    /// ANSI color code for cyan.
    pub cyan: String,
    /// ANSI code to apply underline.
    pub underline: String,
    /// ANSI code to reset to default color.
    pub end: String,
    /// ANSI color code for directories.
    pub dir: String,
    /// ANSI color code for the current directory.
    pub current_dir: String,
    /// ANSI color code for files.
    pub file: String,
    /// ANSI color code for descriptions.
    pub description: String,
}

impl Colors {
    /// Creates a new `Colors` instance using the provided configuration. Defaults are used if no configuration is provided.
    ///
    /// # Arguments
    ///
    /// * `conf` - An optional reference to a `ColorConf` struct containing custom configurations.
    ///
    /// # Returns
    ///
    /// * A `Colors` struct with the applied color settings.
    pub fn new(conf: Option<&ColorConf>) -> Colors {
        match conf {
            Some(c) => Colors::from_cfg(c),
            None => Colors::default(),
        }
    }

    /// Generates a `Colors` instance from the given `ColorConf` configuration.
    ///
    /// # Arguments
    ///
    /// * `color_conf` - A reference to a `ColorConf` struct with custom color settings.
    ///
    /// # Returns
    ///
    /// * A `Colors` struct initialized with settings from the configuration.
    fn from_cfg(color_conf: &ColorConf) -> Colors {
        let terms = [
            "red",
            "blue",
            "green",
            "white",
            "purple",
            "yellow",
            "cyan",
            "underline",
            "end",
            "dir",
            "current_dir",
            "file",
            "description",
        ];
        let mut ansi = HashMap::new();
        for term in terms.iter() {
            match color_conf.get(term).as_ref() {
                Some(cs) => {
                    let mut term_ansi = String::new();
                    for c in cs {
                        term_ansi.push_str(&format!("\x1b{}", c));
                    }
                    ansi.insert(*term, term_ansi);
                }
                None => {
                    ansi.insert(*term, Colors::default_one(term).to_string());
                }
            };
        }
        Colors::create(ansi)
    }

    /// Generates a `Colors` instance using default color settings.
    ///
    /// # Returns
    ///
    /// * A `Colors` struct initialized with default settings.
    fn default() -> Colors {
        let terms = [
            "red",
            "blue",
            "green",
            "white",
            "purple",
            "yellow",
            "cyan",
            "underline",
            "end",
            "dir",
            "current_dir",
            "file",
            "description",
        ];
        let mut ansi = HashMap::new();
        for term in terms.iter() {
            ansi.insert(*term, Colors::default_one(term).to_string());
        }
        Colors::create(ansi)
    }

    /// Retrieves a default ANSI code string for a given term.
    ///
    /// # Arguments
    ///
    /// * `one` - The name of the term to get a default value for.
    ///
    /// # Returns
    ///
    /// * A string slice containing the default ANSI code for the term.
    fn default_one(one: &str) -> &str {
        match one {
            "red" => "\x1b[1;31m",
            "green" => "\x1b[1;32m",
            "yellow" => "\x1b[33m",
            "blue" => "\x1b[1;34m",
            "purple" => "\x1b[1;35m",
            "cyan" => "\x1b[36m",
            "white" => "\x1b[37m",
            "underline" => "\x1b[4m",
            "end" => "\x1b[0m",
            "dir" => "\x1b[36m\x1b[4m",
            "current_dir" => "\x1b[1;36m\x1b[4m",
            "file" => "\x1b[37m",
            "description" => "\x1b[33m",
            _ => "",
        }
    }

    /// Creates a `Colors` instance from a given map of terms to ANSI codes.
    ///
    /// # Arguments
    ///
    /// * `ansi` - A `HashMap` mapping term names to their respective ANSI codes.
    ///
    /// # Returns
    ///
    /// * A `Colors` struct initialized with the values from the hashmap.
    fn create(ansi: HashMap<&str, String>) -> Colors {
        Colors {
            red: ansi["red"].clone(),
            blue: ansi["blue"].clone(),
            green: ansi["green"].clone(),
            white: ansi["white"].clone(),
            purple: ansi["purple"].clone(),
            yellow: ansi["yellow"].clone(),
            cyan: ansi["cyan"].clone(),
            underline: ansi["underline"].clone(),
            end: ansi["end"].clone(),
            dir: ansi["dir"].clone(),
            current_dir: ansi["current_dir"].clone(),
            file: ansi["file"].clone(),
            description: ansi["description"].clone(),
        }
    }
}
