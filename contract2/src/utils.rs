use crate::*;

pub(crate) fn expect_lightweight<T>(
    option: Option<T>,
    message: &str
) -> T {
    option.unwrap_or_else(|| env::panic_str(message))
}


pub(crate) fn estimate_storage(cid: &str) -> u64 {
    cid.len() as u64
    + 29u64  // Magic number 
    + (size_of::<String>() as u64 * 2)  // size on-chain is 12; but stored on mem is 24.
}


pub(crate) fn calc_storage_cost(storage_used: u64) -> Balance {
    env::storage_byte_cost() * Balance::from(storage_used)
}


pub fn yoctonear_to_near(amount: u128) -> f64 {
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