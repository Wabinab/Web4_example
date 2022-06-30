mod web4;

use crate::web4::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen};


const INDEX_BODY: &str = include_str!("./design/index.html");
const CSS_BODY: &str = include_str!("./design/css.html");
const IMPORTMAP_BODY: &str = include_str!("./design/importmap.html");
const JS_BODY: &str = include_str!("./design/js.html");
const BODY_JS_BODY: &str = include_str!("./design/body_js.html");


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
}


#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
      let path = request.path.expect("Path expected.");

      if path == "/robots.txt" {
        return Web4Response::plain_response("User-agent: *\nDisallow:".to_owned());
      }

      Web4Response::html_response(
        INDEX_BODY
            .replace("%STYLE%", CSS_BODY)
            .replace("%IMPORTMAP%", IMPORTMAP_BODY)
            .replace("%SCRIPT%", JS_BODY)
            .replace("%BODY_JS%", BODY_JS_BODY)
            .to_owned()
      )
    }
}