use alloc::string::{String};
use alloc::vec::Vec;
use casper_types::bytesrepr::{ToBytes};
use casper_types::{Key, URef};
use casper_types::account::AccountHash;
use ::{detail};
use casper_contract::{
    contract_api::{runtime, storage}
};
use constants::{IMPLEMENTERS_REGISTRY_KEY_NAME};

#[inline]
pub(crate) fn implementers_registry() -> URef {
    detail::get_uref(IMPLEMENTERS_REGISTRY_KEY_NAME)
}

pub fn create_or_update_implementer(
    implementer_uref: URef,
    account: Key,
    interface_hash: String,
    implementer: Key
) {
    let hash_string: String;

    hash_string = to_str(account, interface_hash);

    storage::dictionary_put(
        implementer_uref,
        hash_string.as_str(),
        implementer);
}

pub fn get_implementer(implementer_uref: URef, account: Key, interface_hash: String) -> Key {
    let hash_string = to_str(account, interface_hash);
    let implementer = storage::dictionary_get(
        implementer_uref,
        hash_string.as_str()
    ).unwrap().unwrap_or(Key::Account(AccountHash::default()));

    implementer
}

pub(crate) fn to_str(owner: Key, tag: String) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut owner.to_bytes().unwrap());
    preimage.append(&mut tag.into_bytes());

    let hash = runtime::blake2b(&preimage);
    hex::encode(&hash)
}
