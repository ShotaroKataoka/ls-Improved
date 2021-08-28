use std::collections::HashMap;

pub struct Colors<'a> {
    pub red: &'a str,
    pub blue: &'a str,
    pub green: &'a str,
    pub white: &'a str,
    pub purple: &'a str,
    pub yellow: &'a str,
    pub cyan: &'a str,
    pub underline: &'a str,
    pub end: &'a str,
    pub dir: &'a str,
    pub current_dir: &'a str,
    pub file: &'a str,
    pub description: &'a str,
}

impl Colors<'_> {
    pub fn new(cfg: Option<&str>) -> Colors {
        match cfg {
            Some(file) => Colors::from_cfg(file),
            None => Colors::default()
        }
    }

    fn from_cfg(cfg: &str) -> Colors {
        Colors::default()
    }

    fn default() -> Colors<'static> {
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
            "file" => "",
            "description" => "\x1b[33m",
            _ => "",
        }
    }

    fn create<'a>(ansi: HashMap<&str, &'a str>) -> Colors<'a> {
        Colors {
            red: ansi["red"],
            blue: ansi["blue"],
            green: ansi["green"],
            white: ansi["white"],
            purple: ansi["purple"],
            yellow: ansi["yellow"],
            cyan: ansi["cyan"],
            underline: ansi["underline"],
            end: ansi["end"],
            dir: ansi["dir"],
            current_dir: ansi["current_dir"],
            file: ansi["file"],
            description: ansi["description"],
        }
    }
}
