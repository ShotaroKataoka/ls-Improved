use std::collections::HashMap;

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
    pub fn new(path: Option<&str>) -> Colors {
        match path {
            Some(p) => Colors::from_cfg(p),
            None => Colors::default()
        }
    }

    fn from_cfg(path: &str) -> Colors {
        Colors::default()
    }

    fn default() -> Colors {
        let terms = ["red", "blue", "green", "white", "purple", "yellow", "cyan", "underline", "end", "dir", "current_dir", "file", "description"];
        let mut ansi = HashMap::new();
        for term in terms.iter() {
            ansi.insert(*term, Colors::default_one(term));
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

    fn create(ansi: HashMap<&str, &str>) -> Colors {
        Colors {
            red: ansi["red"].to_string(),
            blue: ansi["blue"].to_string(),
            green: ansi["green"].to_string(),
            white: ansi["white"].to_string(),
            purple: ansi["purple"].to_string(),
            yellow: ansi["yellow"].to_string(),
            cyan: ansi["cyan"].to_string(),
            underline: ansi["underline"].to_string(),
            end: ansi["end"].to_string(),
            dir: ansi["dir"].to_string(),
            current_dir: ansi["current_dir"].to_string(),
            file: ansi["file"].to_string(),
            description: ansi["description"].to_string(),
        }
    }
}
