mod web4;
mod utils;

use crate::web4::*;
use crate::utils::*;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen};


const PRECOMPILED_BODY: &str = include_str!("/workspaces/Web4_example/design/precompile/precompile.html");

const ABOUT_BODY: &str = include_str!("/workspaces/Web4_example/design/about.html");
const INDEX_BODY: &str = include_str!("/workspaces/Web4_example/design/index.html");
const WRITE_BODY: &str = include_str!("/workspaces/Web4_example/design/write.html");



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

      if path == "/write" {
        return Web4Response::html_response(
          common_page_template(WRITE_BODY)
        );
      }

      Web4Response::html_response(
        common_page_template(INDEX_BODY)
      )
    }
}