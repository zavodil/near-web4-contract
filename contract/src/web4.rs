use crate::*;

use std::collections::HashMap;
use near_sdk::json_types::Base64VecU8;

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    pub(crate) account_id: Option<AccountId>,
    pub(crate) path: String,
    pub(crate) params: Option<HashMap<String, String>>,
    pub(crate) query: Option<HashMap<String, Vec<String>>>,
    pub(crate) preloads: Option<HashMap<String, Web4Response>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum Web4Response {
    Body {
        #[serde(rename = "contentType")]
        content_type: String,
        body: Base64VecU8,
    },
    BodyUrl {
        #[serde(rename = "bodyUrl")]
        body_url: String,
    },
    PreloadUrls {
        #[serde(rename = "preloadUrls")]
        preload_urls: Vec<String>,
    },
    Status {
        status: u32,
    }
}

impl Web4Response {
    pub fn html_response(html: String) -> Self {
        Self::Body {
            content_type: String::from("text/html; charset=UTF-8"),
            body: html.as_bytes().to_owned().into(),
        }
    }

    pub fn plain_response(text: String) -> Self {
        Self::Body {
            content_type: String::from("text/plain; charset=UTF-8"),
            body: text.as_bytes().to_owned().into()
        }
    }

    pub fn svg_response(text: String) -> Self {
        Self::Body {
            content_type: String::from("image/svg+xml"),
            body: text.as_bytes().to_owned().into()
        }
    }

    pub fn png_response(bytes: Vec<u8>) -> Self {
        Self::Body {
            content_type: String::from("image/png"),
            body: bytes.into()
        }
    }

    pub fn preload_urls(urls: Vec<String>) -> Self {
        Self::PreloadUrls {
            preload_urls: urls
        }
    }

    pub fn body_url(url: String) -> Self {
        Self::BodyUrl {
            body_url: url
        }
    }

    pub fn status(status: u32) -> Self {
        Self::Status {
            status
        }
    }
}