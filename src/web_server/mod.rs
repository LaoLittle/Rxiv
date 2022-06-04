use std::sync::Arc;

use crate::client::PixivClient;

#[derive(Clone)]
pub struct AppData {
    pub pixiv_client: Arc<PixivClient>,
}