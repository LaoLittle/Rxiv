pub mod illust_info;

use std::fmt::{Debug, Display, Formatter};
use serde::Deserialize;
use serde_json::Value;
use crate::api::illust_info::IllustPages;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    error: bool,
    message: String,
    body: Value,
}

impl ApiResponse {
    pub fn from_str(s: &str) -> serde_json::Result<Self> {
        serde_json::from_str(s)
    }

    pub fn from_slice(b: &[u8]) -> serde_json::Result<Self> {
        serde_json::from_slice(b)
    }

    pub fn is_err(&self) -> bool {
        self.error
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn body(self) -> Value {
        self.body
    }

    pub fn err(&self) -> Option<ApiError> {
        if self.error {
            Some(ApiError::from_str(self.message.as_str()))
        } else { None }
    }
}

pub struct ApiError {
    message: String,
}

impl ApiError {
    pub fn from_str(msg: &str) -> ApiError {
        ApiError {
            message: msg.to_string()
        }
    }
}

impl Debug for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}