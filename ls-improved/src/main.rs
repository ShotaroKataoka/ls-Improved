mod lsi;
extern crate exitcode;

use argh::FromArgs;

fn main() {
    let args: Args = argh::from_env();

    let (dirs, files) = match lsi::fs::get_pathes(&args.path) {
        Ok(_success) => (_success.0, _success.1),
        Err(_error) => {
            eprintln!("{}", _error);
            std::process::exit(exitcode::OSFILE);
        }
    };

    match lsi::view::display(dirs, files) {
        Ok(()) => (),
        Err(_error) => {
            eprintln!("{}", _error);
            std::process::exit(exitcode::OSFILE);
        }
    };
    std::process::exit(exitcode::OK);
}

#[derive(FromArgs)]
#[argh(description = "...")]
struct Args {
    #[argh(positional)]
    path: String,
}
