mod web4;
mod utils;

use crate::web4::*;
use crate::utils::*;

// use near_sdk::AccountId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{near_bindgen};


const PRECOMPILED_BODY: &str = include_str!("/workspaces/Web4_example/design/precompile/precompile.html");
const PAGINATION_BODY: &str = include_str!("/workspaces/Web4_example/design/shared/pagination.html");

const ABOUT_BODY: &str = include_str!("/workspaces/Web4_example/design/about.html");
const INDEX_BODY: &str = include_str!("/workspaces/Web4_example/design/index.html");
const WRITE_BODY: &str = include_str!("/workspaces/Web4_example/design/write.html");
const READ_BODY : &str = include_str!("/workspaces/Web4_example/design/read.html");
const ACCOUNT_BODY: &str = include_str!("/workspaces/Web4_example/design/account.html");



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

      if splitted_path[1] == "article" {
        return Web4Response::html_response(
          common_page_template(READ_BODY)
        );
      }

      if splitted_path[1] == "account" {
        return Web4Response::html_response(
          common_page_template(ACCOUNT_BODY)
        );
      }

      // when moved here, we'll do the things here than on js. 

      Web4Response::html_response(
        common_page_template(INDEX_BODY).replace("%PAGINATION%", PAGINATION_BODY)
      )
    }
}