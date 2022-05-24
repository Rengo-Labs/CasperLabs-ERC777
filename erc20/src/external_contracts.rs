use alloc::string::String;
use casper_contract::contract_api::runtime;
use casper_types::{ContractHash, Key, runtime_args, RuntimeArgs, U256};
use casper_types::bytesrepr::Bytes;
use crate::{Address, Error};
use crate::constants::{
    ADDRESS_RUNTIME_ARG_NAME, REGISTRY_CONTRACT_GET_INTERFACE_ENTRY_POINT,
    REGISTRY_CONTRACT_NAME, REGISTRY_CONTRACT_SET_INTERFACE_ENTRY_POINT,
    I_HASH_RUNTIME_ARG_NAME, IMPLEMENTER_RUNTIME_ARG_NAME
};

pub(crate) fn initialization(account: Address, i_hash: Bytes) {
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

}

pub(crate) fn tokens_received(
    operator: Address,
    from: Address,
    to: Address,
    amount: U256,
    user_data: Bytes,
    operator_data: Bytes
) {

}