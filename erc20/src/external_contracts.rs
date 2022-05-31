use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_types::{ContractHash, Key, runtime_args, RuntimeArgs, U256};
use casper_types::bytesrepr::Bytes;
use crate::{Address, Error};
use crate::constants::{
    ADDRESS_RUNTIME_ARG_NAME, REGISTRY_CONTRACT_GET_INTERFACE_ENTRY_POINT,
    REGISTRY_CONTRACT_NAME, REGISTRY_CONTRACT_SET_INTERFACE_ENTRY_POINT,
    I_HASH_RUNTIME_ARG_NAME, IMPLEMENTER_RUNTIME_ARG_NAME,
    ERC777_CONTRACT_SENDER_INTERFACE_ENTRY_POINT, OPERATOR_RUNTIME_ARG_NAME,
    FROM_RUNTIME_ARG_NAME, TO_RUNTIME_ARG_NAME, ERC777_SENDER_CONTRACT_NAME,
    ERC777_CONTRACT_RECIPIENT_INTERFACE_ENTRY_POINT, ERC777_RECIPIENT_CONTRACT_NAME,
    USER_DATA_RUNTIME_ARG_NAME, OPERATOR_DATA_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME
};

pub(crate) fn initialization(account: Address, i_hash: Bytes) {
    if i_hash.is_empty() {
        return;
    }

    let token_contract: Key = runtime::get_key(REGISTRY_CONTRACT_NAME)
        .unwrap();

    let registry_args = runtime_args! {
        ADDRESS_RUNTIME_ARG_NAME => account,
        I_HASH_RUNTIME_ARG_NAME => i_hash
    };
    let registry_key: Key = runtime::call_contract(
        ContractHash::new(token_contract.into_hash().unwrap_or_default()),
        REGISTRY_CONTRACT_SET_INTERFACE_ENTRY_POINT,
        registry_args,
    );
}

pub(crate) fn get_interface(account: Address, i_hash: Bytes, implementer: Address) -> Result<Address, Error>{
    if i_hash.is_empty() {
        return Ok(account);
    }

    let token_contract: Key = runtime::get_key(REGISTRY_CONTRACT_NAME)
        .unwrap();

    let registry_args = runtime_args! {
        ADDRESS_RUNTIME_ARG_NAME => account,
        I_HASH_RUNTIME_ARG_NAME => i_hash,
        IMPLEMENTER_RUNTIME_ARG_NAME => implementer
    };
    let result: Address = runtime::call_contract(
        ContractHash::new(token_contract.into_hash().unwrap_or_default()),
        REGISTRY_CONTRACT_GET_INTERFACE_ENTRY_POINT,
        registry_args,
    );

    Ok(result)
}

pub(crate) fn tokens_to_send(
    operator: Address,
    from: Address,
    to: Address,
    amount: U256,
    user_data: Bytes,
    operator_data: Bytes
) {
    if user_data.is_empty() || operator_data.is_empty() {
        return;
    }

    let token_contract: Key = runtime::get_key(ERC777_SENDER_CONTRACT_NAME)
        .unwrap();

    let args = runtime_args! {
        OPERATOR_RUNTIME_ARG_NAME => operator,
        FROM_RUNTIME_ARG_NAME => from,
        TO_RUNTIME_ARG_NAME => to,
        AMOUNT_RUNTIME_ARG_NAME => amount,
        USER_DATA_RUNTIME_ARG_NAME => user_data,
        OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
    };
    let result: Address = runtime::call_contract(
        ContractHash::new(token_contract.into_hash().unwrap_or_default()),
        ERC777_CONTRACT_SENDER_INTERFACE_ENTRY_POINT,
        args,
    );
}

pub(crate) fn tokens_received(
    operator: Address,
    from: Address,
    to: Address,
    amount: U256,
    user_data: Bytes,
    operator_data: Bytes
) {
    if user_data.is_empty() || operator_data.is_empty() {
        return;
    }

    let token_contract: Key = runtime::get_key(ERC777_RECIPIENT_CONTRACT_NAME)
        .unwrap();

    let args = runtime_args! {
        OPERATOR_RUNTIME_ARG_NAME => operator,
        FROM_RUNTIME_ARG_NAME => from,
        TO_RUNTIME_ARG_NAME => to,
        AMOUNT_RUNTIME_ARG_NAME => amount,
        USER_DATA_RUNTIME_ARG_NAME => user_data,
        OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
    };
    let result: Address = runtime::call_contract(
        ContractHash::new(token_contract.into_hash().unwrap_or_default()),
        ERC777_CONTRACT_RECIPIENT_INTERFACE_ENTRY_POINT,
        args,
    );
}