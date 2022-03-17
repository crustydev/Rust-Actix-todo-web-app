
use actix_web::{Responder, HttpRequest};
use super::utils::return_state;

use crate::auth::jwt::JwtToken;

// The difference here is that instead of returning the state directly as it is 
// represented in the json file, we return it in a particular format by defining a
// struct specifically for that purpose.

pub async fn get(req: HttpRequest) -> impl Responder {
    let token: JwtToken = 
        JwtToken::decode_from_request(req).unwrap();
    return return_state(&token.user_id);
}