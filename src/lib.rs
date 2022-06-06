use std::ffi::OsStr;

use std::path::PathBuf;

use std::time::SystemTime;

use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::client::PixivClient;

pub mod client;
mod api;
pub mod web_server;

pub async fn download_full(client: &PixivClient, id: u32) -> reqwest::Result<()> {
    let pages = client.illust_pages(id).await?;

    let mut image = PathBuf::from("images");
    let mut cache = image.clone();
    cache.push("cache");

    image.push("0");
    cache.push("0");

    for page in pages.iter() {
        let pic_url = page.urls().original();

        let file_name = pic_url.split('/').last().unwrap();

        image.pop();
        cache.pop();

        image.push(file_name);
        cache.push(file_name);

        if image.is_file() { continue; }
        if cache.is_file() { return Ok(()); }

        println!("Start download {}", pic_url);

        let prev = SystemTime::now();
        let mut res = client.client().get(pic_url).send().await?;

        let mut file = File::create(&cache).await.unwrap();

        while let Some(chunk) = res.chunk().await? {
            file.write_all(&chunk).await.unwrap();
        }

        if let Err(e) = fs::copy(&cache, &image).await { eprintln!("Unable to copy file: {e}") } else { fs::remove_file(&cache).await.expect("Cache file doesn't exist"); }

        let now = SystemTime::now();
        println!("Successfully downloaded {}, cost {} sec", <PathBuf as AsRef<OsStr>>::as_ref(&image).to_str().unwrap(), now.duration_since(prev).unwrap().as_secs_f32());
    }

    println!("All {} pages of PID({}) have been downloaded", pages.len(), id);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, thread};

    use std::fs::File;
    use std::io::Write;


    use std::sync::Arc;
    use std::time::SystemTime;


    use serde::{Deserialize};
    use serde_json::json;
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
            // 140-149
            let res = p.client().get("https://210.140.92.143/img-original/img/2021/12/17/00/00/03/94819771_p0.png")
                //.header("Host", "https://i.pximg.net")
                .send().await;
            let res = res.unwrap();
            println!("{}", res.status());

            let bytes = res.bytes().await.unwrap();
            let mut f = File::create("i0.png").unwrap();
            f.write_all(&bytes).expect("TODO: panic message");

            let now = SystemTime::now();
            println!("Cost: {:?}", now.duration_since(pr).unwrap());
        });
    }

    #[test]
    fn oauth() {
        let rt = runtime().unwrap();
        let p = PixivClient::new();

        rt.block_on(async move {
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
            let rank = p.rank(2).await.unwrap();

            println!("{:#?}", rank);
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

    #[test]
    fn entry() {
        let dir = fs::read_dir(".").unwrap();

        for entry in dir {
            println!("{:?}", entry.unwrap().file_name());
        }
    }

    fn runtime() -> std::io::Result<Runtime> {
        Builder::new_current_thread().enable_all().build()
    }
}