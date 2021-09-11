mod colors;
mod config;
mod lsi;
mod decoration;
mod errors;
mod file_description;
mod fs;
mod path;
mod view;
mod mkdiri;
extern crate exitcode;
extern crate unicode_width;

#[macro_use]
extern crate clap;
extern crate serde_derive;
extern crate toml;

use anyhow::Result;
use clap::App;
use path::LsiPathKind;

fn main() -> Result<()> {
    let yaml = load_yaml!("args.yml");
    let args = App::from_yaml(yaml).get_matches();
    let path = args.value_of("PATH").unwrap();
    let show_hidden = args.is_present("show_all");
    let is_only_files = args.is_present("only_files");
    let is_only_dirs = args.is_present("only_directories");
    let config_path = args.value_of("config_path");
    let desc_num = match value_t!(args.value_of("desc_num"), usize) {
        Ok(n) => Some(n),
        Err(_) => None,
    };
    let set_description = args.value_of("set_description");
    let edit_description = args.value_of("edit_description");
    let is_edit_description = if args.occurrences_of("edit_description")==0 { false } else { true };

    let args = LsiArgs {
        path: path,
        show_hidden: show_hidden,
        is_only: if is_only_files {
            Some(LsiPathKind::File)
        } else if is_only_dirs {
            Some(LsiPathKind::Dir)
        } else {
            None
        },
        config_path: config_path,
        desc_num: desc_num,
        is_mkdiri_mode: set_description.is_some() || is_edit_description,
        set_description: set_description,
        edit_description: edit_description,
    };


    match &args.is_mkdiri_mode {
        true => { mkdiri::run(&args); },
        false => { lsi::run(&args); },
    }
    Ok(())
}

pub struct LsiArgs<'a> {
    path: &'a str,
    show_hidden: bool,
    is_only: Option<LsiPathKind>,
    config_path: Option<&'a str>,
    desc_num: Option<usize>,
    is_mkdiri_mode: bool,
    set_description: Option<&'a str>,
    edit_description: Option<&'a str>,
}
