//! This module provides functions to display directory listings with color-coded and
//! decorated output. It includes functionalities to sort and format paths, display 
//! current working directory, and handle descriptions.

use crate::colors::Colors;
use crate::decoration;
use crate::path::{LsiPath, LsiPathKind};
use anyhow::Result;
use std::path::PathBuf;

/// Displays a list of paths with color and decorations.
///
/// This function sorts the paths, displays the current working directory,
/// and iterates through each path to display them with appropriate formatting.
///
/// # Arguments
///
/// * `pathes` - A mutable reference to a vector of `LsiPath` to be displayed.
/// * `colors` - A reference to the `Colors` struct for controlling the display colors.
/// * `cwd` - The current working directory as a string slice.
/// * `desc_num` - An optional description number to select specific descriptions.
///
/// # Errors
///
/// This function returns an `anyhow::Result` to handle various types of errors
/// that might occur during IO operations or path handling.
pub fn display(
    pathes: &mut Vec<LsiPath>,
    colors: &Colors,
    cwd: &str,
    desc_num: &Option<usize>,
) -> Result<()> {
    display_cwd(cwd, &colors)?;
    pathes.sort();
    let length = pathes.len();
    let mut i = 0;
    for path in pathes {
        i += 1;
        let is_last = i == length;
        display_a_line(&mut *path, is_last, &colors, desc_num)?;
    }
    Ok(())
}

/// Displays the current working directory with colors.
///
/// This function formats and prints the current working directory using the
/// provided `Colors` struct for color settings.
///
/// # Arguments
///
/// * `cwd` - The current working directory as a string slice.
/// * `colors` - A reference to the `Colors` struct for controlling the display colors.
///
/// # Errors
///
/// This function returns an `anyhow::Result` to handle various types of errors
/// that might occur during path handling.
fn display_cwd(cwd: &str, colors: &Colors) -> Result<()> {
    let mut abs = PathBuf::from(cwd).canonicalize()?;
    let cwd = match abs.file_name() {
        Some(c) => format!(
            "{}{}/{}",
            colors.current_dir,
            c.to_str().unwrap().to_string(),
            colors.end
        ),
        None => format!("{}/{}", colors.current_dir, colors.end),
    };
    if abs.pop() {
        let parent = format!(
            "{}{}/{}",
            colors.dir,
            if abs.to_str().unwrap() == "/" {
                ""
            } else {
                abs.to_str().unwrap()
            },
            colors.end
        );
        println!("{}{}", parent, cwd);
    } else {
        println!("{}", cwd);
    }
    Ok(())
}

/// Displays a single path line with appropriate decorations.
///
/// This function handles the formatting and printing of a single path line,
/// including its prefix, color, and description, if available.
///
/// # Arguments
///
/// * `path` - A mutable reference to the `LsiPath` to be displayed.
/// * `is_last` - A boolean indicating if this is the last path in the list.
/// * `colors` - A reference to the `Colors` struct for controlling the display colors.
/// * `desc_num` - An optional description number to select specific descriptions.
///
/// # Errors
///
/// This function returns an `anyhow::Result` to handle various types of errors
/// that might occur during IO operations or path handling.
fn display_a_line(
    path: &mut LsiPath,
    is_last: bool,
    colors: &Colors,
    desc_num: &Option<usize>,
) -> Result<()> {
    decoration::run(&mut *path, &colors, &desc_num, &is_last)?;
    let prefix_char = match is_last {
        true => "└──",
        false => "├──",
    };
    match path.kind {
        LsiPathKind::Dir => {
            match &path.get_description() {
                Some(_description) => {
                    println!(
                        "{} {}{}{}\t/ {}",
                        prefix_char,
                        colors.dir,
                        path.file_name(),
                        colors.end,
                        _description
                    );
                }
                None => {
                    println!(
                        "{} {}{}{}\t/ Dir",
                        prefix_char,
                        colors.dir,
                        path.file_name(),
                        colors.end
                    );
                }
            };
        }
        LsiPathKind::File => {
            match &path.get_description() {
                Some(_description) => {
                    println!(
                        "{} {}{}{}\t/ {}",
                        prefix_char,
                        colors.file,
                        path.file_name(),
                        colors.end,
                        _description
                    );
                }
                None => {
                    println!(
                        "{} {}{}{}\t/ File",
                        prefix_char,
                        colors.file,
                        path.file_name(),
                        colors.end
                    );
                }
            };
        }
    };
    Ok(())
}
