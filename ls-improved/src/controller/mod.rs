pub mod fs;

extern crate exitcode;
use anyhow::Result;
use crate::{LsiArgs, controller, view};
use crate::models::errors::LsiError;

pub fn run_lsi(args: &LsiArgs) -> Result<()>{
    let pathes = match controller::fs::get_pathes(&args.path) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
    };

    match view::display(pathes, &args.is_only, &args.show_hidden) {
        Ok(()) => (),
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    Ok(())
}
