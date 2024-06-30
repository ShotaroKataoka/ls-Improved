//! This module provides functions to manipulate and display path descriptions
//! with various color codes. It uses `regex` to perform replacements and format
//! descriptions for consistent display.

extern crate regex;
use crate::colors::Colors;
use crate::errors::LsiError;
use crate::path::LsiPath;
use anyhow::Result;
use regex::Regex;

/// Runs the processing pipeline on the given path, including replacing color codes
/// and formatting the description.
///
/// # Arguments
///
/// * `path` - A mutable reference to an `LsiPath` that contains the path description.
/// * `colors` - A reference to `Colors` that holds various color codes.
/// * `desc_num` - An optional reference to the number of description lines to process.
/// * `is_last` - A reference to a boolean indicating if this is the last entry.
///
/// # Errors
///
/// Returns an error if the description is not found or any regex-related error occurs.
pub fn run(
    path: &mut LsiPath,
    colors: &Colors,
    desc_num: &Option<usize>,
    is_last: &bool,
) -> Result<()> {
    let _ = replace_lsi_color_code(path, colors);
    let _ = replace_ansi_color_code(path);
    let _ = format_multiline(path, colors, desc_num, is_last);
    Ok(())
}

/// Replaces custom LSI color codes in the path description with ANSI color codes.
///
/// # Arguments
///
/// * `path` - A mutable reference to an `LsiPath` that contains the path description.
/// * `colors` - A reference to `Colors` that holds various ANSI color codes.
///
/// # Errors
///
/// Returns an error if the description is not found or any regex-related error occurs.
fn replace_lsi_color_code(path: &mut LsiPath, colors: &Colors) -> Result<()> {
    match path.get_description() {
        Some(content) => {
            let content = Regex::new(r";r;")
                .unwrap()
                .replace_all(&content, &colors.red)
                .to_string();
            let content = Regex::new(r";g;")
                .unwrap()
                .replace_all(&content, &colors.green)
                .to_string();
            let content = Regex::new(r";y;")
                .unwrap()
                .replace_all(&content, &colors.yellow)
                .to_string();
            let content = Regex::new(r";b;")
                .unwrap()
                .replace_all(&content, &colors.blue)
                .to_string();
            let content = Regex::new(r";p;")
                .unwrap()
                .replace_all(&content, &colors.purple)
                .to_string();
            let content = Regex::new(r";c;")
                .unwrap()
                .replace_all(&content, &colors.cyan)
                .to_string();
            let content = Regex::new(r";w;")
                .unwrap()
                .replace_all(&content, &colors.white)
                .to_string();
            let content = Regex::new(r";_;")
                .unwrap()
                .replace_all(&content, &colors.underline)
                .to_string();
            let content = Regex::new(r";e;")
                .unwrap()
                .replace_all(&content, format!("{}{}", &colors.end, &colors.description))
                .to_string();
            path.set_description(content);
        }
        None => return Err(LsiError::DescriptionNotFound.into()),
    };
    Ok(())
}

/// Replaces ANSI escape sequences in the path description.
///
/// # Arguments
///
/// * `path` - A mutable reference to an `LsiPath` that contains the path description.
///
/// # Errors
///
/// Returns an error if the description is not found or any regex-related error occurs.
fn replace_ansi_color_code(path: &mut LsiPath) -> Result<()> {
    match path.get_description() {
        Some(content) => {
            let content = Regex::new("\\\\033")
                .unwrap()
                .replace_all(&content, "\x1b")
                .to_string();
            path.set_description(content);
        }
        None => return Err(LsiError::DescriptionNotFound.into()),
    };
    Ok(())
}

/// Adds color to the description based on the provided `Colors` struct.
///
/// # Arguments
///
/// * `description` - A string slice that holds the description text.
/// * `colors` - A reference to `Colors` that holds various ANSI color codes.
///
/// # Returns
///
/// A `String` containing the colorized description.
fn encolor_description(description: &str, colors: &Colors) -> String {
    format!("{}{}{}", colors.description, description, colors.end)
}

/// Formats the description of the path to support multi-line display with proper indentation
/// and tree-like structure.
///
/// # Arguments
///
/// * `path` - A mutable reference to an `LsiPath` that contains the path description.
/// * `colors` - A reference to `Colors` that holds various ANSI color codes.
/// * `line_num` - An optional reference to the number of description lines to process.
/// * `is_last` - A reference to a boolean indicating if this is the last entry.
///
/// # Errors
///
/// Returns an error if the description is not found or any regex-related error occurs.
fn format_multiline(
    path: &mut LsiPath,
    colors: &Colors,
    line_num: &Option<usize>,
    is_last: &bool,
) -> Result<()> {
    let len = path.len();
    match path.get_description() {
        Some(content) => {
            let desc: Vec<&str> = content.split('\n').collect();
            let num = match line_num {
                Some(n) => {
                    if n > &desc.len() {
                        desc.len()
                    } else {
                        *n
                    }
                }
                None => desc.len(),
            };
            if num == 1 {
                path.set_description(encolor_description(desc[0], colors));
            } else {
                let mut description: String = encolor_description(desc[0], colors);
                let tree_prefix = if *is_last { " " } else { "│" };
                for i in 1..num {
                    description = format!(
                        "{}\n{}   {}\t  {}",
                        description,
                        tree_prefix,
                        " ".repeat(len),
                        encolor_description(desc[i], colors)
                    );
                }
                path.set_description(description)
            }
        }
        None => return Err(LsiError::DescriptionNotFound.into()),
    };
    Ok(())
}