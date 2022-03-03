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
}

impl NewItem {
    pub fn new(title: String) -> NewItem {
        return NewItem{title, status: String::from("pending")}
    }
}