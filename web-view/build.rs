use std::env;
use std::fs::{read_dir, remove_file, File, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;

extern crate sass_rs;
use sass_rs::{compile_file, Options};

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

fn main() {
    const STATIC_DIR: &str = "static";
    const OUTPUT_CSS: &str = "styles.css";
    let project_dir = env::current_dir().expect("Got current root project dir");
    let static_dir = format!(
        "{}/{}",
        project_dir.to_str().expect("Got project dir path"),
        STATIC_DIR
    );

    let files = read_dir(static_dir.clone()).expect("Not found folder of static web files");

    let output_css_path = &format!("{}/{}", static_dir, OUTPUT_CSS);

    if Path::new(output_css_path).exists() {
        remove_file(output_css_path);
    }

    let mut css_file = append_to_file(output_css_path).expect("Not found output css file");

    files
        .filter_map(Result::ok)
        .filter(|f| f.path().extension() == Some(OsStr::from_bytes(b"scss")))
        .for_each(|f| {
            let css_data = compile_file(
                f.path().to_str().expect("Got path of css file"),
                Options::default(),
            )
            .expect("Compile scss to css");

            css_file
                .write(css_data.as_bytes())
                .expect("Must been write translated css to file");
        });
}

fn append_to_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    // println!("cargo:warning={}", path.as_ref().to_str().unwrap());
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
}
