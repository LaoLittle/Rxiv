use std::{fs, thread};
use std::fs::create_dir;
use std::io::{BufRead, stdin, StdinLock};
use std::path::PathBuf;
use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use tokio::runtime;

use rxiv::client::PixivClient;
use rxiv::download_full;
use rxiv::web_server::*;
use rxiv::web_server::AppData;

fn main() {
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(24)
        .build()
        .unwrap();

    let runtime = Arc::new(runtime);
    let runtime_li = runtime.clone();

    let mut dir = PathBuf::from("images");
    if !dir.is_dir() { create_dir(&dir).unwrap(); }
    dir.push("cache");
    if !dir.is_dir() { create_dir(&dir).unwrap(); }

    println!("Successfully started");

    let data = AppData {
        pixiv_client: Arc::new(Default::default())
    };

    let handle = thread::spawn(move || {
        let port = 8080;

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(data.clone()))
                .service(info)
                .service(rank)
                .service(get_illust)
        })
            .bind(("127.0.0.1", port))
            .unwrap_or_else(|e| panic!("{e:?}, Cannot bind port {}", port))
            .run();

        let handle = server.handle();

        let rt = runtime_li.clone();
        runtime_li.spawn(async move {
            if let Err(e) = rt.spawn(async move {
                println!("Successfully started server, listening on {port}");
                server.await.unwrap();
            }).await {
                println!("Unable to start server {e:?}");
            };
        });


        let client = PixivClient::new();
        let client = Arc::new(client);
        let mut stdin = stdin().lock();
        let mut buf = String::new();

        fn clear_and_read(buf: &mut String, stdin: &mut StdinLock) -> std::io::Result<usize> {
            buf.clear();
            stdin.read_line(buf)
        }

        let rt = runtime_li;
        while clear_and_read(&mut buf, &mut stdin).is_ok() {
            let str = buf.trim_end();

            match str {
                "exit" | "stop" => {
                    println!("Stopping...");
                    rt.block_on(async move {
                        handle.stop(true).await
                    });
                    break;
                }

                _ => {}
            }

            let id: u32 = match str.parse() {
                Ok(id) => id,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            let client = client.clone();
            rt.spawn(async move {
                download_full(&*client, id).await.unwrap();
            });
        }
    });

    handle.join().unwrap();

    if dir.is_dir() { fs::remove_dir_all(&dir).unwrap(); }
    drop(runtime);
}