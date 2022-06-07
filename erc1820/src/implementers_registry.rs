use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use casper_types::bytesrepr::{Bytes, FromBytes, ToBytes};
use casper_types::{CLType, CLTyped, URef};
use casper_types::account::AccountHash;
use ::{Address, detail};
use casper_contract::{
    contract_api::{runtime, storage}
};
use address::Address::Account;
use constants::MANAGERS_REGISTRY_KEY_NAME;
use detail::get_immediate_caller_address;
use IMPLEMENTERS_REGISTRY_KEY_NAME;

#[inline]
pub(crate) fn implementers_registry() -> URef {
    detail::get_uref(IMPLEMENTERS_REGISTRY_KEY_NAME)
}

pub fn create_or_update_implementer(
    implementer_uref: URef,
    account: AccountHash,
    interface_hash: Bytes,
    implementer: AccountHash
) {
    let mut hash_string: String;

    if AccountHash::default().eq(&account) {
        let caller = get_immediate_caller_address().unwrap_or(Account(AccountHash::default()));
        hash_string = to_str(*caller.as_account_hash().unwrap())
    } else {
        hash_string = to_str(account);
    }

    let mut implementers: BTreeMap<Bytes, AccountHash> = storage::dictionary_get(
        implementer_uref,
        hash_string.as_str()
    ).unwrap_or_default().unwrap_or_default();

    implementers.insert(interface_hash, implementer);

    storage::dictionary_put(
        implementer_uref,
        hash_string.as_str(),
        implementers);
}

pub fn get_implementer(implementer_uref: URef, account: AccountHash, interface_hash: Bytes) -> AccountHash {
    let hash_string = to_str(account);
    let implements: BTreeMap<Bytes, AccountHash> = storage::dictionary_get(
        implementer_uref,
        hash_string.as_str()
    ).unwrap_or_default().unwrap_or_default();

    *implements.get(&interface_hash).unwrap()
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
