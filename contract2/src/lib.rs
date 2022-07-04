use std::mem::size_of;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
// use near_sdk::serde::{Deserialize};
use near_sdk::{env, near_bindgen, PanicOnDefault, BorshStorageKey, Balance, require, Promise};
use near_sdk::collections::{Vector};
// use ehttp::Response;

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


    #[payable]
    pub fn register_log(
        &mut self,
        cid: String
    ) {
        let initial_storage_usage = env::storage_usage();

        let estimate_cost = calc_storage_cost(estimate_storage(&cid));

        require!(
            env::attached_deposit() >= estimate_cost,
            format!(
                "Please attach {}N for storage.",
                yoctonear_to_near(estimate_cost)
            )
        );

        // How to check the given cid is correct? I don't know. 
        self.list_of_wlog.push(&cid);

        let final_storage_usage = env::storage_usage() - initial_storage_usage;

        env::log_str(format!(
            "Estimated Storage: {}\nActual Used: {}",
            estimate_storage(&cid),
            final_storage_usage
        ).as_str());

        // If final usage is less than what we estimate, but we asked user to attach more than
        // that;
        if final_storage_usage < estimate_storage(&cid) {
            let refund = estimate_cost - calc_storage_cost(final_storage_usage);
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }

        if env::attached_deposit() > estimate_cost {
            let refund = env::attached_deposit() - estimate_cost;
            Promise::new(env::predecessor_account_id()).transfer(refund);
        }
    }


    pub fn est_storage(&self, cid: String) -> U128 {
        U128(calc_storage_cost(estimate_storage(&cid)))
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
    pub fn get_items_by_newest(
        &self,
        from_index: Option<u64>,
        limit: Option<u64>,
    ) -> Vec<String> {
        let newest_id = self.list_of_wlog.len() - 1;
        let start = from_index.unwrap_or(0u64);
        let limit = limit.unwrap_or(10);

        let mut result: Vec<String> = vec![];

        for i in 0..limit {
            if newest_id >= (start + i) {
                result.push(self.list_of_wlog.get(newest_id - start - i).unwrap());
            }
        }

        result
    }

    // remove item if detected not matching. Can only be called by specific people.
    
    // Get items by owner id in the future (or if we have time).
}

