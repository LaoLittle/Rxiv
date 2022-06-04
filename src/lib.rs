extern crate reqwest;


use std::time::Duration;

use actix_web::{get, HttpResponse, Responder};

pub mod client;
mod api;

#[get("/")]
pub async fn hello() -> impl Responder {
    println!("1");
    tokio::time::sleep(Duration::from_millis(10_000)).await;
    println!("2");

    HttpResponse::Ok().body("")
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;
    use std::sync::Arc;
    use std::time::SystemTime;

    use serde::{Deserialize, Serialize};
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

    fn runtime() -> std::io::Result<Runtime> {
        Builder::new_current_thread().enable_all().build()
    }
}