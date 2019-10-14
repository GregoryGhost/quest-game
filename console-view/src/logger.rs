use chrono::Utc;
use simplelog::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

fn log_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path)
}

pub fn init_logger() {
    let log_name = format!("quest-game_{}.log", Utc::now().format("%Y-%m-%d"));
    let log_path = Path::new(&log_name);
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        Config::default(),
        log_file(log_path).expect("Create log file"),
    )])
    .expect("Create logger");
}
