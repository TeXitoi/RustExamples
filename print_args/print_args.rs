use std::env;

fn main() {
    let mut index: i32 = 1;
    for arg in env::args() {
        println!("The argument {} is {}", index, arg);
        index += 1;
    }
}
