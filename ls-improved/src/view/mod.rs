/// view/mod.rs
/// Define how to display lines.
pub mod decoration;

extern crate colored;

use crate::models::{LsiPath, LsiPathKind};
use crate::models::errors::LsiError;
use colored::*;
use anyhow::Result;

pub fn display(pathes: Vec<LsiPath>, 
               is_only: &str, show_hidden: &bool) -> Result<()> {
    for path in pathes {
        if filter(&path, &is_only, &show_hidden)? {
            display_a_line(&path)?
        }
    }
    Ok(())
}

fn display_a_line(path: &LsiPath) -> Result<()> {
    match path.kind {
        LsiPathKind::Dir => {
            match &path.get_description() {
                Some(_description) => { println!("{}\t/ {}", path.file_name().cyan(), _description.yellow()); },
                None => { println!("{}\t/ Dir", path.file_name().cyan()); },
            };
        },
        LsiPathKind::File => println!("{}\t/ File", path.file_name()),
    };
    Ok(())
}

fn filter(path: &LsiPath, is_only: &str, show_hidden: &bool) -> Result<bool> {
    match is_only {
        "dirs" => {
            let show_flag = match path.kind {
                LsiPathKind::Dir => true,
                _ => false,
            };
            Ok(show_flag || *show_hidden)
        },
        "files" => {
            let show_flag = match path.kind {
                LsiPathKind::File => true,
                _ => false,
            };
            Ok(show_flag || *show_hidden)
        },
        "all" => {
            Ok(!path.is_hidden() || *show_hidden)
        },
        _ => Err(LsiError::TestError.into()),
    }
}

