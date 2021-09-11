use crate::{decoration, fs, view, LsiArgs};
use crate::errors::LsiError;
use anyhow::Result;
use std::path::PathBuf;
use crate::fs::write_description;
use std::process::Command;
use std::os::unix::process::CommandExt;


pub fn run(args: &LsiArgs) -> Result<()> {
    let path = PathBuf::from(&args.path);
    mkdiri_description(&path, args.set_description, args.edit_description)
}

fn mkdiri_description(path: &PathBuf, description: Option<&str>, editor: Option<&str>) -> Result<()> {
    match description {
        Some(d) => {
            write_description(path, d.to_string())
        },
        None => {
            match editor {
                Some(e) => launch_editor(path, e),
                None => Err(LsiError::TestError.into())
            }
        },
    }
}

fn launch_editor(path: &PathBuf, editor: &str) -> Result<()> {
    let mut path = path.canonicalize()?;
    let filepath = match path.is_dir() {
        true => format!("{}/.description.lsi", path.to_str().unwrap()),
        false => {
            path.pop();
            format!("{}/.file_description.lsi", path.to_str().unwrap())
        },
    };
    println!("Edit {} by {}", &filepath, &editor);

    Command::new(editor).arg(filepath).exec();
    Ok(())
}

