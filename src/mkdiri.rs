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
        file_description(&path)
    } else {
        Err(LsiError::TestError.into())
    }
}

fn dir_description(path: &PathBuf, description: Option<&str>, editor: Option<&str>) -> Result<()> {
    let content: Option<String> = match description {
        Some(d) => {
            Some(d.to_string())
        },
        None => {
            match editor {
                Some(e) => from_editor(e),
                None => None
            }
        },
    };
    match content {
        Some(c) => write_dir_description(path, c),
        None => Ok(()),
    }
}

fn from_editor(editor: &str) -> Option<String> {
    // Unix Only
    let filename = "./lsi-editor-test.txt";

    let _ = Command::new(editor)
            .arg(filename)
            .exec();
    println!("done.");
    None
}

fn file_description(path: &PathBuf) -> Result<()> {
    Ok(())
}
