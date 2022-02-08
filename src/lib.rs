use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("Hello");
    format!("Hellow {}!", &name)
}
async fn health() -> impl Responder {
    HttpResponse::Ok().finish()
}
// #[tokio::main]
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/greet/{name}", web::get().to(greet))
            .route("/health_check", web::get().to(health))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
