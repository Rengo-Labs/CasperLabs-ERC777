use casper_types::{Key, U256};
use casper_types::bytesrepr::Bytes;

pub(crate) fn tokens_received(
    operator: Key,
    from: Key,
    to: Key,
    amount: U256,
    data: Bytes,
    operator_data: Bytes
) {

}