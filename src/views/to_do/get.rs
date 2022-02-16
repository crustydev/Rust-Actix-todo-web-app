use actix_web::{web, Responder};
use serde_json::value::Value;
use serde_json::Map;
use crate::state::read_file;
use crate::to_do::{ItemTypes, to_do_factory};
use crate::json_serialization::to_do_items::ToDoItems;


// The difference here is that instead of returning the state directly as it is 
// represented in the json file, we return it in a particular format by defining a
// struct specifically for that purpose.

pub async fn get() -> impl Responder {
    let state:Map<String, Value> = read_file("./state.json");
    
    let mut array_buffer = Vec::new();

    for(key, value) in state {
        let item_type: String = String::from(value.as_str().unwrap());
        let item: ItemTypes = to_do_factory(&item_type, &key).unwrap();
        array_buffer.push(item);
    }
    let return_package: ToDoItems = ToDoItems::new(array_buffer);
    return web::Json(return_package);
}