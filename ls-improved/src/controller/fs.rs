use std::fs;
use anyhow::Result;
use crate::models::LsiPath;
use crate::models::errors::LsiError;


pub fn get_pathes(path: &str) -> Result<(Vec<LsiPath>, Vec<LsiPath>)> {
    let pathes = match fs::read_dir(path) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for path in pathes {
        let p = LsiPath::new(path.unwrap().path());
        match p {
            LsiPath::Dir {..} => dirs.push(p),
            LsiPath::File {..} => files.push(p),
        }
    }
    dirs.sort();
    files.sort();
    Ok((dirs, files))
}
