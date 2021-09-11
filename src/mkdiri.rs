use crate::{decoration, fs, view, LsiArgs};
use crate::errors::LsiError;
use anyhow::Result;
use std::path::PathBuf;
use crate::fs::write_dir_description;

pub fn run(args: &LsiArgs) -> Result<()> {
    let path = PathBuf::from(&args.path);
    if path.is_dir() {
        dir_description(&path, args.set_description)
    } else if path.is_file() {
        file_description(&path)
    } else {
        Err(LsiError::TestError.into())
    }
}

fn dir_description(path: &PathBuf, description: Option<&str>) -> Result<()> {
    let content: Option<String> = match description {
        Some(d) => {
            Some(d.to_string())
        },
        None => {
            None
        },
    };
    match content {
        Some(c) => write_dir_description(path, c),
        None => Ok(()),
    }
}

fn file_description(path: &PathBuf) -> Result<()> {
    Ok(())
}
