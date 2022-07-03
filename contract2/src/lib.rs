use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
// use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, PanicOnDefault, BorshStorageKey};
use near_sdk::collections::{Vector};


mod utils;

pub(crate) use crate::utils::*;


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub list_of_wlog: Vector<String>,
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey {
    ListOfWlog 
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self { 
          list_of_wlog: Vector::new(StorageKey::ListOfWlog)
        }
    }


    pub fn register_log(
        &mut self,
        cid: String
    ) {
        // How to check the given cid is correct? I don't know. 
        self.list_of_wlog.push(&cid);
    }


    pub fn get_item(
        &self,
        item_no: u64
    ) -> String {
        expect_lightweight(
            self.list_of_wlog.get(item_no),
            "Cannot find this item."
        )
    }

    // get items paginated.

    // remove item if detected not matching. Can only be called by specific people.
}

