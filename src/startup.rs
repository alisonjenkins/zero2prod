use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use actix_web::dev::Server;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(super::routes::health_check::health_check))
            .route("/subscriptions", web::post().to(super::routes::subscriptions::subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
