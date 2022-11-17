/*
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
mod nmr_queue;

async fn greet(req: HttpRequest) -> impl Responder {
    HttpRequest
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
*/

use nmr_queue::NmrQueue;
mod nmr_queue;

fn a<T>(ab: T) -> String where T: AsRef<str>{
    return ab.as_ref();
}

fn main() {
    let mut queue = NmrQueue::new();
    let ad = "bacsd";
    let b = a(ad);
}