pub mod decoration;

extern crate colored;

use crate::models;
use crate::LsiArgs;
use colored::*;

pub fn display(dirs: Vec<models::LsiPath>, files: Vec<models::LsiPath>, args: &LsiArgs) -> Result<(), String> {
    if args.is_only == "dirs" || args.is_only == "all" {
        for path in dirs {
            if path.is_hidden() {
                if args.is_all {
                    println!("{}", path.file_name().cyan());
                }
            } else {
                println!("{}", path.file_name().cyan());
            }
        }
    }
    if args.is_only == "files" || args.is_only == "all" {
        for path in files {
            if path.is_hidden() {
                if args.is_all {
                    println!("{}", path.file_name());
                }
            } else {
                println!("{}", path.file_name());
            }
        }
    }
    Ok(())
}
