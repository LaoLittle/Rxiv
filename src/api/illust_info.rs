use serde::Deserialize;
use serde_json::Value;

use crate::api::ApiResponse;

#[derive(Deserialize, Debug)]
pub struct IllustInfo {
    #[serde(rename = "illustId")]
    illust_id: String,
    #[serde(rename = "illustTitle")]
    illust_title: String,
    #[serde(rename = "illustComment")]
    illust_comment: String,
    id: String,
    title: String,
    description: String,
    #[serde(rename = "illustType")]
    illust_type: u8,
    #[serde(rename = "createDate")]
    create_date: String,
    #[serde(rename = "uploadDate")]
    upload_date: String,
    restrict: u8,
    #[serde(rename = "xRestrict")]
    x_restrict: u8,
    urls: Value,
    tags: Value,
    alt: String,
    #[serde(rename = "storableTags")]
    storable_tags: Value,
    #[serde(rename = "userId")]
    user_id: String,
    #[serde(rename = "userName")]
    user_name: String,
    #[serde(rename = "userAccount")]
    user_account: String,
    #[serde(rename = "userIllusts")]
    user_illusts: Value,
    #[serde(rename = "likeData")]
    like_data: bool,
    width: u16,
    height: u16,
    #[serde(rename = "pageCount")]
    page_count: u16,
    #[serde(rename = "likeCount")]
    like_count: u32,
    #[serde(rename = "responseCount")]
    response_count: u32,
    #[serde(rename = "pollData")]
    poll_data: Value,
    #[serde(rename = "seriesNavData")]
    series_nav_data: Value,
    #[serde(rename = "descriptionBoothId")]
    description_booth_id: Value,
    #[serde(rename = "descriptionYoutubeId")]
    description_youtube_id: Value,
    #[serde(rename = "comicPromotion")]
    comic_promotion: Value,
    #[serde(rename = "fanboxPromotion")]
    fanbox_promotion: Value,
    #[serde(rename = "contestBanners")]
    contest_banners: Vec<Value>,
    #[serde(rename = "isBookmarkable")]
    is_bookmarkable: bool,
    #[serde(rename = "bookmarkCount")]
    bookmark_count: u32,
    #[serde(rename = "bookmarkData")]
    bookmark_data: Value,
    #[serde(rename = "contestData")]
    contest_data: Value,
    #[serde(rename = "zoneConfig")]
    zone_config: Value,
    #[serde(rename = "extraData")]
    extra_data: Value,
    #[serde(rename = "titleCaptionTranslation")]
    title_caption_translation: Value,
    #[serde(rename = "isUnlisted")]
    is_unlisted: bool,
    #[serde(rename = "isHowto")]
    is_howto: bool,
    #[serde(rename = "isOriginal")]
    is_original: bool,
    request: Value,
    #[serde(rename = "commentCount")]
    comment_count: u32,
    #[serde(rename = "commentOff")]
    comment_off: u8,
}

impl IllustInfo {
    pub fn illust_id(&self) -> u32 {
        self.illust_id.parse().unwrap()
    }

    pub fn illust_title(&self) -> &str {
        self.illust_title.as_str()
    }

    pub fn illust_comment(&self) -> &str {
        self.illust_comment.as_str()
    }

    pub fn id(&self) -> u32 {
        self.id.parse().unwrap()
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }
}

impl<'a> TryInto<IllustInfo> for ApiResponse {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<IllustInfo, Self::Error> {
        let info: IllustInfo = serde_json::from_value(self.body)?;
        Ok(info)
    }
}

#[derive(Deserialize, Debug)]
pub struct IllustPages {
    urls: PageUrls,
    width: u16,
    height: u16,
}

impl IllustPages {
    pub fn urls(&self) -> &PageUrls {
        &self.urls
    }

    pub fn image_info(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

#[derive(Deserialize, Debug)]
pub struct PageUrls {
    thumb_mini: String,
    small: String,
    regular: String,
    original: String,
}

impl PageUrls {
    pub fn thumb_mini(&self) -> &str {
        &self.thumb_mini
    }

    pub fn small(&self) -> &str {
        &self.small
    }

    pub fn regular(&self) -> &str {
        &self.regular
    }

    pub fn original(&self) -> &str {
        &self.original
    }
}