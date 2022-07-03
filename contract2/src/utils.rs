use crate::*;

pub(crate) fn expect_lightweight<T>(
    option: Option<T>,
    message: &str
) -> T {
    option.unwrap_or_else(|| env::panic_str(message))
}