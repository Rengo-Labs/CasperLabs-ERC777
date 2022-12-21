use casper_contract::{contract_api::{runtime, storage}};
use casper_types::{ContractHash, Key, runtime_args, RuntimeArgs, U256, URef};
use casper_types::bytesrepr::Bytes;
use crate::{Address, detail};
use crate::constants::{
    GET_INTERFACE_OF_EXTERNAL_ENTRY_POINT, REGISTRY_CONTRACT_NAME,
    I_HASH_RUNTIME_ARG_NAME, TOKENS_TO_SEND_OF_EXTERNAL_ENTRY_POINT,
    OPERATOR_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME, TO_RUNTIME_ARG_NAME, TOKENS_RECEIVED_OF_EXTERNAL_ENTRY_POINT,
    USER_DATA_RUNTIME_ARG_NAME, OPERATOR_DATA_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME,
    ACCOUNT_RUNTIME_ARG_NAME
};

#[inline]
pub(crate) fn get_registry_uref() -> URef {
    detail::get_uref(REGISTRY_CONTRACT_NAME)
}

pub(crate) fn set_registry(registry_uref: URef, contract_hash: ContractHash) {
    storage::dictionary_put(registry_uref, REGISTRY_CONTRACT_NAME, contract_hash);
}

pub(crate) fn get_interface(registry_uref: URef, account: Address, i_hash: Bytes) -> Key {

    //let hash_contract = ContractHash::from_formatted_str(HASH_ERC1820_REGISTRY).unwrap();
    let hash_contract = storage::dictionary_get(
        registry_uref,
        REGISTRY_CONTRACT_NAME
    ).unwrap_or_default().unwrap_or_default();

    let registry_args = runtime_args! {
        ACCOUNT_RUNTIME_ARG_NAME => account,
        I_HASH_RUNTIME_ARG_NAME => i_hash
    };

    return runtime::call_contract(
        hash_contract,
        GET_INTERFACE_OF_EXTERNAL_ENTRY_POINT,
        registry_args
    );
}

pub(crate) fn tokens_to_send(
    operator: Address,
    from: Address,
    to: Address,
    amount: U256,
    user_data: Bytes,
    operator_data: Bytes,
    implementer: Key
) {

    let args = runtime_args! {
        OPERATOR_RUNTIME_ARG_NAME => operator,
        FROM_RUNTIME_ARG_NAME => from,
        TO_RUNTIME_ARG_NAME => to,
        AMOUNT_RUNTIME_ARG_NAME => amount,
        USER_DATA_RUNTIME_ARG_NAME => user_data,
        OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
    };
    runtime::call_contract::<()>(
        ContractHash::new(implementer.into_hash().unwrap()),
        TOKENS_TO_SEND_OF_EXTERNAL_ENTRY_POINT,
        args,
    );
}

pub(crate) fn tokens_received(
    operator: Address,
    from: Address,
    to: Address,
    amount: U256,
    user_data: Bytes,
    operator_data: Bytes,
    implementer: Key
) {

    let args = runtime_args! {
        OPERATOR_RUNTIME_ARG_NAME => operator,
        FROM_RUNTIME_ARG_NAME => from,
        TO_RUNTIME_ARG_NAME => to,
        AMOUNT_RUNTIME_ARG_NAME => amount,
        USER_DATA_RUNTIME_ARG_NAME => user_data,
        OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
    };
    runtime::call_contract::<()>(
        ContractHash::new(implementer.into_hash().unwrap()),
        TOKENS_RECEIVED_OF_EXTERNAL_ENTRY_POINT,
        args,
    );
}