use std::alloc::System;
use std::env;
use std::ffi::OsStr;
use std::fs::{create_dir, create_dir_all, File};
use std::io::{BufRead, Read, stdin, Write};
use std::net::{SocketAddr, SocketAddrV6};
use std::path::{Path, PathBuf};
use std::process::id;
use std::sync::Arc;
use std::time::SystemTime;
use actix_web::{App, HttpServer};
use actix_web::rt::Runtime;
use reqwest::tls::Version;
use tokio::runtime;
use rxiv::client::PixivClient;
use rxiv::hello;


fn main() {
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(23)
        .build()
        .unwrap();

    let mut dir = PathBuf::from("./images");

    if !dir.is_dir() { create_dir(&dir).unwrap(); }

    runtime.block_on(async {
        println!("Successfully started");
        let client = PixivClient::new();
        let mut stdin = stdin().lock();
        let mut buf = String::new();

        while let Ok(_) = stdin.read_line(&mut buf) {
            let str = buf.trim_end();

            match str {
                "exit" => {
                    println!("?");
                    break;
                }
                _ => {

                }
            }

            let id: u32 = match str.parse() {
                Ok(id) => id,
                Err(e) => {
                    buf.clear();
                    eprintln!("{}", e);
                    continue
                }
            };

            buf.clear();

            let pages = client.illust_pages(id).await.unwrap();
            println!("Get {:?}", pages);
            let mut index = 0;
            for page in pages {
                let prev = SystemTime::now();
                let pic_url = page.urls().original();
                println!("Start download {}", pic_url);
                let res = client.client().get(pic_url).send().await.unwrap();
                let bytes = res.bytes().await.unwrap();

                dir.push(format!("{}_p{}.png", id, index));
                index += 1;

                let mut file = File::create(&dir).unwrap();
                file.write(&bytes).unwrap();
                let now = SystemTime::now();
                println!("Successfully downloaded {}, Cost {} sec", <PathBuf as AsRef<OsStr>>::as_ref(&dir).to_str().unwrap(), now.duration_since(prev).unwrap().as_secs_f32());
                dir.pop();
            }
        }
    });
    /*let server = HttpServer::new(|| {
        App::new()
            .service(hello)
    }).bind(("127.0.0.1", 8848)).unwrap().run();

    let client = reqwest::Client::builder()
        .build().expect("Cannot build client");

    let c = client.post("").body("").build().expect("");
    client.get("").send();

    let a: i32 = 1.into();

    println!("Hello, world!");

    runtime.block_on(async {
        server.await.expect("TODO: panic message");
    })*/
}