use alloc::{string::String, vec::Vec};

use casper_contract::{
    contract_api::storage,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::ToBytes, URef, U256};

use crate::{constants::OPERATORS_KEY_NAME, detail, Address};

#[inline]
pub(crate) fn operators_uref() -> URef {
    detail::get_uref(OPERATORS_KEY_NAME)
}

pub fn find_operators(address: Address) -> Vec<Address> {
    read_operators(
        operators_uref(),
        to_str(address).as_str()
    )
}

pub fn read_operators(operators_uref: URef, owner_key: &str) -> Vec<Address> {
    let value: Vec<Address> = storage::dictionary_get(operators_uref, owner_key)
        .unwrap_or_revert()
        .unwrap_or_default();
    value
}

pub fn write_operators(operators_uref: URef, caller: &str, operator: Address) {
    //let owner_str_key = to_str(caller).as_str();

    let mut operators = read_operators(operators_uref, caller);
    operators.push(operator);
    storage::dictionary_put(operators_uref, caller, operators);
}

pub fn remove_operator(operators_uref: URef, operator: Address) {
    let caller: Address = detail::get_caller_address().unwrap_or_revert();
    let mut list: Vec<Address> = read_operators(operators_uref, to_str(caller).as_str());
    let pos = list.iter().position(|i| *i == operator).unwrap();
    list.remove(pos);
}

pub fn contains(operators_uref: URef, owner: Address, operator: Address) -> bool {
    let list: Vec<Address> = read_operators(operators_uref, to_str(owner).as_str());
    list.iter().any(|i| *i == operator)
}

pub(crate) fn to_str(owner: Address) -> String {
    let key_bytes = owner.to_bytes().unwrap_or_revert();
    hex::encode(&key_bytes)
}