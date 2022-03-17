//when reading this all over again, try to consider what and
//what blocks of code are for the server side/client side.


#[macro_use] extern crate diesel;
extern crate dotenv;

use actix_web::{App, HttpServer, HttpResponse};
use actix_service::Service;
use futures::future::{ok, Either};

use log;
use env_logger;

mod views;
mod to_do;
mod json_serialization;
mod schema;
mod database;
mod models;
mod auth;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
   env_logger::init();
   HttpServer::new( || {
       let app = App::new()
       .wrap_fn(|req, srv| {
           //The service request will be inaccessible after 
           //it's passed through the call function, hence we
           //clone the request's URI path so the URl is 
           //available throughout the server process.
           let request_url: String = String::from(
               *&req.uri().path().clone());
           let passed: bool;

           if *&req.path().contains("/item/") {
               match auth::process_token(&req) {
                   Ok(_token) => {passed = true;},
                   Err(_message) => {passed = false;}
               };
           } else { passed = true; }

           let end_result = match passed {
               true => {
                   Either::Left(srv.call(req))
               },
               false => {
                   Either::Right(
                       ok(req.into_response(
                           HttpResponse::Unauthorized()
                            .finish()
                            .into_body()))
                   )
               }
           };
        
           //we use an async move block instead of an async
           //block because the async block might outlive our 
           //URL string. The async move block moves ownership
           //into the block. 
           async move {
               let result = end_result.await?;
               log::info!("{} -> {}", request_url,
                        &result.status());
               Ok(result)
           }
       })
       .configure(views::views_factory);
       return app
   })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}




