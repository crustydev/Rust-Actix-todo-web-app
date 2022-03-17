use std::vec::Vec;

// for building db queries
use crate::diesel;
use diesel::prelude::*;

// for serialization to package our items and pass them to the
// frontend
use crate::to_do::to_do_factory;
use crate::json_serialization::to_do_items::ToDoItems;

use crate::database::establish_connection;
use crate::models::item::item::Item;
use crate::schema::to_do;


pub fn return_state(user_id: &i32) -> ToDoItems {
    // establish connection with database
    let connection = establish_connection();

    // get our table and build a database query to it
    let items = to_do::table
        .order(to_do::columns::id.asc())
        .filter(to_do::columns::user_id.eq(&user_id))
        .load::<Item>(&connection)
        .unwrap();
    
    let mut array_buffer = Vec::new();

    for item in items {
        let item = to_do_factory(&item.status, &item.title)
            .unwrap();
        array_buffer.push(item);
    }

    return ToDoItems::new(array_buffer); 
}