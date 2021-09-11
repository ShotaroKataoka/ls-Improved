use crate::errors::LsiError;
use crate::path::{LsiPath, LsiPathKind};
use anyhow::Result;
/// controller/fs.rs
/// Define file/dir io.
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::io::Write;
use regex::Regex;

pub fn get_pathes(
    path: &str,
    is_only: &Option<LsiPathKind>,
    show_hidden: &bool,
) -> Result<Vec<LsiPath>> {
    let pathes = match fs::read_dir(path) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    let mut p = Vec::new();
    for path in pathes {
        let path = path.unwrap().path();
        if path_filter(&path, is_only, show_hidden) {
            let lsi_path = LsiPath::new(path);
            p.push(lsi_path);
        }
    }
    p.sort();
    Ok(p)
}

fn path_filter(path: &PathBuf, is_only: &Option<LsiPathKind>, show_hidden: &bool) -> bool {
    match is_only {
        Some(kind) => match kind {
            LsiPathKind::Dir => {
                let is_target = if path.is_dir() { true } else { false };
                let is_hidden = LsiPath::is_hidden(path);
                is_target && (!is_hidden || *show_hidden)
            }
            LsiPathKind::File => {
                let is_target = if path.is_file() { true } else { false };
                let is_hidden = LsiPath::is_hidden(path);
                is_target && (!is_hidden || *show_hidden)
            }
        },
        None => !LsiPath::is_hidden(path) || *show_hidden,
    }
}

pub fn read_dir_description(path: &LsiPath) -> Result<String> {
    let desc_path = path.absolute_path()? + "/.description.lsi";
    let desc_path = Path::new(&desc_path);
    let mut f = File::open(desc_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
}

pub fn read_file_description(path: &LsiPath) -> Result<String> {
    let mut path = PathBuf::from(path.absolute_path()?);
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    path.pop();
    let path = path.to_str().unwrap();
    let desc_path = format!("{}/.file_description_lsi/.{}.lsi", path, filename);
    println!("{}", &desc_path);

    let desc_path = Path::new(&desc_path);
    let mut f = File::open(desc_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
}

pub fn write_dir_description(path: &PathBuf, content: String) -> Result<()> {
    let content = Regex::new(r"\\n")
        .unwrap()
        .replace_all(&content, "\n")
        .to_string();

    let filename = path
        .canonicalize()?
        .to_str()
        .unwrap()
        .to_string() + "/.description.lsi";

    let mut file = match File::create(&filename) {
        Err(why) => panic!("Couldn't create {}: {}", &filename, why),
        Ok(file) => file,
    };

    match file.write_all(content.as_bytes()) {
        Err(why) => panic!("Couldn't write \"{}\" to {}: {}", content, &filename, why),
        Ok(_) => println!("Success: Write description to {}", &filename),
    }
    Ok(())
}
