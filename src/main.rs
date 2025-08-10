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
    match parser.parse_pe_signature() {
        Ok((sig, e_lfanew)) => {
            println!("PE signature: 0x{:08X} at offset 0x{:X}", sig, e_lfanew);
        }
        Err(e) => eprintln!("PE signature error: {:?}", e),
    }
}
