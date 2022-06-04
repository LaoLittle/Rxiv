use std::ffi::OsStr;
use std::fs::{create_dir, File};
use std::io::{BufRead, stdin, StdinLock, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::runtime;

use rxiv::client::PixivClient;

fn main() {
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .worker_threads(23)
        .build()
        .unwrap();

    fn clear_and_read(buf: &mut String, stdin: &mut StdinLock) -> std::io::Result<usize> {
        buf.clear();
        stdin.read_line(buf)
    }

    println!("Successfully started");
    let client = PixivClient::new();
    let client = Arc::new(client);
    let mut stdin = stdin().lock();
    let mut buf = String::new();

    let dir = PathBuf::from("images");

    if !dir.is_dir() { create_dir(&dir).unwrap(); }

    while let Ok(_) = clear_and_read(&mut buf, &mut stdin) {
        let str = buf.trim_end();

        match str {
            "exit" => {
                println!("?");
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

        let mut dir = dir.clone();
        let client = client.clone();
        runtime.spawn(async move {
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
        });
    }

    /*runtime.block_on(async {
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
    });*/
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