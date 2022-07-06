use crate::*;

pub(crate) fn expect_lightweight<T>(
    option: Option<T>,
    message: &str
) -> T {
    option.unwrap_or_else(|| env::panic_str(message))
}


pub(crate) fn hash_account_id(account_id: &AccountId) -> CryptoHash {
    let mut hash = CryptoHash::default();

    hash.copy_from_slice(&env::sha256(account_id.as_bytes()));
    hash
}


pub(crate) fn estimate_total_storage(
  first_time: bool,
  cid: &str,
  account_id: &AccountId
) -> u64 {
    estimate_storage(cid) 
    + estimate_owner_storage(first_time, account_id)
    + 57u64  // total_tip storage cost; always constant. 
}


pub(crate) fn estimate_storage(cid: &str) -> u64 {
    cid.len() as u64
    + 29u64  // Magic number 
    + (size_of::<String>() as u64 * 2)  // size on-chain is 12; but stored on mem is 24.
}


pub(crate) fn estimate_owner_storage(
  first_time: bool,
  account_id: &AccountId
) -> u64 {
    if first_time {
      return 422u64 + (account_id.as_str().len() * 2) as u64
    } else {
      return 180u64
    }
}


pub(crate) fn calc_storage_cost(storage_used: u64) -> Balance {
    env::storage_byte_cost() * Balance::from(storage_used)
}


// This and below function can be refactored out in the future. 
pub(crate) fn yoctonear_to_near(amount: u128) -> f64 {
    let decimals = 5;

    let amount_str = amount.to_string();
    let amount_bytes = amount_str.as_bytes();

    let amount_len = amount_bytes.len();

    let mut num: String = "".to_owned();
    if amount_len <= 24 {  // below 1 NEAR, which has len = 25
      num.push_str("0.");

      let append_zeros = 24 - amount_len;
      for _ in 0..append_zeros {
        num.push_str("0")
      }

      for i in 0..decimals {
        num.push(amount_bytes[i] as char)
      }

    } else {  // above 1 NEAR
      let left = amount_len - 24;
      
      for i in 0..left {
        num.push(amount_bytes[i] as char)
      }
      
      num.push_str(".");

      for i in left..left+decimals {
        num.push(amount_bytes[i] as char)
      }
    }


    num.parse().unwrap()
}


pub(crate) fn tip_language_to_near(amount: u64) -> f64 {
    let decimals = 3;

    let amount_str = amount.to_string();
    let amount_bytes = amount_str.as_bytes();
    let amount_len = amount_bytes.len();

    let mut num: String = "".to_owned();

    if amount_len < decimals {
      // less than 1 NEAR
      num.push_str("0.");

      let append_zeros = decimals - amount_len;
      for _ in 0..append_zeros {
        num.push_str("0")
      }

      for i in 0..amount_len {
        num.push(amount_bytes[i] as char)
      }
    } else {
      // at least 1 NEAR

      let left = amount_len - 3;

      for i in 0..left {
        num.push(amount_bytes[i] as char)
      }
      num.push_str(".");
      for i in left..left+decimals {
        num.push(amount_bytes[i] as char)
      }
    }

    num.parse().unwrap()
}


pub(crate) fn yoctonear_to_tip_language(deposit: u128) -> u64 {
    (yoctonear_to_near(deposit) * 1000 as f64) as u64
}





impl Contract {
    /// Add token to owner set. 
    pub(crate) fn internal_add_token_to_owner(
      &mut self,
      account_id: &AccountId,
      token_id: &u64
    ) {
      let mut tokens_set = self.wlog_by_owner.get(account_id)
          .unwrap_or_else(|| {
            UnorderedSet::new(
              StorageKey::WlogByOwnerInner { token_type_hash: hash_account_id(&account_id) }
            )
      });

      tokens_set.insert(token_id);
      self.wlog_by_owner.insert(account_id, &tokens_set);
    }


    /// Remove token from owner set.
    pub(crate) fn internal_remove_token_from_owner(
      &mut self,
      account_id: &AccountId,
      token_id: &u64
    ) {
      let mut tokens_set = expect_lightweight(
        self.wlog_by_owner.get(account_id),
        "This token doesn't belong to this person."
      ); 

      tokens_set.remove(token_id);

      // Remove owner from collection if now empty. 
      if tokens_set.is_empty() {
        self.wlog_by_owner.remove(account_id);
      } else {
        self.wlog_by_owner.insert(account_id, &tokens_set);
      }
    }
}
