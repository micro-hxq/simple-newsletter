use std::io::Error;
use std::net::TcpListener;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn health(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("ok")
}

pub async fn run_server(addr: &str) -> Result<(), Error> {
    let listener = TcpListener::bind(addr).unwrap();
    run(listener)?.await
}

pub fn run(listener: TcpListener) -> Result<Server, Error> {
    let server = HttpServer::new(|| App::new().route("/health", actix_web::web::get().to(health)))
        .listen(listener)
        .expect("")
        .run();
    println!("server start");
    Ok(server)
}

pub fn spawn_app() -> String {
    let listener = TcpListener::bind("0.0.0.0:0").unwrap();
    let addr = listener.local_addr().unwrap();
    let host = format!("http://{}:{}", addr.ip(), addr.port());
    let server = run(listener).unwrap();
    tokio::spawn(server);
    host
}
