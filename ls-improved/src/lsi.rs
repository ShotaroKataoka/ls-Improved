mod controller;
mod models;
mod view;
extern crate exitcode;

use anyhow::Result;
#[macro_use]
extern crate clap;
use clap::App;

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();
    let path = args.value_of("PATH").unwrap();
    let is_all = args.is_present("show_all");
    let is_only_files = args.is_present("only_files");
    let is_only_dirs = args.is_present("only_directories");

    let args = LsiArgs {
        path: path,
        is_all: is_all,
        is_only: if is_only_files { "files" } else if is_only_dirs { "dirs" } else { "all" },
    };

    controller::run_lsi(&args)
    // match controller::run_lsi(&args) {
    //     Ok(()) => {std::process::exit(exitcode::OK);},
    //     Err(_error) => { eprintln!("{}", _error); }
    // }
}

pub struct LsiArgs<'a> {
    path: &'a str,
    is_all: bool,
    is_only: &'a str,
}
