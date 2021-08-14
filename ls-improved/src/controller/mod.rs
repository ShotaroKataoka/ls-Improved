pub mod fs;

extern crate exitcode;
use anyhow::Result;
use crate::{LsiArgs, controller, view};
use crate::models::errors::LsiError;

pub fn run_lsi(args: &LsiArgs) -> Result<()>{
    let (dirs, files) = match controller::fs::get_pathes(&args.path) {
        Ok(_success) => (_success.0, _success.1),
        Err(_error) => return Err(LsiError::TestError.into()),
    };

    match view::display(dirs, files, &args) {
        Ok(()) => (),
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    Ok(())
}
