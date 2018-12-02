//! Holds utility methods used for solving the
//! [Advent Of Code](http://adventofcode.com/) tasks.


use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Reads the file located at `input_path` and returns the content as a string.
///
/// Arguments
/// ---------
/// * `input_path` - The path of the file to read.
pub fn read_input
<P: AsRef<Path>>
(input_path: P) -> String {

    let mut file = File::open(input_path).expect("Failed to open input file");

    let mut file_content = String::new();
    let result = file.read_to_string(&mut file_content);

    match result {
        Ok(_) => file_content,
        Err(_) => panic!("Failed to read input file")
    }
}
