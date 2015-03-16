use std::env;
use std::fs::File;

fn main() {
    match file_size_from_arg() {
        Ok(size) => println!("{} bytes", size),
        Err(cause) => println!("{}", cause)
    }
}

fn file_size_from_arg() -> Result<u64, &'static str> {
    match env::args().nth(1) {
        Some(arg) => file_size_by_name(arg),
        None => Err("Please, provide a file as argument")
    }
}

fn file_size_by_name(file_name: String) -> Result<u64, &'static str> {
    match File::open(&file_name) {
        Ok(file) => file_size(file),
        _ => Err("The file does not exist")
    }
}

fn file_size(file: File) -> Result<u64, &'static str> {
    match file.metadata() {
        Ok(metadata) => if metadata.is_file() {
                            Ok(metadata.len())
                        } else {
                            Err("This is not a file")
                        },
        _ => Err("Cannot get file metadata")
    }
}
