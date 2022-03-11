// This uses the db schema to get data from the database into the 
// data structures defined here so rust can interact with it.

// So we created a pgsql table, created a table schema which is kind of the mapping 
// and then now we're creating the data structure that we'll interact with in our Rust code.

// I guess what the schema does is map the relation between sql and rust.


// if we don't import this, the table won't understand the reference
use crate::schema::to_do;

// states that we allow the date to be inserted into the database
#[derive(Insertable)]
// defines the table as belonging to this struct
#[table_name="to_do"]
// NewItem doesn't have an id value because the id of our model is
// not defined beforehand
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub user_id: i32
}

impl NewItem {
    pub fn new(title: String, user_id: i32) -> NewItem {
        return NewItem{title, status: String::from("pending"),
            user_id } 
    }
}