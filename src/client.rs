use std::net::{Ipv4Addr, SocketAddrV4};

use native_tls::TlsConnector;
use reqwest::{Client, ClientBuilder, header, IntoUrl};
use reqwest::header::{HeaderMap, HeaderValue};

use crate::api::ApiResponse;
use crate::api::illust_info::IllustPages;
use crate::api::rank::Rank;

pub struct PixivClient {
    http_client: Client,
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

        //header.insert("authority", HeaderValue::from_static("www.pixiv.net"));
        //header.insert("upgrade-insecure-requests", HeaderValue::from(1));

        let client = ClientBuilder::new()
            .default_headers(header)
            //.user_agent("PixivIOSApp/6.0.4 (iOS 9.0.2; iPhone6,1)")
            .user_agent("PixivIOSApp/7.14.8 (iOS 15.5; iPhone14,5)")
            .cookie_store(true)
            .no_proxy()
            .use_preconfigured_tls(connector)
            .resolve("www.pixiv.net", SocketAddrV4::new(Ipv4Addr::new(210, 140, 131, 199), 443).into())
            .resolve("app-api.pixiv.net", SocketAddrV4::new(Ipv4Addr::new(210, 140, 131, 199), 443).into())
            .resolve("oauth.secure.pixiv.net", SocketAddrV4::new(Ipv4Addr::new(210, 140, 131, 219), 443).into())
            .resolve("i.pximg.net", SocketAddrV4::new(Ipv4Addr::new(210, 140, 92, 144), 443).into())
            .resolve("s.pximg.net", SocketAddrV4::new(Ipv4Addr::new(210, 140, 92, 143), 443).into())
            .build().unwrap();

        Self {
            http_client: client
        }
    }

    pub fn client(&self) -> &Client {
        &self.http_client
    }

    pub async fn get_api<U: IntoUrl>(&self, url: U) -> reqwest::Result<ApiResponse> {
        let response = self.client().get(url).send().await?;
        let api_res = ApiResponse::from_http_response(response).await;

        Ok(api_res)
    }

    pub async fn rank(&self, page: u16) -> reqwest::Result<Rank> {
        let page = page.to_string();

        /*let mut params = HashMap::new();
        params.insert("format", "json");
        params.insert("p", page.as_str());*/

        let response = self.client().get(format!("https://www.pixiv.net/ranking.php?format=json&p={}", page))
            .header("user-agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36 Edg/91.0.864.37"))
            //.form(&params)
            .send()
            .await?;

        let rank: Rank = serde_json::from_value(response.json().await?).unwrap();
        Ok(rank)
    }

    pub async fn illust_pages(&self, id: u32) -> reqwest::Result<Vec<IllustPages>> {
        let response = self.get_api(format!("https://www.pixiv.net/ajax/illust/{}/pages?lang=zh", id)).await?;
        let page: Vec<IllustPages> = serde_json::from_value(response.body()).unwrap();
        Ok(page)
    }
}

impl Default for PixivClient {
    fn default() -> Self {
        Self::new()
    }
}