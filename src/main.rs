use actix_web::{App, HttpServer};
mod views;
mod processes;
mod to_do;
mod state;
mod json_serialization;

use actix_service::Service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new( || {
       let app = App::new()
       .wrap_fn(|req, srv| {
           if *&req.path().contains("/item/") {
               match views::token::process_token(&req) {
                   Ok(_token) => println!("the token is passable"),
                   Err(message) => println!("token error: {}", message)
               }
           }
           let fut = srv.call(req);
           async {
               let result = fut.await?;
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

