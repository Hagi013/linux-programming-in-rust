use std::env;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::{ BufReader, BufRead };
use std::io::{ Write, stdout };
use std::io::BufWriter;

extern crate regex;
use regex::Regex;

fn main() {

    let args: Vec<String> = env::args().collect();

    let r = Regex::new();
}


fn do_grep(path: &String) {
    match fs::canonicalize(PathBuf::from(path)) {
        Ok(ap) => check_file(ap),
        Err(e) => write_error(e),
    }
}

fn check_file(abs_path: PathBuf) {
    match File::open(abs_path) {
        Ok(f) => check_contents(f),
        Err(e) => write_error(e),
    }
}

fn check_contents(f: File) {
    let file_by_row = BufReader::new(&file);
    let mut writer = BufWriter::new(stdout());
    for (num, row) in file_by_row.line().enumerate() {
        let r = row.unwrap();
    }
}


fn write_error(e: std::io::Error) {
    let stdout = stdout();
    let mut handler = stdout.lock();
    handler.write(e.to_string().as_bytes());
    handler.write(b"\n");
}