use std::collections::HashMap;
use crate::config::ColorConf;

pub struct Colors {
    pub red: String,
    pub blue: String,
    pub green: String,
    pub white: String,
    pub purple: String,
    pub yellow: String,
    pub cyan: String,
    pub underline: String,
    pub end: String,
    pub dir: String,
    pub current_dir: String,
    pub file: String,
    pub description: String,
}

impl Colors {
    pub fn new(conf: Option<&ColorConf>) -> Colors {
        match conf {
            Some(c) => Colors::from_cfg(c),
            None => Colors::default()
        }
    }

    fn from_cfg(color_conf: &ColorConf) -> Colors {
        let terms = ["red", "blue", "green", "white", "purple", "yellow", "cyan", "underline", "end", "dir", "current_dir", "file", "description"];
        let mut ansi = HashMap::new();
        for term in terms.iter() {
            match  color_conf.get(*term).as_ref() {
                Some(c) => {
                    let term_ansi = "\x1b".to_string()+c.as_ref();
                    ansi.insert(*term, term_ansi)
                },
                None => ansi.insert(*term, Colors::default_one(term).to_string()),
            };
        }
        Colors::create(ansi)
    }

    fn default() -> Colors {
        let terms = ["red", "blue", "green", "white", "purple", "yellow", "cyan", "underline", "end", "dir", "current_dir", "file", "description"];
        let mut ansi = HashMap::new();
        for term in terms.iter() {
            ansi.insert(*term, Colors::default_one(term).to_string());
        }
        Colors::create(ansi)
    }

    fn default_one(one: &str) -> &str {
        match one {
            "red" => "\x1b[31m",
            "green" => "\x1b[32m",
            "yellow" => "\x1b[33m",
            "blue" => "\x1b[34m",
            "purple" => "\x1b[35m",
            "cyan" => "\x1b[36m",
            "white" => "\x1b[37m",
            "underline" => "\x1b[4m",
            "end" => "\x1b[0m",
            "dir" => "\x1b[36m\x1b[4m",
            "current_dir" => "",
            "file" => "\x1b[37m",
            "description" => "\x1b[33m",
            _ => "",
        }
    }

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
