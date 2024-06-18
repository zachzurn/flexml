pub mod document;
pub mod parser;

use std::fs;
use std::path::PathBuf;

fn main() {
    let mut testfile_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    testfile_path.push("resources/test.flexml");

    let src = fs::read_to_string(testfile_path).expect("Failed to read file");
    let document = parser::parse_document(&src);

    println!("{:#?}", document);
}