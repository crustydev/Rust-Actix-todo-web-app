//manages to_do views, should contain a factory
use actix_web::web;
mod create;
mod get;
mod utils;
mod edit;
mod delete;
use super::path::Path;


//this function tells the application what function to call for whatever http route
pub fn item_factory(app:&mut web::ServiceConfig) {
    //define base_path to avoid repitition
    let base_path: Path = Path{prefix: String::from("/item"),
                                backend: true};

    //create view accepts a POST request
    app.route(&base_path.define(String::from("/create/{title}")),
                web::post().to(create::create))
        .route(&base_path.define(String::from("/get")),
                web::get().to(get::get))
        .route(&base_path.define(String::from("/edit")),
                web::put().to(edit::edit))
        .route(&base_path.define(String::from("/delete")),
                web::post().to(delete::delete));
}