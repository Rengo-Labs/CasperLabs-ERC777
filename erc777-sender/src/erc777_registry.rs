//! Implementation of balances.
use alloc::string::String;
use core::convert::TryInto;

use casper_contract::{contract_api::{storage, runtime}, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{URef, U256, ApiError, ContractHash, Key, runtime_args, RuntimeArgs};

use crate::{
    constants::{
        ERC777_REGISTRY_KEY_NAME, OPERATOR_SEND_EXTERNAL_ENTRY_POINT, SENDER_RUNTIME_ARG_NAME,
        RECIPIENT_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, ACCOUNT_RUNTIME_ARG_NAME,
        DATA_RUNTIME_ARG_NAME, OPERATOR_DATA_RUNTIME_ARG_NAME, OPERATOR_BURN_EXTERNAL_ENTRY_POINT
    }
};

/// get the erc777 uref.
pub(crate) fn get_erc777_uref() -> URef {
    let key = runtime::get_key(ERC777_REGISTRY_KEY_NAME)
        .ok_or(ApiError::MissingKey)
        .unwrap_or_revert();
    key.try_into().unwrap_or_revert()
}

/// this works to save the erc777 when it deploys the contract.
pub(crate) fn save_erc777_contract(erc777_uref: URef, contract_hash: ContractHash) {
    storage::dictionary_put(erc777_uref, ERC777_REGISTRY_KEY_NAME, contract_hash);
}

/// transfer tokens calling an erc777 contract
pub(crate) fn transfer(erc777_uref: URef, from: Key, to: Key, amount: U256, user_data: String, operator_data: String) {
    let hash_contract = storage::dictionary_get(
        erc777_uref,
        ERC777_REGISTRY_KEY_NAME
    ).unwrap_or_default().unwrap_or_default();

    let registry_args = runtime_args! {
        SENDER_RUNTIME_ARG_NAME => from,
        RECIPIENT_RUNTIME_ARG_NAME => to,
        AMOUNT_RUNTIME_ARG_NAME => amount,
        DATA_RUNTIME_ARG_NAME => user_data,
        OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
    };
    runtime::call_contract::<()>(
        hash_contract,
        OPERATOR_SEND_EXTERNAL_ENTRY_POINT,
        registry_args,
    );
}

/// burn tokens calling an erc777 contract
pub(crate) fn burn(erc777_uref: URef, account: Key, amount: U256, user_data: String, operator_data: String) {
    let hash_contract = storage::dictionary_get(
        erc777_uref,
        ERC777_REGISTRY_KEY_NAME
    ).unwrap_or_default().unwrap_or_default();

    let registry_args = runtime_args! {
        ACCOUNT_RUNTIME_ARG_NAME => account,
        AMOUNT_RUNTIME_ARG_NAME => amount,
        DATA_RUNTIME_ARG_NAME => user_data,
        OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
    };
    runtime::call_contract::<()>(
        hash_contract,
        OPERATOR_BURN_EXTERNAL_ENTRY_POINT,
        registry_args,
    );
}