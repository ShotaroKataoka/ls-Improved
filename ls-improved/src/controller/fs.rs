use std::fs;
use crate::models;

pub fn get_pathes(path: &String) -> Result<(Vec<models::LsiPath>, Vec<models::LsiPath>), String>{
    let pathes = fs::read_dir(path);
    let pathes = match pathes {
        Ok(_success) => _success,
        Err(_error) => return Err(_error.to_string()),
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
