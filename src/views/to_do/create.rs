use crate::diesel;
use diesel::prelude::*;

use actix_web::HttpRequest;
use actix_web::Responder;

use crate::database::establish_connection;
use crate::models::item::new_item::NewItem;
use crate::models::item::item::Item;

use crate::schema::to_do;
use super::utils::return_state;

use crate::auth::jwt::JwtToken;


pub async fn create(req: HttpRequest) -> impl Responder {
    let title: String = req.match_info().get("title")
    .unwrap().to_string();
    let title_ref: String = title.clone();

    // we unwrap rather than match because we know that if 
    // a request gets to this view, it has been allowed through
    // by the middleware and has a token.
    let token: JwtToken = JwtToken::decode_from_request(
                            req).unwrap();

    let connection = establish_connection();

    //start query, checking ONLY for entries that have title "title_ref"
    let items = to_do::table
        .filter(to_do::columns::title.eq(
            title_ref.as_str()))
        .filter(to_do::columns::user_id.eq(
            &token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&connection)
        .unwrap();
    
    // only create if entry doesn't exist already
    if items.len() == 0 {
        let new_post = NewItem::new(title, token.user_id.clone());
        let _ = diesel::insert_into(to_do::table)
                .values(&new_post)
                .execute(&connection);       
    }

    return return_state(&token.user_id)
}

