use std::fs::File;

pub fn open_file(s: &str) -> File {
    let greeting_file = File::open(s).expect("File not found");
    greeting_file
}
