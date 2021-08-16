/// controller/mod.rs
/// Define all code stream.
pub mod fs;

extern crate exitcode;
use anyhow::Result;
use crate::{LsiArgs, controller, view};
use crate::models::errors::LsiError;
use crate::models::LsiPath;

pub fn run_lsi(args: &LsiArgs) -> Result<()>{
    let mut pathes = match controller::fs::get_pathes(&args.path) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
    };

    get_and_set_descriptions(&mut pathes)?;

    match view::display(pathes, &args.is_only, &args.show_hidden) {
        Ok(()) => (),
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    Ok(())
}

fn get_and_set_description(path: &mut LsiPath) -> Result<()> {
    let _description = controller::fs::read_description(&path);
    match _description {
        Ok(content) => { path.set_description(content); },
        Err(_error) => { },
    }
    Ok(())
}

fn get_and_set_descriptions(pathes: &mut Vec<LsiPath>) -> Result<()> {
    for path in pathes {
        get_and_set_description(&mut *path);
    }
    Ok(())
}
