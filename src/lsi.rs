/// Define all code stream.
extern crate exitcode;
use crate::colors::Colors;
use crate::config::read_config;
use crate::errors::LsiError;
// use crate::file_description::{read_file_description, FileDescriptions};
use crate::path::{LsiPath, LsiPathKind};
use crate::{fs, view, LsiArgs};
use anyhow::Result;

pub fn run(args: &LsiArgs) -> Result<()> {
    // ---------------------------------- //
    // Glob target files and directories! //
    // ---------------------------------- //
    let mut pathes = match fs::get_pathes(
        &args.path,
        &args.is_only,
        &args.show_hidden,
        &args.sort_mode,
    ) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::PathNotFound.into()),
    };

    // ------------  //
    // Read Configs  //
    // ------------  //
    let config = match &args.config_path {
        Some(path) => read_config(path.to_string()),
        None => None,
    };
    let colors_config = match config {
        Some(ref c) => c.colors.as_ref(),
        None => None,
    };
    let colors = Colors::new(colors_config);

    // -------------------------- //
    // Read and set descriptions! //
    // -------------------------- //

    get_and_set_descriptions(&mut pathes)?;

    // -------------------- //
    // Display LSI results! //
    // -------------------- //
    match view::display(&mut pathes, &colors, &args.path, &args.desc_num) {
        Ok(()) => (),
        Err(_error) => return Err(LsiError::FailedDisplay.into()),
    };
    Ok(())
}

fn get_and_set_description(path: &mut LsiPath) -> Result<()> {
    let _description = match path.kind {
        LsiPathKind::Dir => fs::read_dir_description(&path),
        LsiPathKind::File => fs::read_file_description(&path),
    };
    match _description {
        Ok(content) => {
            path.set_description(content);
        }
        Err(_error) => {
            return Err(LsiError::DescriptionNotFound.into());
        }
    }
    Ok(())
}

fn get_and_set_descriptions(pathes: &mut Vec<LsiPath>) -> Result<()> {
    for path in pathes {
        let _ = get_and_set_description(&mut *path);
    }
    Ok(())
}
