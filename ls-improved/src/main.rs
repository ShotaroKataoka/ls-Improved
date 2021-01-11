use std::ffi::CString;
use std::path::Path;
use std::slice;
use std::str;

use anyhow::{anyhow, bail, Result};
use libc;
use argh::FromArgs;


fn main() {
    let args: Args = argh::from_env();
    let path = Path::new(&args.path);
    let path_vec = read_dir(path).expect("error: read_dir");
    for i in path_vec {
        println!("{}", i);
    };
}

#[derive(FromArgs)]
#[argh(description = "...")]
struct Args {
    #[argh(positional)]
    path: String,
}

#[cfg(target_os = "linux")]
fn read_dir<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let path = path.as_ref().to_str().ok_or(anyhow!("No path str"))?;
    let path = CString::new(path)?;
    let dir = unsafe { libc::opendir(path.as_ptr()) };
    if dir.is_null() {
        bail!("Dir in null");
    }
    let mut v = vec![];
    loop {
        let entry = unsafe { libc::readdir(dir) };
        if entry.is_null() {
            break;
        }
        let file_name = unsafe {
            let name = (*entry).d_name.as_ptr();
            let len = libc::strlen(name) as usize;
            let slice = slice::from_raw_parts(name as *const u8, len);
            str::from_utf8_unchecked(slice as &[u8]).into()
        };
        v.push(file_name);
    }
    unsafe { libc::closedir(dir) };
    v.sort();
    Ok(v)
}
