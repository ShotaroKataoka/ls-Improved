use std::fs;
use anyhow::Result;
use crate::models;
use crate::models::errors::LsiError;


pub fn get_pathes(path: &str) -> Result<(Vec<models::LsiPath>, Vec<models::LsiPath>)> {
    let pathes = match fs::read_dir(path) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for path in pathes {
        let p = models::LsiPath::new(path.unwrap().path());
        match p.ptype() {
            models::PathType::Dir => dirs.push(p),
            models::PathType::File => files.push(p),
        }
    }
    dirs.sort();
    files.sort();
    Ok((dirs, files))
}
