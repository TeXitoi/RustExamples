use std::env;
use std::fs::File;

fn main() {
    let arg = env::args().nth(1).expect("Please, provide a file as argument");
    let file = File::open(&arg).ok().expect("The file does not exist");
    let file_info = file.metadata().ok().expect("Cannot get file metadata");
    if file_info.is_file() {
        println!("{} bytes", file_info.len());
    } else {
        println!("This is not a file");
    }
}
