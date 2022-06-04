extern crate reqwest;

use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::SystemTime;

use actix_web::{get, web};

use crate::client::PixivClient;
use crate::web_server::AppData;

pub mod client;
mod api;
pub mod web_server;

#[get("/info/i={p}")]
pub async fn info(id: web::Path<u32>, data: web::Data<AppData>) -> actix_web::Result<String> {
    let client = &data.pixiv_client;
    let pages = client.illust_pages(id.into_inner());

    Ok(format!("{:?}", pages.await.unwrap()))
}

#[get("/")]
pub async fn root() -> String {
    String::from("Hello!")
}

#[get("/path/{str}")]
pub async fn pp(str: web::Path<String>) -> String {
    format!("Hello! {str}")
}

pub async fn download_full(client: &PixivClient, id: u32) -> reqwest::Result<()> {
    let pages = client.illust_pages(id).await.unwrap();

    let mut image = PathBuf::from("images");

    let mut cache = image.clone();
    cache.push("cache");

    for page in pages {
        let pic_url = page.urls().original();

        let file_name = pic_url.split('/').last().unwrap();

        image.push(file_name);

        if image.is_file() {
            image.pop();
            continue;
        }

        cache.push(file_name);

        if cache.is_file() {
            cache.pop();
            continue;
        }

        println!("Start download {}", pic_url);

        let prev = SystemTime::now();
        let mut res = client.client().get(pic_url).send().await?;

        let mut file = File::create(&cache).unwrap();

        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk).unwrap();
        }

        drop(file);

        if let Err(e) = fs::copy(&cache, &image) {
            eprintln!("Unable to copy file: {e}")
        }

        let now = SystemTime::now();
        println!("Successfully downloaded {}, cost {} sec", <PathBuf as AsRef<OsStr>>::as_ref(&image).to_str().unwrap(), now.duration_since(prev).unwrap().as_secs_f32());

        image.pop();
        cache.pop();
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::sync::Arc;
    use std::thread;
    use std::time::SystemTime;

    use reqwest::header::HeaderValue;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use tokio::runtime::{Builder, Runtime};

    use crate::api::ApiResponse;
    use crate::api::illust_info::IllustInfo;
    use crate::client::PixivClient;

    #[test]
    fn connect() {
        let rt = runtime().unwrap();
        let p = PixivClient::new();
        let p = Arc::new(p);

        let rt = Arc::new(rt);
        let rt0 = rt.clone();

        rt.block_on(async {
            let p = p;
            let pp = p.clone();
            let _b0 = rt0.spawn(async move {
                let pr = SystemTime::now();

                //let res = pp.client().get("https://i.pximg.net/img-original/img/2021/12/17/00/00/03/94819771_p0.png").send().await;
                //https://app-api.pixiv.net/ajax/illust/98672628?lang=zh
                // 98495751
                // https://www.pixiv.net/ajax/illust/91393405/pages?lang=zh
                let res = pp.client().get("https://www.pixiv.net/ajax/illust/91393405?lang=zh").send().await;
                let res = res.unwrap();

                let b = res.bytes().await.unwrap();

                let res = ApiResponse::from_slice(&b[..]);

                let info = <ApiResponse as TryInto<IllustInfo>>::try_into(res.unwrap());

                println!("{:#?}", info);

                //println!("{}", String::from_utf8_lossy(&b[..]));
                println!("{}", b.len());
                //let mut f = File::create("i0.png").unwrap();
                //f.write_all(&b[..]).expect("");
                let now = SystemTime::now();
                println!("Cost: {:?}", now.duration_since(pr).unwrap());
            }).await;
        });
    }

    #[test]
    fn pages() {
        let rt = runtime().unwrap();
        let p = PixivClient::new();
        let p = Arc::new(p);

        let rt = Arc::new(rt);
        let rt0 = rt.clone();

        rt.block_on(async {
            let p = p;
            let pp = p.clone();
            let _b0 = rt0.spawn(async move {
                let pr = SystemTime::now();

                //let res = pp.client().get("https://i.pximg.net/img-original/img/2021/12/17/00/00/03/94819771_p0.png").send().await;
                //https://app-api.pixiv.net/ajax/illust/98672628?lang=zh
                // 98495751
                let res = pp.illust_pages(91393405).await;
                let res = res.unwrap();

                println!("{:#?}", res);

                //println!("{}", String::from_utf8_lossy(&b[..]));
                println!("{}", res.len());
                //let mut f = File::create("i0.png").unwrap();
                //f.write_all(&b[..]).expect("");
                let now = SystemTime::now();
                println!("Cost: {:?}", now.duration_since(pr).unwrap());
            }).await;
        });
    }

    #[test]
    fn get_ill() {
        let rt = runtime().unwrap();
        let p = PixivClient::new();

        let rt = Arc::new(rt);

        rt.block_on(async move {
            let pr = SystemTime::now();
            let res = p.client().get("https://i.pximg.net/img-original/img/2021/12/17/00/00/03/94819771_p0.png").send().await;
            let res = res.unwrap();

            let b = res.bytes().await.unwrap();
            println!("{}", b.len());
            let mut f = File::create("i0.png").unwrap();
            f.write_all(&b[..]).expect("");
            let now = SystemTime::now();
            println!("Cost: {:?}", now.duration_since(pr).unwrap());
        });
    }

    #[test]
    fn oauth() {
        let rt = runtime().unwrap();
        let p = PixivClient::new();

        rt.block_on(async move {
            #[derive(Debug, Serialize, Deserialize)]
            struct A {
                inn: i32,
                will: String,
            }

            let res = p.client()
                .post("https://oauth.secure.pixiv.net/auth/token")
                .body("{}")
                .send().await;
            let res = res.unwrap();

            let b = res.bytes().await.unwrap();
            println!("{}", String::from_utf8_lossy(&b[..]));
        });
    }

    #[test]
    fn rank() {
        // https://www.pixiv.net/ranking.php?mode=daily&p=1&format=json
        let rt = runtime().unwrap();
        let p = PixivClient::new();

        let rt = Arc::new(rt);

        rt.block_on(async move {
            let pr = SystemTime::now();
            let res = p.client().get("https://www.pixiv.net/ranking.php?p=1&format=json")
                .header("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36 Edg/91.0.864.37"))
                .send().await;
            let res = res.unwrap();

            let json: Value = res.json().await.unwrap();

            println!("{:#}", json);
            let now = SystemTime::now();
            println!("Cost: {:?}", now.duration_since(pr).unwrap());
        });
    }

    #[test]
    fn json() {
        #[derive(Debug, Deserialize)]
        struct Res {
            res: Option<u8>,
        }

        #[derive(Deserialize, Debug)]
        struct Id {
            id: u8,
        }

        let bb = json!([{"id": 12}, {"id": 213}]);
        let v: Vec<Id> = serde_json::from_value(bb).unwrap();

        println!("{:#?}", v);
    }

    #[test]
    fn arc() { // Arc到底是怎么回事呢？下面就和小便一起来看看吧
        let me: Vec<u8> = vec![11, 45, 14, 19, 19, 81, 0];
        let me = Arc::new(me); //Arc其实就是Arc，小便也很难相信，但他就是那样
        for _ in 0..10 {
            let lao = me.clone(); //但是Arc不只是Arc
            thread::spawn(move || {
                let _bb = &**lao;

                assert_eq!(*lao, [11, 45, 14, 19, 19, 81, 0])
            });
            //以上就是Arc的用法了，不知道大家都明白了么
        }
    }

    fn runtime() -> std::io::Result<Runtime> {
        Builder::new_current_thread().enable_all().build()
    }
}