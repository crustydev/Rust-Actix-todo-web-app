// this schema acts as a representation for
// the database itself.

table! {
    to_do (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        user_id -> Int4,
    }
}
table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        unique_id  -> Varchar
    }
}