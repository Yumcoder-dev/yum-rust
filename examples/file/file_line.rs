// if file does not in root of example dir, first add the following tag in Cargo.toml file
// [[example]]
// name = "file_line"
// path = "examples/file/file_line.rs"
//
// for running add the following command:
// $> cargo run --example file_line
//

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    println!("Reading a file line by line via BufReader::lines()");

    // Creates a File object that requires a path argument
    // and error handling if the file does not exist. This program
    // crashes if a readme.md is not present.
    let f = File::open("readme.md").unwrap();
    let reader = BufReader::new(f);
    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("--> {} ({} bytes long)", line, line.len());
    }
}
