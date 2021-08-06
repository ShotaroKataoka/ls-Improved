pub mod decoration;

extern crate colored;

use crate::models;
use colored::*;

pub fn display(dirs: Vec<models::LsiPath>, files: Vec<models::LsiPath>) -> Result<(), String>{
    for path in dirs {
        if path.is_hidden() {
            // println!("{}", path.file_name().cyan());
        } else {
            println!("{}", path.file_name().cyan());
        }
    };
    for path in files {
        if path.is_hidden() {
            // println!("{}", path.file_name());
        } else {
            println!("{}", path.file_name());
        }
    };
    Ok(())
}
