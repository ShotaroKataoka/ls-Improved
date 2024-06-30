extern crate regex;
use crate::colors::Colors;
use crate::errors::LsiError;
use crate::path::LsiPath;
use anyhow::Result;
use regex::Regex;

pub fn run(
    path: &mut LsiPath,
    colors: &Colors,
    desc_num: &Option<usize>,
    is_last: &bool,
) -> Result<()> {
    let _ = replace_lsi_color_code(path, colors);
    let _ = replace_ansi_color_code(path);
    let _ = format_multiline(path, colors, desc_num, is_last);
    Ok(())
}

fn replace_lsi_color_code(path: &mut LsiPath, colors: &Colors) -> Result<()> {
    match path.get_description() {
        Some(content) => {
            let content = Regex::new(r";r;")
                .unwrap()
                .replace_all(&content, &colors.red)
                .to_string();
            let content = Regex::new(r";g;")
                .unwrap()
                .replace_all(&content, &colors.green)
                .to_string();
            let content = Regex::new(r";y;")
                .unwrap()
                .replace_all(&content, &colors.yellow)
                .to_string();
            let content = Regex::new(r";b;")
                .unwrap()
                .replace_all(&content, &colors.blue)
                .to_string();
            let content = Regex::new(r";p;")
                .unwrap()
                .replace_all(&content, &colors.purple)
                .to_string();
            let content = Regex::new(r";c;")
                .unwrap()
                .replace_all(&content, &colors.cyan)
                .to_string();
            let content = Regex::new(r";w;")
                .unwrap()
                .replace_all(&content, &colors.white)
                .to_string();
            let content = Regex::new(r";_;")
                .unwrap()
                .replace_all(&content, &colors.underline)
                .to_string();
            let content = Regex::new(r";e;")
                .unwrap()
                .replace_all(&content, format!("{}{}", &colors.end, &colors.description))
                .to_string();
            path.set_description(content);
        }
        None => return Err(LsiError::TestError.into()),
    };
    Ok(())
}

fn replace_ansi_color_code(path: &mut LsiPath) -> Result<()> {
    match path.get_description() {
        Some(content) => {
            let content = Regex::new("\\\\033")
                .unwrap()
                .replace_all(&content, "\x1b")
                .to_string();
            path.set_description(content);
        }
        None => return Err(LsiError::TestError.into()),
    };
    Ok(())
}

fn encolor_description(description: &str, colors: &Colors) -> String {
    format!("{}{}{}", colors.description, description, colors.end)
}

fn format_multiline(
    path: &mut LsiPath,
    colors: &Colors,
    line_num: &Option<usize>,
    is_last: &bool,
) -> Result<()> {
    let len = path.len();
    match path.get_description() {
        Some(content) => {
            let desc: Vec<&str> = content.split("\n").collect();
            let num = match line_num {
                Some(n) => {
                    if n > &desc.len() {
                        desc.len()
                    } else {
                        *n
                    }
                }
                None => desc.len(),
            };
            if num == 1 {
                path.set_description(encolor_description(desc[0], colors));
            } else {
                let mut description: String = encolor_description(desc[0], colors);
                let tree_prefix = if *is_last { " " } else { "│" };
                for i in 1..num {
                    description = format!(
                        "{}\n{}   {}\t  {}",
                        description,
                        tree_prefix,
                        " ".repeat(len),
                        encolor_description(desc[i], colors)
                    );
                }
                path.set_description(description)
            }
        }
        None => return Err(LsiError::TestError.into()),
    };
    Ok(())
}
