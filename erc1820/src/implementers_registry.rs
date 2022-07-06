use alloc::string::{String};
use alloc::vec::Vec;
use casper_types::bytesrepr::{Bytes, ToBytes};
use casper_types::{ApiError, Key, URef};
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
    interface_hash: Bytes,
    implementer: Key,
    manager: Key
) -> Result<(), ApiError> {
    let hash_string: String;

    if manager.ne(&account) {
        return Err(ApiError::User(1000));
    }

    if implementer.eq(&manager) || implementer.eq(&Key::Account(AccountHash::default())) {
        return Err(ApiError::User(1001))
    }

    hash_string = to_str(account, interface_hash);

    storage::dictionary_put(
        implementer_uref,
        hash_string.as_str(),
        implementer);
    Ok(())
}

pub fn get_implementer(implementer_uref: URef, account: Key, interface_hash: Bytes) -> Key {
    let hash_string = to_str(account, interface_hash);
    let implementer = storage::dictionary_get(
        implementer_uref,
        hash_string.as_str()
    ).unwrap().unwrap_or(Key::Account(AccountHash::default()));

    implementer
}

pub(crate) fn to_str(owner: Key, tag: Bytes) -> String {
    let mut preimage = Vec::new();
    preimage.append(&mut owner.to_bytes().unwrap());
    preimage.append(&mut tag.to_vec());

    let hash = runtime::blake2b(&preimage);
    hex::encode(&hash)
}
