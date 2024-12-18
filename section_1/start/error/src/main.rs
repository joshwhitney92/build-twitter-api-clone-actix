mod simple;

use actix_web::{HttpServer, App};
use crate::simple::get;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", actix_web::web::get().to(get))

    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}
