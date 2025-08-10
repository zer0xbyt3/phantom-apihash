mod bytes;
mod pe;

use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1)
        .expect("usage: program <path-to-file>");
    let data = fs::read(&file_path).expect("failed to read file");
    // let mut cursor = bytes::ByteCursor::new(&data);
    let mut parser = pe::PEParser::new(&data);
    println!("loaded {} bytes from {}", parser.cursor().len(), file_path);

    dbg!(parser.parse_dos_header());
}
