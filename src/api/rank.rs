use serde_json::Value;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Rank {
    contents: Vec<Content>,
    mode: String,
    content: String,
    page: u16,
    prev: Value,
    next: Value,
    date: String,
    prev_date: String,
    next_date: bool,
    rank_total: u16,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    title: String,
    date: String,
    tags: Vec<String>,
    url: String,
    illust_type: String,
    illust_book_style: String,
    illust_page_count: String,
    user_name: String,
    profile_img: String,
    illust_content_type: Value,
    illust_series: Value,
    illust_id: u32,
    width: u16,
    height: u16,
    user_id: u32,
    rank: u16,
    yes_rank: u16, // prev
    rating_count: u16,
    view_count: u32,
    illust_upload_timestamp: u32, // second
    attr: String,
    #[serde(default)]
    is_bookmarked: bool,
    #[serde(default)]
    bookmarkable: bool,
}