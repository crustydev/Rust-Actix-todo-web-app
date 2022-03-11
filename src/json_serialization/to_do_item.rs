use serde::Deserialize;


// For deserializing data passed in a request body in json form
// (deserialization means conversion into an object)

#[derive(Deserialize)]
pub struct ToDoItem {
    pub title: String,
    pub status: String
}