use alloc::{string::String, vec::Vec};
use alloc::string::ToString;
use core::iter::FromIterator;
use core::str::Bytes;

use casper_contract::{
    contract_api::storage,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_contract::contract_api::runtime;
use casper_types::{bytesrepr::{ToBytes, FromBytes}, URef, U256, Key, account::AccountHash};
use crate::{constants::OPERATORS_KEY_NAME, detail, Address};
use crate::Address::Account;

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
    let values: Vec<Address> = storage::dictionary_get(operators_uref, owner_key)
        .unwrap_or_revert()
        .unwrap_or_default();

    values
}

pub fn write_operators(operators_uref: URef, caller: &str, operator: Address) {

    let operators = read_operators(operators_uref, caller);


    storage::dictionary_put(operators_uref, caller, operators);
}

pub fn remove_operator(operators_uref: URef, operator: Address) {
    let caller: Address = detail::get_caller_address().unwrap_or_revert();
    let mut list: Vec<Address> = read_operators(operators_uref, to_str(caller).as_str());
    let pos = list.iter().position(|i| *i == operator).unwrap();
    list.remove(pos);
}

pub fn contains(operators_uref: URef, owner: Address, operator: Address) -> bool {
    if owner.eq(&operator) {
        return true;
    }

    let list: Vec<Address> = read_operators(operators_uref, to_str(owner).as_str());

    list.iter().any(|&i| i.eq(&operator))
}

pub fn check_if_exists(operators_uref: URef, owner:Address, operator: Address) -> bool {
    if owner == operator {
        return true;
    }

    let arrays_address: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_default().unwrap();

    arrays_address.contains(operator.as_account_hash().unwrap().clone().to_string().as_str())
}

pub fn get_rid_of(operators_uref: URef, owner: Address, operator: Address) {

    let arrays_address: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_default().unwrap();

    let mut aux = operator.as_account_hash().unwrap().clone().to_string();
    aux.push('|');

    let new_array= arrays_address.replace(aux.as_str(), "");

    storage::dictionary_put(operators_uref, to_str(owner).as_str(), new_array);
}

pub fn concat_in_string(operators_uref: URef, owner: Address, operator: Address) {

    let mut arrays_address: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_revert()
        .unwrap_or_default();

    arrays_address.push_str(operator.as_account_hash().unwrap().clone().to_string().as_str());
    arrays_address.push('|');
    storage::dictionary_put(operators_uref, to_str(owner).as_str(), arrays_address);
}

pub fn make_array(operators_uref: URef, owner: Address) -> Vec<Address> {
    let arrays_address: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_revert()
        .unwrap_or_default();

    let list = Vec::from_iter(
        arrays_address.split('|')
            .map(String::from)
            .filter(|value | ! value.is_empty())
    );

    let new_list = Vec::from_iter(
        list.iter()
            .map(
                |value
                | AccountHash::from_formatted_str(value).unwrap()
            )
    );

    Vec::from_iter(
        new_list.iter()
            .map(
                | &value
                | Account(value)
            )
    )
}

pub(crate) fn to_str(owner: Address) -> String {
    let key_bytes = owner.to_bytes().unwrap_or_revert();
    let hash = runtime::blake2b(&key_bytes);
    hex::encode(&hash)
}

pub(crate) fn to_bytes(address: Address) -> Vec<u8> {
    address.to_bytes().unwrap_or_revert()
}