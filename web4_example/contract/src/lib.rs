mod web4;
mod utils;

use crate::web4::*;
use crate::utils::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen};


const HEAD_BODY: &str = include_str!("./design/shared/head.html");
const CSS_BODY: &str = include_str!("./design/shared/css.html");
const IMPORTMAP_BODY: &str = include_str!("./design/shared/importmap.html");
const JS_BODY: &str = include_str!("./design/shared/js.html");
const NAVBAR_BODY: &str = include_str!("./design/shared/navbar.html");
const FOOTER_BODY: &str = include_str!("./design/shared/footer.html");

const LOGIN_CONTROLLER_BODY: &str = include_str!("./design/controller/login_controller.html");

const ABOUT_BODY: &str = include_str!("./design/about.html");
const INDEX_BODY: &str = include_str!("./design/index.html");



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct Contract {
}


#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
      let path = request.path.expect("Path expected.");
      let splitted_path = path.split("/").collect::<Vec<&str>>();

      if path == "/robots.txt" {
        return Web4Response::plain_response("User-agent: *\nDisallow:".to_owned());
      }

      if path == "/about" {
        return Web4Response::html_response(
          common_page_template(ABOUT_BODY)
        );
      }

      Web4Response::html_response(
        common_page_template(INDEX_BODY)
      )
    }
}