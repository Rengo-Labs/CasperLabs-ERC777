use alloc::{string::String, vec::Vec};
use alloc::string::ToString;
use core::iter::FromIterator;

use casper_contract::{
    contract_api::storage,
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_contract::contract_api::runtime;
use casper_types::{bytesrepr::{ToBytes}, URef, account::AccountHash, Key};
use crate::{constants::OPERATORS_KEY_NAME, detail, Address, Error};
use crate::Address::Account;

#[inline]
pub(crate) fn operators_uref() -> URef {
    detail::get_uref(OPERATORS_KEY_NAME)
}

pub fn check_if_exists(operators_uref: URef, owner:Address, operator: Address) -> Result<bool, Error> {
    if owner.eq(&operator) {
        return Ok(true);
    }

    let data: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_default().unwrap_or_default();

    let addresses_string = decode(data);
    let result = addresses_string.contains(&operator.as_account_hash().unwrap().to_string());
    Ok(result)
}

pub fn get_rid_of(operators_uref: URef, owner: Address, operator: Address) {

    let data: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_default().unwrap();

    let addresses_string = decode(data);

    let mut aux = operator.as_account_hash().unwrap().clone().to_string();
    aux.push('|');
    let new_array= addresses_string.replace(aux.as_str(), "");

    storage::dictionary_put(operators_uref, to_str(owner).as_str(), encode(new_array));
}

pub fn concat_in_string(operators_uref: URef, owner: Address, operator: Address) {

    let data: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_revert()
        .unwrap_or_default();

    let mut addresses_string = "".to_string();
    if ! data.is_empty() {
        addresses_string = decode(data);
    }

    if addresses_string.contains((*operator.as_account_hash().unwrap()).to_string().as_str()) {
        return;
    }
    addresses_string.push_str((*operator.as_account_hash().unwrap()).to_string().as_str());
    addresses_string.push('|');

    storage::dictionary_put(operators_uref, to_str(owner).as_str(), encode(addresses_string));
}

pub fn make_array(operators_uref: URef, owner: Address) -> Vec<Address> {
    let data: String = storage::dictionary_get(
        operators_uref,
        to_str(owner).as_str()
    ).unwrap_or_revert()
        .unwrap_or_default();

    let addresses_string = decode(data);

    let list = Vec::from_iter(
        addresses_string.split('|')
            .map(String::from)
            .filter(|value | ! value.is_empty())
    );

    Vec::from_iter(
        list.iter()
            .map(
                | value
                | Account(AccountHash::from_formatted_str(value).unwrap())
            )
    )
}

pub(crate) fn to_str(owner: Address) -> String {
    let key_bytes = owner.to_bytes().unwrap_or_revert();
    let hash = runtime::blake2b(&key_bytes);
    hex::encode(&hash)
}

fn encode(data: String) -> String {
    base64::encode(data)
}

fn decode(data: String) -> String {
    String::from_utf8(base64::decode(data).unwrap()).unwrap_or_default()
}