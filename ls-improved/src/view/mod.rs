/// view/mod.rs
/// Define how to display lines.
pub mod decoration;

extern crate colored;

use crate::models::{LsiPath, LsiPathKind};
use crate::models::errors::LsiError;
use colored::*;
use anyhow::Result;

pub fn display(pathes: Vec<LsiPath>) -> Result<()> {
    for path in pathes {
        display_a_line(&path)?
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

