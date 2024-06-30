//! This module provides utilities for interacting with file paths, directories, and
//! managing descriptions associated with them. It includes functions to gather paths,
//! filter them according to specified criteria, and read/write descriptions.

use crate::errors::LsiError;
use crate::path::{LsiPath, LsiPathKind};
use anyhow::Result;
use regex::Regex;
use std::fs::{self, File};
use std::io::{Read, Write};
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
    let pathes = match fs::read_dir(path) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::PathNotFound.into()),
    };
    let mut p = Vec::new();
    for path in pathes {
        let path = path.unwrap().path();
        if path_filter(&path, is_only, show_hidden) {
            let lsi_path = LsiPath::new(path, sort_mode);
            p.push(lsi_path);
        }
    }
    p.sort();
    Ok(p)
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
    match is_only {
        Some(kind) => match kind {
            LsiPathKind::Dir => {
                let is_target = path.is_dir();
                let is_hidden = LsiPath::is_hidden(path);
                is_target && (!is_hidden || *show_hidden)
            }
            LsiPathKind::File => {
                let is_target = path.is_file();
                let is_hidden = LsiPath::is_hidden(path);
                is_target && (!is_hidden || *show_hidden)
            }
        },
        None => !LsiPath::is_hidden(path) || *show_hidden,
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
    let desc_path = path.absolute_path()? + "/.description.lsi";
    let desc_path = Path::new(&desc_path);
    let mut f = File::open(desc_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
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
    let mut path = PathBuf::from(path.absolute_path()?);
    let filename = path.file_name().unwrap().to_str().unwrap().to_string();
    path.pop();
    let path = path.to_str().unwrap();
    let desc_path = format!("{}/.file_description_lsi/.{}.lsi", path, filename);

    let desc_path = Path::new(&desc_path);
    let mut f = File::open(desc_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
}

/// Writes a description to a file or directory description file.
///
/// # Arguments
///
/// * `path` - A reference to the `PathBuf` representing the target path.
/// * `content` - A string containing the description to write.
///
/// # Errors
///
/// Returns an error if the description file cannot be created or written to.
pub fn write_description(path: &Path, content: String) -> Result<()> {
    let content = Regex::new(r"\\n")
        .unwrap()
        .replace_all(&content, "\n")
        .to_string();

    let filename = if path.is_dir() {
        format!(
            "{}/.description.lsi",
            path.canonicalize()?.to_str().unwrap()
        )
    } else {
        let mut path = PathBuf::from(path.canonicalize()?.to_str().unwrap());
        let filename = path.file_name().unwrap().to_str().unwrap().to_string();
        path.pop();
        path.push(".file_description_lsi");
        if !path.is_dir() {
            fs::create_dir(path.to_str().unwrap())?;
        }
        format!("{}/.{}.lsi", path.to_str().unwrap(), filename)
    };

    let mut file = File::create(&filename)
        .map_err(|why| anyhow::anyhow!("Couldn't create {}: {}", &filename, why))?;

    file.write_all(content.as_bytes()).map_err(|why| {
        anyhow::anyhow!("Couldn't write \"{}\" to {}: {}", content, &filename, why)
    })?;
    println!("Success: Write description to {}", &filename);

    Ok(())
}
