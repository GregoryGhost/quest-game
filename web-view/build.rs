use std::env;
use std::fs::{read_dir, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process;
use std::process::Command;

extern crate sass_rs;
use sass_rs::{compile_file, Options};

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

fn main() {
    const STATIC_DIR: &str = "static";
    let project_dir = env::current_dir().expect("Got current root project dir");
    let static_dir = format!("{}/{}", project_dir.to_str().unwrap(), STATIC_DIR);

    let files = read_dir(static_dir.clone()).expect("Not found folder of static web files");
    files
        .filter_map(Result::ok)
        .filter(|f| f.path().extension() == Some(OsStr::from_bytes(b"scss")))
        .for_each(|f| {
            let css_data = compile_file(f.path().to_str().unwrap(), Options::default());
            append_to_file("style1.css");
        });
}

fn append_to_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
}
