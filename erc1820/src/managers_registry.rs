use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use casper_types::bytesrepr::{Bytes, FromBytes, ToBytes};
use casper_types::{CLType, CLTyped, URef};
use casper_types::account::AccountHash;
use ::{Address, detail};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use constants::MANAGERS_REGISTRY_KEY_NAME;
use detail::get_immediate_caller_address;
use IMPLEMENTERS_REGISTRY_KEY_NAME;

#[inline]
pub(crate) fn managers_registry() -> URef {
    detail::get_uref(MANAGERS_REGISTRY_KEY_NAME)
}

pub fn set_manager(manager_uref: URef, account: AccountHash, manager: AccountHash) {
    let mut hash_account = to_str(account);

    storage::dictionary_put(
        manager_uref,
        hash_account.as_str(),
        manager);
}

pub fn get_manager(manager_uref: URef, account: AccountHash) -> AccountHash {
    let hash_string = to_str(account);
    let manager: AccountHash = storage::dictionary_get(
        manager_uref,
        hash_string.as_str()
    ).unwrap_or_default().unwrap_or_default();

    manager
}

pub(crate) fn to_str(owner: AccountHash) -> String {
    let key_bytes = owner.to_bytes().unwrap();
    let hash = runtime::blake2b(&key_bytes);
    hex::encode(&hash)
}

fn encode(data: String) -> String {
    base64::encode(data)
}

fn decode(data: String) -> String {
    String::from_vec(base64::decode(data).unwrap()).unwrap_or_default().0
}
