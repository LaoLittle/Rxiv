extern crate reqwest;
mod client;

use std::thread;
use std::time::Duration;
use actix_web::{get, HttpResponse, Responder};
use actix_web::http::StatusCode;

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
    use std::io::{Read, Write};
    use tokio::runtime::{Builder, Runtime};
    use crate::client::{PixivClient};

    #[test]
    fn connect() {
        let rt = runtime().unwrap();
        let p = PixivClient::new();
        let client = p.client();

        rt.block_on(async {
            let res = client.get("https://i.pximg.net/img-master/img/2022/05/24/17/26/10/98571977_p0_master1200.jpg").send().await;
            let res = res.unwrap();

            let b = res.bytes().await.unwrap();
            println!("{}", String::from_utf8_lossy(&b[..]));
            println!("{}",b.len());
            let mut f = File::create("i.jpg").unwrap();
            f.write_all(&b[..]).expect("");
        });
    }

    fn runtime() -> std::io::Result<Runtime> {
        Builder::new_current_thread().enable_all().build()
    }
}
