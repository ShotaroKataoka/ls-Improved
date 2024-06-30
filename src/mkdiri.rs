//! This module provides functionality for handling the mkdiri tasks.
//! It includes functions to set or edit descriptions for directories or files,
//! and to launch text editors.

use crate::errors::LsiError;
use crate::fs::write_description;
use crate::LsiArgs;
use anyhow::Result;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Runs the mkdiri module based on the provided arguments.
///
/// This function handles the main logic of the mkdiri module. It determines
/// whether to set a new description, edit an existing one, or print an error
/// if neither is provided.
///
/// # Arguments
///
/// * `args` - A reference to the `LsiArgs` struct containing the current configuration.
///
/// # Returns
///
/// Returns a `Result` which is `Ok` if the operation is successful or an error otherwise.

pub fn run(args: &LsiArgs) -> Result<()> {
    let path = PathBuf::from(&args.path);
    mkdiri_description(&path, args.set_description, args.edit_description)
}

/// Handles the description setting or editing logic.
///
/// Depending on whether a description or editor is provided, this function
/// sets a description directly or launches an editor to edit it.
///
/// # Arguments
///
/// * `path` - A reference to the target `PathBuf`.
/// * `description` - An optional description string to set.
/// * `editor` - An optional text editor command to use.
///
/// # Returns
///
/// Returns a `Result` which is `Ok` if the operation is successful or an error otherwise.
fn mkdiri_description(path: &Path, description: Option<&str>, editor: Option<&str>) -> Result<()> {
    match description {
        Some(d) => write_description(path, d.to_string()),
        None => match editor {
            Some(e) => launch_editor(path, e),
            None => Err(LsiError::FailedLaunchEditor.into()),
        },
    }
}

/// Launches a text editor to edit the description of a directory or file.
///
/// This function attempts to launch the specified text editor with the path to
/// the description file as an argument.
///
/// # Arguments
///
/// * `path` - A reference to the target `PathBuf`.
/// * `editor` - The text editor command to launch.
///
/// # Returns
///
/// Returns a `Result` which is `Ok` if the command is launched successfully or an error otherwise.
fn launch_editor(path: &Path, editor: &str) -> Result<()> {
    let mut path = path.canonicalize()?;
    let filepath = match path.is_dir() {
        true => format!("{}/.description.lsi", path.to_str().unwrap()),
        false => {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            path.pop();
            path.push(".file_description_lsi");
            if !path.is_dir() {
                std::fs::create_dir(path.to_str().unwrap())?;
            }
            format!("{}/.{}.lsi", path.to_str().unwrap(), filename)
        }
    };
    println!("Exec: {} {}", &editor, &filepath);

    Command::new(editor).arg(filepath).exec();
    Ok(())
}
