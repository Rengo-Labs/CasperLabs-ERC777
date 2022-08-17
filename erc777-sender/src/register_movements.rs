use alloc::collections::BTreeMap;
use alloc::string::String;
use core::convert::TryInto;
use casper_contract::contract_api::{runtime, storage};
use casper_contract::unwrap_or_revert::UnwrapOrRevert;
use casper_types::{ApiError, Key, U256, URef};
use casper_types::bytesrepr::{Bytes, ToBytes};
use ::{MOVEMENTS_REGISTRY_KEY_NAME};


#[inline]
pub(crate) fn get_registry_uref() -> URef {
    let key = runtime::get_key(MOVEMENTS_REGISTRY_KEY_NAME)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}

pub(crate) fn tokens_received(
    registry_uref: URef,
    operator: Key,
    from: Key,
    to: Key,
    _amount: U256,
    data: Bytes,
    operator_data: Bytes
) {

    if operator.eq(&from) {
        //if this is the same owner, it doesn't make sense to save data
        return;
    }

    let operator_key= make_dictionary_item_key(operator);
    let mut map = BTreeMap::new();
    map.insert(to, data);
    map.insert(operator, operator_data);

    storage::dictionary_put(registry_uref, operator_key.as_str(), map);
}

#[inline]
fn make_dictionary_item_key(address: Key) -> String {
    let preimage = address.to_bytes().unwrap_or_revert();
    base64::encode(&preimage)
}