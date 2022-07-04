use crate::*;

pub(crate) fn expect_lightweight<T>(
    option: Option<T>,
    message: &str
) -> T {
    option.unwrap_or_else(|| env::panic_str(message))
}


// pub(crate) fn calc_storage_cost(storage_used: u64) -> Balance {
//     env::storage_byte_cost() * Balance::from(storage_used)
// }