// This contains the functions for reading and writing to and from the state
// of the to_do app. (State means the content of the app)

use std::fs::File;
use std::fs;
use std::io::Read;

use serde_json::Map;
use serde_json::value::Value;
use serde_json::json;

// This function returns the content of a json file as a data structure(a serde Map)
pub fn read_file(file_name: &str) -> Map<String, Value> {
    // open file
    let mut file = File::open(file_name.to_string()).unwrap();
    // create String for reading file contents into
    let mut data = String::new();
    // read contents into String
    file.read_to_string(&mut data).unwrap();
    // use serde's from_str() function. It returns a "Value" datatype(represents json format) 
    let json: Value = serde_json::from_str(&data).unwrap();
    // convert the Value datatype into a Map<String, Value> and return
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    return state;

    // File -> String -> Json format -> Map<String, Value> (state)

}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    // converts back to json
    let new_data = json!(state);
    // converts back to a string and writes to file
    fs::write(file_name.to_string(), new_data.to_string()).expect(
        "Unable to write file");

    // Map<String, Value> (state) -> Json format -> String -> File
}