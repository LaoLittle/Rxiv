use std::net::{SocketAddr, SocketAddrV6};
use std::sync::Arc;
use actix_web::{App, HttpServer};
use actix_web::rt::Runtime;
use reqwest::tls::Version;
use tokio::runtime;
use rxel::hello;


fn main() {
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(23)
        .build()
        .unwrap();
    let server = HttpServer::new(|| {
        App::new()
            .service(hello)
    }).bind(("127.0.0.1", 8848)).unwrap().run();

    let client = reqwest::Client::builder()
        .build().expect("Cannot build client");

    let c = client.post("").body("").build().expect("");
    client.get("").send();



    println!("Hello, world!");

    runtime.block_on(async {
        server.await.expect("TODO: panic message");
    })
}