extern crate regex;
use regex::Regex;
use anyhow::Result;
use crate::models::path::LsiPath;
use crate::errors::LsiError;
use crate::colors::Colors;

pub fn replace_color_codes(pathes: &mut Vec<LsiPath>, colors: &Colors) -> Result<()> {
    for path in pathes {
        let _ = replace_lsi_color_code(&mut *path, colors);
        let _ = replace_ansi_color_code(&mut *path);
        let _ = put_start_code(&mut *path, colors);
        let _ = put_end_code(&mut *path, colors);
    }
    Ok(())
}

fn replace_lsi_color_code(path: &mut LsiPath, colors: &Colors) -> Result<()> {
    match path.get_description() {
        Some(content) => {
            let content = Regex::new(r";r;").unwrap().replace_all(&content, &colors.red).to_string();
            let content = Regex::new(r";g;").unwrap().replace_all(&content, &colors.green).to_string();
            let content = Regex::new(r";y;").unwrap().replace_all(&content, &colors.yellow).to_string();
            let content = Regex::new(r";b;").unwrap().replace_all(&content, &colors.blue).to_string();
            let content = Regex::new(r";p;").unwrap().replace_all(&content, &colors.purple).to_string();
            let content = Regex::new(r";c;").unwrap().replace_all(&content, &colors.cyan).to_string();
            let content = Regex::new(r";w;").unwrap().replace_all(&content, &colors.white).to_string();
            let content = Regex::new(r";_;").unwrap().replace_all(&content, &colors.underline).to_string();
            let content = Regex::new(r";e;").unwrap().replace_all(&content, &colors.end).to_string();
            path.set_description(content);
        },
        None => { return Err(LsiError::TestError.into()) },
    };
    Ok(())
}

fn replace_ansi_color_code(path: &mut LsiPath) -> Result<()> {
    match path.get_description() {
        Some(content) => {
            let content = Regex::new("\\\\033").unwrap().replace_all(&content, "\x1b").to_string();
            path.set_description(content);
        },
        None => { return Err(LsiError::TestError.into()) },
    };
    Ok(())
}

fn put_start_code(path: &mut LsiPath, colors: &Colors) -> Result<()> {
    match path.get_description() {
        Some(content) => { path.set_description(format!("{}{}", colors.yellow, content)); },
        None => { return Err(LsiError::TestError.into()) },
    };
    Ok(())
}

fn put_end_code(path: &mut LsiPath, colors: &Colors) -> Result<()> {
    match path.get_description() {
        Some(content) => { path.set_description(format!("{}{}", content.to_string(), colors.end)); },
        None => { return Err(LsiError::TestError.into()) },
    };
    Ok(())
}

