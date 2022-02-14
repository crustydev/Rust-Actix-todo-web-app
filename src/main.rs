mod state;
mod processes;
mod to_do;

use std::env;
use state::read_file;
use serde_json::value::Value;
use serde_json::Map;
use to_do::to_do_factory;
use processes::process_input;

fn main() {
    // get variables from command line arguments 
    let args: Vec<String> = env::args().collect();

    //get command
    let command: &String = &args[1];
    //get title
    let title: &String = &args[2];

    //get state from json file
    let state: Map<String, Value> = read_file("./state.json");

    let status: String;

    //get status if it exists in state, set status to pending if it doesn't exist
    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = String::from("pending");
        }
    }

    // Creates item object for manipulation. it is an abstract representation of the todo components of our state.
    let item = to_do_factory(&status, title).expect(&status);

    
    // If command is "Create" then it adds it to the state.
    // If command is "Edit" then it replaces the field with which it shares a title in the state.
    // If command is "Delete" then it deletes the field that shares its title.
    // If command is "Get" then it gets the field that shares its title.
    process_input(item, command.to_string(), &state);
}