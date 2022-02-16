// to be used by the app views for reading html files

use std::fs; 

// reads data from a file into a string
pub fn read_file(file_path: &str) -> String {
    let data: String = fs::read_to_string(
        file_path).expect("Unable to read file");
    return data
}