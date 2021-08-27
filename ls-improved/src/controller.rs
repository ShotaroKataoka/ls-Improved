/// Define all code stream.

extern crate exitcode;
use anyhow::Result;
use crate::{LsiArgs, fs, view, decoration};
use crate::errors::LsiError;
use crate::models::{LsiPath, LsiPathKind};

pub fn run_lsi(args: &LsiArgs) -> Result<()>{
    /// ---------------------------------- ///
    /// Glob target files and directories! ///
    /// ---------------------------------- ///
    let mut pathes = match fs::get_pathes(&args.path, &args.is_only, &args.show_hidden) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
    };

    /// -------------------------- ///
    /// Read and set descriptions! ///
    /// -------------------------- ///
    let file_descriptions = fs::read_file_descriptions(&args.path);
    get_and_set_descriptions(&mut pathes)?;
    decoration::replace_color_codes(&mut pathes);

    /// -------------------- ///
    /// Display LSI results! ///
    /// -------------------- ///
    match view::display(pathes) {
        Ok(()) => (),
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    Ok(())
}

fn get_and_set_description(path: &mut LsiPath) -> Result<()> {
    match path.kind {
        LsiPathKind::Dir => {
            let _description = fs::read_dir_description(&path);
            match _description {
                Ok(content) => { path.set_description(content); },
                Err(_error) => { return Err(LsiError::TestError.into()); },
            }
        },
        LsiPathKind::File => {
        }
    }
    Ok(())
}

fn get_and_set_descriptions(pathes: &mut Vec<LsiPath>) -> Result<()> {
    for path in pathes {
        get_and_set_description(&mut *path);
    }
    Ok(())
}
