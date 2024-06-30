//! This module contains the main logic for the lsi application.
//! It handles file and directory listing, reading configurations,
//! fetching descriptions, and displaying the results in the desired format.

extern crate exitcode;
use crate::colors::Colors;
use crate::config::read_config;
use crate::errors::LsiError;
use crate::path::{LsiPath, LsiPathKind};
use crate::{fs, view, LsiArgs};
use anyhow::Result;

/// Executes the main functionality of the `lsi` application.
///
/// This function:
/// - Globs target files and directories based on provided arguments
/// - Reads configuration settings
/// - Sets file and directory descriptions
/// - Displays the results in a structured format
///
/// # Parameters
///
/// - `args`: A reference to `LsiArgs` containing the configuration and command-line parameters
///
/// # Returns
///
/// - `Result<()>`: Success returns `Ok(())`, while any errors return a relevant `anyhow::Error`
///
/// # Errors
///
/// - `LsiError::PathNotFound`: If the specified path cannot be found
/// - `LsiError::FailedDisplay`: If the display function encounters an error

pub fn run(args: &LsiArgs) -> Result<()> {
    // Glob target files and directories
    let mut pathes = match fs::get_pathes(
        &args.path,
        &args.is_only,
        &args.show_hidden,
        &args.sort_mode,
    ) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::PathNotFound.into()),
    };

    // Read Configs
    let config = match &args.config_path {
        Some(path) => read_config(path.to_string()),
        None => None,
    };
    let colors_config = match config {
        Some(ref c) => c.colors.as_ref(),
        None => None,
    };
    let colors = Colors::new(colors_config);

    // Read and set descriptions
    get_and_set_descriptions(&mut pathes)?;

    // Display LSI results
    match view::display(&mut pathes, &colors, &args.path, &args.desc_num) {
        Ok(()) => (),
        Err(_error) => return Err(LsiError::FailedDisplay.into()),
    };
    Ok(())
}

/// Retrieves and sets the description for a given file or directory path.
///
/// # Parameters
///
/// - `path`: A mutable reference to an `LsiPath` object representing a file or directory
///
/// # Returns
///
/// - `Result<()>`: Success returns `Ok(())`, while any errors return a relevant `anyhow::Error`
///
/// # Errors
///
/// - `LsiError::DescriptionNotFound`: If the description cannot be read
fn get_and_set_description(path: &mut LsiPath) -> Result<()> {
    let _description = match path.kind {
        LsiPathKind::Dir => fs::read_dir_description(&path),
        LsiPathKind::File => fs::read_file_description(&path),
    };
    match _description {
        Ok(content) => {
            path.set_description(content);
        }
        Err(_error) => {
            return Err(LsiError::DescriptionNotFound.into());
        }
    }
    Ok(())
}

/// Retrieves and sets descriptions for a list of file and directory paths.
///
/// # Parameters
///
/// - `pathes`: A mutable reference to a vector of `LsiPath` objects
///
/// # Returns
///
/// - `Result<()>`: Success returns `Ok(())`, while any errors return a relevant `anyhow::Error`
fn get_and_set_descriptions(pathes: &mut Vec<LsiPath>) -> Result<()> {
    for path in pathes {
        let _ = get_and_set_description(&mut *path);
    }
    Ok(())
}
