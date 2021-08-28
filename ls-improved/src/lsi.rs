mod controller;
mod models;
mod view;
mod fs;
mod decoration;
mod errors;
mod colors;
mod config;
extern crate exitcode;

#[macro_use]
extern crate clap;
extern crate serde_derive;
extern crate toml;

use anyhow::Result;
use clap::App;
use models::path::LsiPathKind;


fn main() -> Result<()> {
    let yaml = load_yaml!("args.yml");
    let args = App::from_yaml(yaml).get_matches();
    let path = args.value_of("PATH").unwrap();
    let show_hidden = args.is_present("show_all");
    let is_only_files = args.is_present("only_files");
    let is_only_dirs = args.is_present("only_directories");

    let args = LsiArgs {
        path: path,
        show_hidden: show_hidden,
        is_only: if is_only_files { Some(LsiPathKind::File) } else if is_only_dirs { Some(LsiPathKind::Dir) } else { None },
    };

    controller::run_lsi(&args)
}

pub struct LsiArgs<'a> {
    path: &'a str,
    show_hidden: bool,
    is_only: Option<LsiPathKind>,
}
