pub mod decoration;

extern crate colored;

use crate::models::LsiPath;
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
    match path {
        LsiPath::Dir{..} => println!("{}", path.file_name().cyan()),
        LsiPath::File{..} => println!("{}", path.file_name()),
    };
    Ok(())
}

fn filter(path: &LsiPath, is_only: &str, show_hidden: &bool) -> Result<bool> {
    match is_only {
        "dirs" => {
            let show_flag = match path {
                LsiPath::Dir{..} => true,
                _ => false,
            };
            Ok(show_flag || *show_hidden)
        },
        "files" => {
            let show_flag = match path {
                LsiPath::File{..} => true,
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

