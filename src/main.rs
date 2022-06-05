use std::{fs, thread};
use std::collections::HashMap;
use std::fs::{create_dir, File, OpenOptions};
use std::io::{BufRead, Read, stdin, StdinLock, Write};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use tokio::runtime;

use rxiv::client::PixivClient;
use rxiv::download_full;
use rxiv::web_server::*;
use rxiv::web_server::AppData;

fn main() {
    let config = HashMap::<String, String>::new();

    let _config_file = Path::new("server.properties");

    let (mut address,mut port) = ("127.0.0.1".to_string(), 8848);
    if !_config_file.is_file() {
        File::create(_config_file).unwrap().write_all(b"address=127.0.0.1\nport=8848").unwrap();
    }
    else {
        let mut str = String::new();
        File::open(_config_file).unwrap().read_to_string(&mut str).expect("Cannot open file");

        for line in str.split('\n') {
            let mut s: Vec<&str> = line.split('=').collect();
            if s.len() < 2 { panic!("Properties file unknown") }

            let first = s.swap_remove(0);

            if "address" == first { address = s.swap_remove(0).to_string(); }
            if "port" == first { port = s.swap_remove(0).parse().unwrap(); }
        };
    }

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

        let server = HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(data.clone()))
                .service(info)
                .service(rank)
                .service(get_illust)
        })
            .bind((address.as_str(), port))
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