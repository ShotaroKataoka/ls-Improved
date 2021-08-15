use std::fs;
use anyhow::Result;
use crate::models::LsiPath;
use crate::models::errors::LsiError;


pub fn get_pathes(path: &str) -> Result<Vec<LsiPath>> {
    let pathes = match fs::read_dir(path) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    let mut p = Vec::new();
    for path in pathes {
        p.push(LsiPath::new(path.unwrap().path()));
    }
    p.sort();
    Ok(p)
}
