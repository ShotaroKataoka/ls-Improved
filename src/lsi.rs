/// Define all code stream.
extern crate exitcode;
use crate::colors::Colors;
use crate::config::read_config;
use crate::errors::LsiError;
use crate::file_description::{read_file_description, FileDescriptions};
use crate::path::{LsiPath, LsiPathKind};
use crate::{decoration, fs, view, LsiArgs};
use anyhow::Result;
use std::path::PathBuf;

pub fn run(args: &LsiArgs) -> Result<()> {
    // ---------------------------------- //
    // Glob target files and directories! //
    // ---------------------------------- //
    let mut pathes = match fs::get_pathes(&args.path, &args.is_only, &args.show_hidden) {
        Ok(_success) => _success,
        Err(_error) => return Err(LsiError::TestError.into()),
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
    let abs = PathBuf::from(&args.path).canonicalize()?;
    let _file_descriptions =
        read_file_description(format!("{}/.file_description.lsi", abs.to_str().unwrap()));

    get_and_set_descriptions(&mut pathes, _file_descriptions)?;
    decoration::run(&mut pathes, &colors, args.desc_num)?;

    // -------------------- //
    // Display LSI results! //
    // -------------------- //
    match view::display(pathes, &colors, &args.path) {
        Ok(()) => (),
        Err(_error) => return Err(LsiError::TestError.into()),
    };
    Ok(())
}

fn get_and_set_description(
    path: &mut LsiPath,
    file_descriptions: &Option<FileDescriptions>,
) -> Result<()> {
    match path.kind {
        LsiPathKind::Dir => {
            let _description = fs::read_dir_description(&path);
            match _description {
                Ok(content) => {
                    path.set_description(content);
                }
                Err(_error) => {
                    return Err(LsiError::TestError.into());
                }
            }
        }
        LsiPathKind::File => {
            match file_descriptions {
                Some(fd) => {
                    match &fd.files.as_ref() {
                        Some(files) => {
                            match files.get(path.file_name()) {
                                Some(d) => {
                                    path.set_description(d.to_string());
                                }
                                None => {}
                            };
                        }
                        None => {}
                    };
                }
                None => {}
            };
        }
    }
    Ok(())
}

fn get_and_set_descriptions(
    pathes: &mut Vec<LsiPath>,
    file_descriptions: Option<FileDescriptions>,
) -> Result<()> {
    for path in pathes {
        let _ = get_and_set_description(&mut *path, &file_descriptions);
    }
    Ok(())
}
