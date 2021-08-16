extern crate regex;
use regex::Regex;
use anyhow::Result;
use crate::models::{LsiPath, LsiPathKind};
use crate::models::errors::LsiError;


pub fn replace_color_codes(pathes: &mut Vec<LsiPath>) -> Result<()> {
    for path in pathes {
        replace_lsi_color_code(&mut *path);
        replace_ansi_color_code(&mut *path);
        put_start_code(&mut *path);
        put_end_code(&mut *path);
    }
    Ok(())
}

fn replace_lsi_color_code(path: &mut LsiPath) -> Result<()> {
    match path.get_description() {
        Some(content) => {
            let content = Regex::new(r";r;").unwrap().replace_all(&content, "\x1b[31m").to_string();
            let content = Regex::new(r";g;").unwrap().replace_all(&content, "\x1b[32m").to_string();
            let content = Regex::new(r";y;").unwrap().replace_all(&content, "\x1b[33m").to_string();
            let content = Regex::new(r";b;").unwrap().replace_all(&content, "\x1b[34m").to_string();
            let content = Regex::new(r";p;").unwrap().replace_all(&content, "\x1b[35m").to_string();
            let content = Regex::new(r";c;").unwrap().replace_all(&content, "\x1b[36m").to_string();
            let content = Regex::new(r";w;").unwrap().replace_all(&content, "\x1b[37m").to_string();
            let content = Regex::new(r";_;").unwrap().replace_all(&content, "\x1b[4m").to_string();
            let content = Regex::new(r";e;").unwrap().replace_all(&content, "\x1b[0m").to_string();
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

fn put_start_code(path: &mut LsiPath) -> Result<()> {
    match path.get_description() {
        Some(content) => { path.set_description("\x1b[33m".to_string()+content); },
        None => { return Err(LsiError::TestError.into()) },
    };
    Ok(())
}

fn put_end_code(path: &mut LsiPath) -> Result<()> {
    match path.get_description() {
        Some(content) => { path.set_description(content.to_string()+"\x1b[0m"); },
        None => { return Err(LsiError::TestError.into()) },
    };
    Ok(())
}

