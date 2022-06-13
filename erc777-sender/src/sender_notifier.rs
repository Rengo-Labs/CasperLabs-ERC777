use casper_types::{
    account::AccountHash,
    bytesrepr::Bytes,
    U256
};

pub(crate) fn tokens_to_send(
    operator: AccountHash,
    from: AccountHash,
    to: AccountHash,
    amount: U256,
    user_data: Bytes,
    operator_data: Bytes
) {

}