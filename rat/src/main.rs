use std::env;
use std::path::PathBuf;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::{ Write, stdout };
use std::io::{ BufReader, BufRead };
use std::io::BufWriter;

fn main() {

    let args: Vec<String> = env::args().collect();

//    for arg in &args {
//        println!("arg: {}", arg);
//    }
    do_cat(&args[1]);
}


fn do_cat(path: &String) {
    match fs::canonicalize(PathBuf::from(path)) {
        Ok(p) => print_file(p),
        Err(e) => write_error(e),
    };

//    let abs_path = match fs::canonicalize(PathBuf::from(path)).unwrap() {
//        Ok(p) => p,
//        Err(e) => write_error(e),
//    };
    // println!("This is abs path {:?}", abs_path);

//    let mut file = match File::open(abs_path).unwrap() {
//        Ok(f) => f,
//        Err(e) => write_error(e),
//    };

    // general write
//    let mut buf = Vec::new();
//    file.read_to_end(&mut buf);
//
//    let stdout = stdout();
//    let mut handler = stdout.lock();
//    handler.write(&buf);

    // write by row
//    let file_by_row = BufReader::new(&file);
//    let mut writer = BufWriter::new(stdout());
//    for (num, row) in file_by_row.lines().enumerate() {
//        let r = row.unwrap();
//        writer.write(r.as_bytes()).unwrap();
//        writer.write(b"\n").unwrap();
//    }
}

fn print_file(abs_path: PathBuf) {
    match File::open(abs_path) {
        Ok(f) => write_buf(f),
        Err(e) => write_error(e),
    };
}

fn write_buf(file: File) {
    let file_by_row = BufReader::new(&file);
    let mut writer = BufWriter::new(stdout());
    for (num, row) in file_by_row.lines().enumerate() {
        let r = row.unwrap();
        writer.write(r.as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
    }
}

fn write_error(e: std::io::Error) {
    let stdout = stdout();
    let mut handler = stdout.lock();
    handler.write(e.to_string().as_bytes());
    handler.write(b"\n");
}
