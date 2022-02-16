
use actix_web::Responder;
use super::utils::return_state;


// The difference here is that instead of returning the state directly as it is 
// represented in the json file, we return it in a particular format by defining a
// struct specifically for that purpose.

pub async fn get() -> impl Responder {
    return return_state();
}