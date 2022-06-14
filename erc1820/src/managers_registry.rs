use alloc::string::{String};
use casper_types::bytesrepr::{ToBytes};
use casper_types::{Key, URef};
use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use constants::MANAGERS_REGISTRY_KEY_NAME;
use detail;

#[inline]
pub(crate) fn managers_registry() -> URef {
    detail::get_uref(MANAGERS_REGISTRY_KEY_NAME)
}

pub fn set_manager(manager_uref: URef, account: Key, manager: Key) {
    let hash_account = to_str(account);

    storage::dictionary_put(
        manager_uref,
        hash_account.as_str(),
        manager);
}

pub fn get_manager(manager_uref: URef, account: Key) -> Key {
    let hash_string = to_str(account);
    let manager: Key = storage::dictionary_get(
        manager_uref,
        hash_string.as_str()
    ).unwrap_or_default().unwrap_or_revert();

    manager
}

pub(crate) fn to_str(owner: Key) -> String {
    let key_bytes = owner.to_bytes().unwrap();
    let hash = runtime::blake2b(&key_bytes);
    hex::encode(&hash)
}
