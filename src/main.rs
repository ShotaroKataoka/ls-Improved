//! This module defines and implements the main functionality of the application.
//! It processes command-line arguments, handles configuration settings, and orchestrates
//! the execution flow of different modules including lsi and mkdiri.

mod colors;
mod config;
mod decoration;
mod errors;
mod fs;
mod lsi;
mod mkdiri;
mod path;
mod view;

extern crate exitcode;
extern crate unicode_width;

#[macro_use]
extern crate clap;
extern crate serde_derive;
extern crate toml;

use anyhow::Result;
use async_std::io;
use clap::App;
use path::LsiPathKind;
use std::time::Duration;

/// The main function serves as the entry point of the application.
///
/// It reads command-line arguments, configures the runtime environment,
/// and dispatches tasks to relevant modules based on the specified options.
///
/// # Errors
///
/// This function returns an `anyhow::Result` to handle various types of errors
/// that might occur during IO operations or argument processing.
#[async_std::main]
async fn main() -> Result<()> {
    // Load command-line arguments from the YAML file.
    let yaml = load_yaml!("args.yml");
    let args = App::from_yaml(yaml).get_matches();

    // Retrieve values from the command-line arguments.
    let path = args.value_of("PATH").unwrap();
    let show_hidden = args.is_present("show_all");
    let is_only_files = args.is_present("only_files");
    let is_only_dirs = args.is_present("only_directories");
    let config_path = args.value_of("config_path");

    // Handle the optional description number.
    let desc_num = match value_t!(args.value_of("desc_num"), usize) {
        Ok(n) => Some(n),
        Err(_) => None,
    };

    // Options for managing descriptions.
    let set_description = args.value_of("set_description");
    let edit_description = args.value_of("edit_description");
    let is_edit_description = args.occurrences_of("edit_description") > 0;
    let sort_mode = args.value_of("sort_mode").unwrap();

    // Read piped input with a timeout.
    let input = io::timeout(Duration::from_millis(1), async {
        let stdin = io::stdin();
        let mut line = String::new();
        stdin.read_line(&mut line).await?;
        Ok(line)
    })
    .await;

    // Determine the effective path to use.
    let path = match input {
        Ok(mut i) => {
            i.retain(|c| c != '\n');
            i
        }
        Err(_) => path.to_string(),
    };

    // Configuration arguments for lsi/mkdiri.
    let args = LsiArgs {
        path: &path,
        show_hidden,
        is_only: match (is_only_files, is_only_dirs) {
            (true, _) => Some(LsiPathKind::File),
            (_, true) => Some(LsiPathKind::Dir),
            _ => None,
        },
        config_path,
        desc_num,
        is_mkdiri_mode: set_description.is_some() || is_edit_description,
        set_description,
        edit_description,
        sort_mode: sort_mode.to_string(),
    };

    // Execute the appropriate module based on the mode.
    match args.is_mkdiri_mode {
        true => mkdiri::run(&args),
        false => lsi::run(&args),
    }
}

/// This struct encapsulates command-line argument values and configurations for lsi/mkdiri.
pub struct LsiArgs<'a> {
    /// The path to list or manage.
    pub path: &'a str,
    /// Whether to show hidden files or not.
    pub show_hidden: bool,
    /// An optional filter to list only files or directories.
    pub is_only: Option<LsiPathKind>,
    /// The path to the configuration file.
    pub config_path: Option<&'a str>,
    /// An optional description number.
    pub desc_num: Option<usize>,
    /// Whether the current mode is mkdiri or not.
    pub is_mkdiri_mode: bool,
    /// The description to set (if applicable).
    pub set_description: Option<&'a str>,
    /// The description to edit (if applicable).
    pub edit_description: Option<&'a str>,
    /// The mode for sorting entries.
    pub sort_mode: String,
}
