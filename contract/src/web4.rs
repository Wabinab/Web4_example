use crate::*;
use near_sdk::json_types::Base64VecU8;
use std::collections::HashMap;


// Web4 Required: A must!

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,

    pub path: Option<String>,

    // #[serde(default)]
    pub params: Option<HashMap<String, String>>,

    // #[serde(default)]
    pub query: Option<HashMap<String, Vec<String>>>,

    pub preloads: Option<HashMap<String, Web4Response>>,
}


#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Response {
    #[serde(rename = "contentType")]
    content_type: Option<String>,
    
    status: Option<u32>,
    body: Option<Base64VecU8>,
    
    #[serde(rename = "bodyUrl")]
    body_url: Option<String>,

    #[serde(rename = "preloadUrls")]
    preload_urls: Option<Vec<String>>,
}


impl Web4Response {
    pub fn html_response(text: String) -> Self {
      Self {
        content_type: Some("text/html; charset=UTF-8".to_owned()),
        body: Some(text.into_bytes().into()),
        ..Default::default()
      }
    }


    pub fn plain_response(text: String) -> Self {
      Self {
        content_type: Some("text/plain; charset=UTF-8".to_owned()),
        body: Some(text.into_bytes().into()),
        ..Default::default()
      }
    }


    pub fn preload_urls(urls: Vec<String>) -> Self {
      Self {
        preload_urls: Some(urls),
        ..Default::default()
      }
    }


    pub fn body_url(url: String) -> Self {
      Self {
        body_url: Some(url),
        ..Default::default()
      }
    }

    pub fn status(status: u32) -> Self {
      Self {
        status: Some(status),
        ..Default::default()
      }
    }
}
