use actix_web::web;
mod path;
mod auth;
mod to_do;

pub fn views_factory(app:&mut web::ServiceConfig) {
    auth::auth_factory(app);
    to_do::item_factory(app);
}