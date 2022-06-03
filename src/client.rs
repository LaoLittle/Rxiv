use std::collections::HashMap;
use std::fs::{create_dir, File, OpenOptions};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use native_tls::TlsConnector;
use reqwest::{Certificate, Client, ClientBuilder, header, IntoUrl, Proxy};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::redirect::Policy;
use crate::api::ApiResponse;
use crate::api::illust_info::IllustPages;

pub struct PixivClient {
    http_client: Client
}

impl PixivClient {
    pub fn new() -> PixivClient {
        let connector = TlsConnector::builder()
            .use_sni(false)
            .build().unwrap();

        let mut header = HeaderMap::new();
        header.insert(header::REFERER, HeaderValue::from_static("https://app-api.pixiv.net/"));
        header.insert("APP-OS", HeaderValue::from_static("ios"));
        header.insert("App-OS-Version", HeaderValue::from_static("15.5"));
        header.insert("App-Version", HeaderValue::from_static("7.14.8"));

        let client = ClientBuilder::new()
            .default_headers(header)
            //.user_agent("PixivIOSApp/6.0.4 (iOS 9.0.2; iPhone6,1)")
            .user_agent("PixivIOSApp/7.14.8 (iOS 15.5; iPhone14,5)")
            .cookie_store(true)
            .no_proxy()
            .use_preconfigured_tls(connector)
            .resolve("www.pixiv.net", SocketAddrV4::new(Ipv4Addr::new(210,140,131,199), 443).into())
            .resolve("app-api.pixiv.net", SocketAddrV4::new(Ipv4Addr::new(210,140,131,199), 443).into())
            .resolve("oauth.secure.pixiv.net", SocketAddrV4::new(Ipv4Addr::new(210,140,131,219), 443).into())
            .resolve("i.pximg.net", SocketAddrV4::new(Ipv4Addr::new(210,140,92,144), 443).into())
            .resolve("s.pximg.net", SocketAddrV4::new(Ipv4Addr::new(210,140,92,143), 443).into())
            .build().unwrap();

        Self {
            http_client: client
        }
    }

    pub fn client(&self) -> &Client {
        &self.http_client
    }

    pub async fn get_api<U: IntoUrl>(&self, url: U) -> Result<ApiResponse, reqwest::Error> {
        let response = self.client().get(url).send().await?;
        let bytes = response.bytes().await.unwrap();
        let api_res: ApiResponse = serde_json::from_slice(&bytes[..]).unwrap();

        Ok(api_res)
    }

    pub async fn illust_pages(&self, id: u32) -> Result<Vec<IllustPages>, reqwest::Error> {
        let response = self.get_api(format!("https://www.pixiv.net/ajax/illust/{}/pages?lang=zh", id)).await?;
        let page: Vec<IllustPages> = serde_json::from_value(response.body()).unwrap();
        Ok(page)
    }
}
