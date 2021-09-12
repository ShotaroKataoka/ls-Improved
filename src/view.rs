use crate::colors::Colors;
/// Define how to display lines.
use crate::path::{LsiPath, LsiPathKind};
use anyhow::Result;
use std::path::PathBuf;
use crate::decoration;

pub fn display(pathes: &mut Vec<LsiPath>, colors: &Colors, cwd: &str, desc_num: &Option<usize>) -> Result<()> {
    display_cwd(cwd, &colors)?;
    pathes.sort();
    let length = pathes.len();
    // for (i, &mut path) in pathes.iter().enumerate() {
    //     let is_last = i + 1 == length;
    //     display_a_line(&mut *path, is_last, &colors, desc_num)?
    // }
    let mut i = 0;
    for path in pathes {
        i += 1;
        let is_last = i==length;
        display_a_line(&mut *path, is_last, &colors, desc_num)?;
    }
    Ok(())
}

fn display_cwd(cwd: &str, colors: &Colors) -> Result<()> {
    let mut abs = PathBuf::from(cwd).canonicalize()?;
    let cwd = match abs.file_name() {
        Some(c) => format!(
            "{}{}/{}",
            colors.current_dir,
            c.to_str().unwrap().to_string(),
            colors.end
        ),
        None => format!("{}/{}", colors.current_dir, colors.end),
    };
    if abs.pop() {
        let parent = format!(
            "{}{}/{}",
            colors.dir,
            if abs.to_str().unwrap()=="/" {""} else { abs.to_str().unwrap() },
            colors.end
        );
        println!("{}{}", parent, cwd);
    } else {
        println!("{}", cwd);
    }
    Ok(())
}

fn display_a_line(path: &mut LsiPath, is_last: bool, colors: &Colors, desc_num: &Option<usize>) -> Result<()> {
    decoration::run(&mut *path, &colors, &desc_num, &is_last)?;
    let prefix_char = match is_last {
        true => "└──",
        false => "├──",
    };
    match path.kind {
        LsiPathKind::Dir => {
            match &path.get_description() {
                Some(_description) => {
                    println!(
                        "{} {}{}{}\t/ {}",
                        prefix_char,
                        colors.dir,
                        path.file_name(),
                        colors.end,
                        _description
                    );
                }
                None => {
                    println!(
                        "{} {}{}{}\t/ Dir",
                        prefix_char,
                        colors.dir,
                        path.file_name(),
                        colors.end
                    );
                }
            };
        }
        LsiPathKind::File => {
            match &path.get_description() {
                Some(_description) => {
                    println!(
                        "{} {}{}{}\t/ {}",
                        prefix_char,
                        colors.file,
                        path.file_name(),
                        colors.end,
                        _description
                    );
                }
                None => {
                    println!(
                        "{} {}{}{}\t/ File",
                        prefix_char,
                        colors.file,
                        path.file_name(),
                        colors.end
                    );
                }
            };
        }
    };
    Ok(())
}
