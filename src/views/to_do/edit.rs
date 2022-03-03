use crate::diesel;
use diesel::prelude::*;

use actix_web::{web, HttpResponse};

use super::utils::return_state;

use crate::database::establish_connection;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::schema::to_do;


pub async fn edit(to_do_item: web::Json<ToDoItem>) -> 
HttpResponse {
    let title_ref: String = to_do_item.title.clone();
    let connection = establish_connection();
    let results = to_do::table.filter(to_do::columns::title
        .eq(title_ref));
    
    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("done"))
        .execute(&connection);

    // returns an HttpResponse with a json body attached to it.
    return HttpResponse::Ok().json(return_state())
}