/// controller/fs.rs
/// Define file/dir io.
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
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
        let mut lsi_path = LsiPath::new(path.unwrap().path());
        p.push(lsi_path);
    }
    p.sort();
    Ok(p)
}

pub fn read_description(path: &LsiPath) -> Result<String>{
    let desc_path = path.absolute_path() + "/.description.lsi";
    let desc_path = Path::new(&desc_path);
    let mut f = File::open(desc_path)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content.trim().to_string())
}
