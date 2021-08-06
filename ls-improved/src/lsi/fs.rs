use std::fs;
use crate::lsi;

pub fn get_pathes(path: &String) -> Result<(Vec<lsi::path::LsiPath>, Vec<lsi::path::LsiPath>), String>{
    let pathes = fs::read_dir(path);
    let pathes = match pathes {
        Ok(_success) => _success,
        Err(_error) => return Err(_error.to_string()),
    };
    let mut dirs = Vec::new();
    let mut files = Vec::new();
    for path in pathes {
        let p = lsi::path::LsiPath::new(path.unwrap().path());
        match p.ptype() {
            lsi::path::PathType::Dir => dirs.push(p),
            lsi::path::PathType::File => files.push(p),
        }
    }
    dirs.sort();
    files.sort();
    Ok((dirs, files))
}
