//! This module contains the main logic for the lsi application.
//! It handles file and directory listing, reading configurations,
//! fetching descriptions, and displaying the results in the desired format.

use crate::colors::Colors;
use crate::config::read_config;
use crate::errors::LsiError;
use crate::path::LsiPath;
use crate::{fs, view, LsiArgs};
use anyhow::{Context, Result};

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
    let mut paths = fs::get_pathes(&args.path, &args.is_only, &args.show_hidden, &args.sort_mode)
        .with_context(|| format!("Failed to get paths from: {}", args.path))?;

    // Read Configs
    let config = args.config_path.and_then(|path| read_config(path.to_string()));
    let colors_config = config.as_ref().and_then(|c| c.colors.as_ref());
    let colors = Colors::new(colors_config);

    // Read and set descriptions
    get_and_set_descriptions(&mut paths)
        .with_context(|| "Failed to retrieve descriptions for paths")?;

    // Display LSI results
    view::display(&mut paths, &colors, args.path, &args.desc_num)
        .map_err(|e| LsiError::FailedDisplay(e.to_string()))?;
    
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
fn get_and_set_description(path: &mut LsiPath) -> Result<()> {
    let description_result = match path.kind {
        crate::path::LsiPathKind::Dir => fs::read_dir_description(path),
        crate::path::LsiPathKind::File => fs::read_file_description(path),
    };

    match description_result {
        Ok(content) => {
            path.set_description(content);
            Ok(())
        }
        Err(_) => {
            // We don't want to fail the entire operation if a single description is missing
            // Just continue without setting a description
            Ok(())
        }
    }
}

/// Retrieves and sets descriptions for a list of file and directory paths.
///
/// # Parameters
///
/// - `paths`: A mutable reference to a vector of `LsiPath` objects
///
/// # Returns
///
/// - `Result<()>`: Success returns `Ok(())`, while any errors return a relevant `anyhow::Error`
fn get_and_set_descriptions(paths: &mut [LsiPath]) -> Result<()> {
    for path in paths.iter_mut() {
        let _ = get_and_set_description(path);
    }
    Ok(())
}
