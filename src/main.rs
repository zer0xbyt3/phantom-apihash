mod bytes;

use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1)
        .expect("usage: program <path-to-file>");
    let data = fs::read(&file_path).expect("failed to read file");
    let mut cursor = bytes::ByteCursor::new(&data);

    println!("loaded {} bytes from {}", cursor.len(), file_path);
}
