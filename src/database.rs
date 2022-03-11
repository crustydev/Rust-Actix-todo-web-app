use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;

use std::env;

//establishes and returns a connection to the database
pub fn establish_connection() -> PgConnection {
    //ensures that the program does not throw an error if we fail to
    //load the environment
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", 
            database_url))
}