use std::mem::size_of;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
// use near_sdk::serde::{Deserialize};
use near_sdk::{
    env, near_bindgen, PanicOnDefault, BorshStorageKey, Balance, require, 
    Promise, AccountId, CryptoHash
};
use near_sdk::collections::{Vector, UnorderedMap, UnorderedSet};
// use ehttp::Response;

mod utils;

pub(crate) use crate::utils::*;


// #[near_bindgen]
// #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
// pub struct OldContract {
//     pub list_of_wlog: Vector<String>,
// }

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub list_of_wlog: Vector<String>,
    pub wlog_by_owner: UnorderedMap<AccountId, UnorderedSet<u64>>
}

#[derive(BorshDeserialize, BorshSerialize, BorshStorageKey)]
enum StorageKey {
    ListOfWlog,
    WlogByOwner,
    WlogByOwnerInner { token_type_hash: CryptoHash }
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self { 
          list_of_wlog: Vector::new(StorageKey::ListOfWlog),
          wlog_by_owner: UnorderedMap::new(StorageKey::WlogByOwner)
        }
    }


    


    #[payable]
    pub fn register_log(
        &mut self,
        cid: String
    ) {
        let initial_storage_usage = env::storage_usage();

        let first_time = match self.wlog_by_owner.get(&env::predecessor_account_id()) {
            Some(_) => false,
            None => true
        };

        let est_total_storage = estimate_total_storage(
            first_time, 
            &cid, 
            &env::predecessor_account_id()
        );

        let estimate_cost = calc_storage_cost(est_total_storage);

        require!(
            env::attached_deposit() >= estimate_cost,
            format!(
                "Please attach {}N for storage.",
                yoctonear_to_near(estimate_cost)
            )
        );

        // How to check the given cid is correct? I don't know. 
        self.list_of_wlog.push(&cid);

        // Add token to owner
        self.internal_add_token_to_owner(
            &env::predecessor_account_id(), 
            &(self.list_of_wlog.len() - 1)  // index from 0 so -1. 
        );

        let final_storage_usage = env::storage_usage() - initial_storage_usage;

        env::log_str(format!(
            "Estimated Storage: {}\nActual Used: {}",
            est_total_storage,
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
        let first_time = match self.wlog_by_owner.get(&env::predecessor_account_id()) {
            Some(_) => false,
            None => true
        };
        
        U128(calc_storage_cost(estimate_total_storage(
            first_time, 
            &cid, 
            &env::predecessor_account_id()
        )))
    }

    
    
    pub fn get_item(
        &self,
        item_no: String
    ) -> String {
        expect_lightweight(
            self.list_of_wlog.get(item_no.parse().unwrap()),
            "Cannot find this item."
        )
    }


    // get items paginated.
    pub fn get_items_by_newest(
        &self,
        from_index: Option<String>,
        limit: Option<u64>,
    ) -> Vec<String> {
        let newest_id = self.list_of_wlog.len() - 1;
        let from_index = from_index.unwrap_or("0".to_owned());
        let start: u64 = from_index.as_str().parse().unwrap();
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
    

    /// Get items by owner id (owner). 
    /// Sort by newest. 
    pub fn get_items_by_owner(
        &self,
        owner_id: AccountId,
        from_index: Option<String>,
        limit: Option<u64>
    ) -> Vec<String> {
        let tokens_set = self.wlog_by_owner.get(&owner_id);

        let tokens: UnorderedSet<u64> = if let Some(value) = tokens_set {
            value
        } else {
            return vec![];
        };

        let ids = tokens.as_vector();

        let newest_id = ids.len() - 1;
        let from_index = from_index.unwrap_or("0".to_owned());
        let start: u64 = from_index.as_str().parse().unwrap();
        let limit = limit.unwrap_or(10);

        let mut result: Vec<String> = vec![];

        for i in 0..limit {
            if newest_id >= (start + i) {
                result.push(self.get_item(
                    ids.get(newest_id - start - i).unwrap().to_string()
                ));
            }
        }

        result
    }


    // #[private]
    // #[init(ignore_state)]
    // pub fn migrate() -> Self {
    //     let old_state: OldContract = expect_lightweight(
    //         env::state_read(), 
    //         "Migration failed."
    //     );

    //     Self {
    //         list_of_wlog: old_state.list_of_wlog,
    //         wlog_by_owner: UnorderedMap::new(StorageKey::WlogByOwner)
    //     }
    // }
}

