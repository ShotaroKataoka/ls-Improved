use crate::{decoration, fs, view, LsiArgs};
use crate::errors::LsiError;
use anyhow::Result;
use std::path::PathBuf;

pub fn run(args: &LsiArgs) -> Result<()> {
    let path = PathBuf::from(&args.path);
    if path.is_dir() {
        dir_description(&path)
    } else if path.is_file() {
        file_description(&path)
    } else {
        Err(LsiError::TestError.into())
    }
}

fn dir_description(path: &PathBuf) -> Result<()> {
    Ok(())
}

fn file_description(path: &PathBuf) -> Result<()> {
    Ok(())
}
