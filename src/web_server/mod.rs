use std::path::PathBuf;
use std::sync::Arc;

use actix_web::{get, HttpResponse, web};
use serde::Deserialize;
use tokio::io::AsyncReadExt;

use crate::client::PixivClient;
use crate::download_full;

#[derive(Clone)]
pub struct AppData {
    pub pixiv_client: Arc<PixivClient>,
}

impl AppData {
    fn client(&self) -> &PixivClient {
        &self.pixiv_client
    }
}

#[get("/info/{pid}")]
pub async fn info(id: web::Path<u32>, data: web::Data<AppData>) -> actix_web::Result<String> {
    let client = &data.pixiv_client;
    let pages = client.illust_pages(id.into_inner());

    Ok(format!("{:?}", pages.await.unwrap()))
}

#[derive(Deserialize, Debug)]
pub struct Page {
    p: u16,
}

#[get("/rank")]
pub async fn rank(query: Option<web::Query<Page>>, data: web::Data<AppData>) -> actix_web::Result<String> {
    let client = &data.pixiv_client;
    let rank = match client.rank(query.map(|q| { q.p }).unwrap_or(1)).await {
        Ok(r) => r,
        Err(e) => return Ok(e.to_string())
    };

    Ok(format!("{:#?}", rank))
}

#[get("/image/{pid}")]
pub async fn get_illust(pid: web::Path<u32>, query: Option<web::Query<Page>>, data: web::Data<AppData>) -> HttpResponse {
    let (pid, page) = (pid.into_inner(), query.map(|q| { q.p }).unwrap_or(0));

    let no_content = HttpResponse::NoContent().finish();

    println!("{pid}_p{page}");
    let client = data.client();
    if (download_full(client, pid).await).is_err() { return no_content; }

    let mut img_path = PathBuf::from("images");

    let dir = if let Ok(dir) = img_path.read_dir() { dir } else { return no_content; };

    for entry in dir.flatten() {
        let file_name = entry.file_name();

        let m = format!("{pid}_p{page}");

        if file_name.to_str().unwrap().contains(m.as_str()) {
            img_path.push(file_name);
            break;
        };
    }

    let mut builder = HttpResponse::Ok();
    let res: HttpResponse;

    if img_path.is_file() {
        let mut f = tokio::fs::File::open(&img_path).await.unwrap();

        let mut buf = Vec::with_capacity(f.metadata().await.unwrap().len() as usize);
        if f.read_to_end(&mut buf).await.is_err() { return HttpResponse::NoContent().finish(); };

        let bytes: web::Bytes = buf.into();

        res = builder.body(bytes);
    } else { return no_content; }

    res
}