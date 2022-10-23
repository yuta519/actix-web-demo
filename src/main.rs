mod lib;
mod tutorial;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tutorial::{echo, hello};

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lib::front_of_house::print_hoge();
    lib::front_of_house::hosting::add_to_waitlist();

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
