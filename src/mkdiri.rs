use crate::{decoration, fs, view, LsiArgs};
use crate::errors::LsiError;
use anyhow::Result;
use std::path::PathBuf;
use crate::fs::write_dir_description;
use std::process::Command;
use std::os::unix::process::CommandExt;


pub fn run(args: &LsiArgs) -> Result<()> {
    let path = PathBuf::from(&args.path);
    if path.is_dir() {
        dir_description(&path, args.set_description, args.edit_description)
    } else if path.is_file() {
        file_description(&path, args.set_description, args.edit_description)
    } else {
        Err(LsiError::TestError.into())
    }
}

fn dir_description(path: &PathBuf, description: Option<&str>, editor: Option<&str>) -> Result<()> {
    match description {
        Some(d) => {
            write_dir_description(path, d.to_string())
        },
        None => {
            match editor {
                Some(e) => from_editor(path, e),
                None => Err(LsiError::TestError.into())
            }
        },
    }
}

fn from_editor(path: &PathBuf, editor: &str) -> Result<()> {
    let mut path = path.canonicalize()?;
    let filepath = match path.is_dir() {
        true => format!("{}/.description.lsi", path.to_str().unwrap()),
        false => {
            path.pop();
            format!("{}/.file_description.lsi", path.to_str().unwrap())
        },
    };
    println!("Edit {} with {}", &filepath, &editor);

    Command::new(editor).arg(filepath).exec();
    Ok(())
}

fn file_description(path: &PathBuf, description: Option<&str>, editor: Option<&str>) -> Result<()> {
    match description {
        Some(d) => {
            write_dir_description(path, d.to_string())
        },
        None => {
            match editor {
                Some(e) => from_editor(path, e),
                None => Err(LsiError::TestError.into())
            }
        },
    }
}
