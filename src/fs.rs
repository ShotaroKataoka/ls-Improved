//! This module provides utilities for interacting with file paths, directories, and
//! managing descriptions associated with them. It includes functions to gather paths,
//! filter them according to specified criteria, and read/write descriptions.

use crate::errors::LsiError;
use crate::path::{LsiPath, LsiPathKind};
use anyhow::{Context, Result};
use regex::Regex;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Collects paths from the target directory and applies specified filters and sorting.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path of the directory to list.
/// * `is_only` - An optional filter to list only files or directories specified by `LsiPathKind`.
/// * `show_hidden` - A boolean flag indicating whether to include hidden files.
/// * `sort_mode` - A string slice specifying how to sort the paths.
///
/// # Errors
///
/// Returns an error if the target path does not exist or cannot be read.
///
/// # Returns
///
/// A vector of `LsiPath` objects representing paths in the target directory.
pub fn get_pathes(
    path: &str,
    is_only: &Option<LsiPathKind>,
    show_hidden: &bool,
    sort_mode: &str,
) -> Result<Vec<LsiPath>> {
    let pathes = fs::read_dir(path)
        .with_context(|| format!("Failed to read directory: {}", path))?;
    
    let mut paths = Vec::new();
    for entry in pathes {
        let entry = entry.with_context(|| format!("Failed to read directory entry in {}", path))?;
        let path = entry.path();
        
        if path_filter(&path, is_only, show_hidden) {
            let lsi_path = LsiPath::new(path, sort_mode);
            paths.push(lsi_path);
        }
    }
    
    paths.sort();
    Ok(paths)
}

/// Filters a path according to specified criteria.
///
/// # Arguments
///
/// * `path` - A reference to the path to filter.
/// * `is_only` - An optional filter to include only specific kinds of paths (`LsiPathKind`).
/// * `show_hidden` - A boolean flag indicating whether to include hidden paths.
///
/// # Returns
///
/// A boolean indicating whether the path meets the filter criteria.
fn path_filter(path: &Path, is_only: &Option<LsiPathKind>, show_hidden: &bool) -> bool {
    let is_hidden = LsiPath::is_hidden(path);
    
    if !is_hidden || *show_hidden {
        match is_only {
            Some(LsiPathKind::Dir) => path.is_dir(),
            Some(LsiPathKind::File) => path.is_file(),
            None => true,
        }
    } else {
        false
    }
}

/// Reads a directory description from a `.description.lsi` file located in the directory.
///
/// # Arguments
///
/// * `path` - A reference to an `LsiPath` representing the directory.
///
/// # Errors
///
/// Returns an error if the description file cannot be read.
///
/// # Returns
///
/// A string containing the description read from the file.
pub fn read_dir_description(path: &LsiPath) -> Result<String> {
    let abs_path = path.absolute_path()?;
    let desc_path = format!("{}/.description.lsi", abs_path);
    read_description_file(&desc_path)
}

/// Reads a file description from a `.file_description_lsi/.<filename>.lsi` file.
///
/// # Arguments
///
/// * `path` - A reference to an `LsiPath` representing the file.
///
/// # Errors
///
/// Returns an error if the description file cannot be read.
///
/// # Returns
///
/// A string containing the description read from the file.
pub fn read_file_description(path: &LsiPath) -> Result<String> {
    let abs_path = path.absolute_path()?;
    let path_buf = PathBuf::from(&abs_path);
    
    let filename = path_buf.file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| LsiError::InvalidPath)?;
    
    let parent = path_buf.parent()
        .and_then(|p| p.to_str())
        .ok_or_else(|| LsiError::InvalidPath)?;
    
    let desc_path = format!("{}/.file_description_lsi/.{}.lsi", parent, filename);
    read_description_file(&desc_path)
}

/// Helper function to read a description file.
///
/// # Arguments
///
/// * `path` - The path to the description file.
///
/// # Returns
///
/// A Result containing the description as a String.
fn read_description_file(path: &str) -> Result<String> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open description file: {}", path))?;
    
    let reader = BufReader::new(file);
    let content: Result<String> = reader.lines()
        .collect::<std::result::Result<Vec<_>, _>>()
        .map(|lines| lines.join("\n"))
        .with_context(|| format!("Failed to read lines from description file: {}", path));
    
    Ok(content?.trim().to_string())
}

/// Writes a description to a file or directory description file.
///
/// # Arguments
///
/// * `path` - A reference to the `Path` representing the target path.
/// * `content` - A string containing the description to write.
///
/// # Errors
///
/// Returns an error if the description file cannot be created or written to.
pub fn write_description(path: &Path, content: String) -> Result<()> {
    let content = Regex::new(r"\\n")
        .unwrap_or_else(|_| Regex::new(r"").unwrap())
        .replace_all(&content, "\n")
        .to_string();

    let canonical_path = path.canonicalize()
        .with_context(|| format!("Failed to canonicalize path: {}", path.display()))?;
    
    let filename = if path.is_dir() {
        format!(
            "{}/.description.lsi",
            canonical_path.to_str().ok_or_else(|| LsiError::InvalidPath)?
        )
    } else {
        let mut path_buf = PathBuf::from(canonical_path);
        let filename = path_buf.file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| LsiError::InvalidPath)?
            .to_string();
        
        path_buf.pop();
        path_buf.push(".file_description_lsi");
        
        if !path_buf.is_dir() {
            fs::create_dir(&path_buf)
                .with_context(|| format!("Failed to create directory: {}", path_buf.display()))?;
        }
        
        format!(
            "{}/.{}.lsi", 
            path_buf.to_str().ok_or_else(|| LsiError::InvalidPath)?, 
            filename
        )
    };

    let mut file = File::create(&filename)
        .with_context(|| format!("Failed to create description file: {}", filename))?;

    file.write_all(content.as_bytes())
        .with_context(|| format!("Failed to write to description file: {}", filename))?;
    
    println!("Success: Write description to {}", &filename);

    Ok(())
}
