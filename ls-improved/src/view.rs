/// Define how to display lines.

use crate::models::{LsiPath, LsiPathKind};
use anyhow::Result;

pub fn display(pathes: Vec<LsiPath>) -> Result<()> {
    let length = pathes.len();
    for (i, path) in pathes.iter().enumerate() {
        let is_last = i+1 == length;
        display_a_line(&path, is_last)?
    }
    Ok(())
}

fn display_a_line(path: &LsiPath, is_last: bool) -> Result<()> {
    let prefix_char = match is_last {
        true => "└──",
        false => "├──",
    };
    match path.kind {
        LsiPathKind::Dir => {
            match &path.get_description() {
                Some(_description) => { println!("{} \x1b[36m\x1b[4m{}\x1b[0m\t/ {}", prefix_char, path.file_name(), _description); },
                None => { println!("{} \x1b[36m\x1b[4m{}\x1b[0m\t/ Dir", prefix_char, path.file_name()); },
            };
        },
        LsiPathKind::File => println!("{} {}\t/ File", prefix_char, path.file_name()),
    };
    Ok(())
}

