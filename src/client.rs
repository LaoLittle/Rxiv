use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::Arc;
use native_tls::TlsConnector;
use reqwest::{Certificate, Client, ClientBuilder, header, Proxy};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::redirect::Policy;

pub struct PixivClient {
    http_client: Client
}

impl PixivClient {
    pub fn new() -> PixivClient {
        let connector = TlsConnector::builder()
            .use_sni(false)
            .build().unwrap();

        let mut header = HeaderMap::new();
        header.insert(header::REFERER, HeaderValue::from_static("https://www.pixiv.net/"));
        //header.insert(header::HOST, HeaderValue::from_static("i.pximg.net"));

        let client = ClientBuilder::new()
            //.no_trust_dns()
            //.danger_accept_invalid_certs(true)
            //.danger_accept_invalid_hostnames(true)
            .default_headers(header)
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.64 Safari/537.36 Edg/101.0.1210.53")
            .cookie_store(true)
            .use_preconfigured_tls(connector)
            //.proxy(Proxy::http("https://127.0.0.1:7890").unwrap())
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
}
