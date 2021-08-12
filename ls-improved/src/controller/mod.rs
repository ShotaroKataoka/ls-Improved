pub mod fs;

extern crate exitcode;
use crate::{LsiArgs, controller, view};

pub fn run_lsi(args: &LsiArgs) -> Result<(), String>{
    let (dirs, files) = match controller::fs::get_pathes(&args.path) {
        Ok(_success) => (_success.0, _success.1),
        Err(_error) => {
            eprintln!("{}", _error);
            std::process::exit(exitcode::OSFILE);
        }
    };

    match view::display(dirs, files, &args) {
        Ok(()) => (),
        Err(_error) => {
            eprintln!("{}", _error);
            std::process::exit(exitcode::OSFILE);
        }
    };
    Ok(())
}
