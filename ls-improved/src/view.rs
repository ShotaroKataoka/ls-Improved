/// Define how to display lines.

use crate::models::path::{LsiPath, LsiPathKind};
use crate::colors::Colors;
use anyhow::Result;

pub fn display(pathes: Vec<LsiPath>, colors:&Colors) -> Result<()> {
    let length = pathes.len();
    for (i, path) in pathes.iter().enumerate() {
        let is_last = i+1 == length;
        display_a_line(&path, is_last, &colors)?
    }
    Ok(())
}

fn display_a_line(path: &LsiPath, is_last: bool, colors: &Colors) -> Result<()> {
    let prefix_char = match is_last {
        true => "└──",
        false => "├──",
    };
    match path.kind {
        LsiPathKind::Dir => {
            match &path.get_description() {
                Some(_description) => { println!("{} {}{}{}\t/ {}", prefix_char, colors.dir, path.file_name(), colors.end, _description); },
                None => { println!("{} {}{}{}\t/ Dir", prefix_char, colors.dir, path.file_name(), colors.end); },
            };
        },
        LsiPathKind::File => println!("{} {}\t/ File", prefix_char, path.file_name()),
    };
    Ok(())
}

