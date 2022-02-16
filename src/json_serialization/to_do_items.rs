// define our json_serialization structs, that is an object representation of how we want
// our json data to be formatted.

use serde::Serialize;
use crate::to_do::ItemTypes;
use crate::to_do::structs::base::Base;


/* json formatted data will look something like:
    {
        "pending_items":
        [
        {"title":"shopping","status":"pending"},
        {"title":"test","status":"pending"},
        {"title":"washing","status":"pending"}
        ],

        "done_items":[{"title":"singing","status":"done"}],

        "pending_item_count":3,

        "done_item_count":1
    }
*/

// implement the Serialize trait
#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8
}

// Constructor for the json_serialization struct that initializes its elements from
// a vector of all items present in the json file. 
impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> ToDoItems {
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(packed) =>
                pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) =>
                done_array_buffer.push(packed.super_struct)
            }
        }

        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        return ToDoItems{
            pending_items: pending_array_buffer,
            done_items: done_array_buffer,
            pending_item_count: pending_count,
            done_item_count: done_count
        }
    }
}