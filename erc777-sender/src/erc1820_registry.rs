use casper_contract::{contract_api::runtime};
use casper_types::{ContractHash, Key, runtime_args, RuntimeArgs};
use casper_types::bytesrepr::Bytes;
use constants::{
    SET_INTERFACE_OF_EXTERNAL_ENTRY_POINT, ACCOUNT_RUNTIME_ARG_NAME,
    I_HASH_RUNTIME_ARG_NAME, IMPLEMENTER_RUNTIME_ARG_NAME,
};

pub(crate) fn set_implementer(
    account: Key,
    i_hash: Bytes,
    implementer: Key,
    contract_hash: ContractHash
) {

    let registry_args = runtime_args! {
        ACCOUNT_RUNTIME_ARG_NAME => account,
        I_HASH_RUNTIME_ARG_NAME => i_hash,
        IMPLEMENTER_RUNTIME_ARG_NAME => implementer
    };
    runtime::call_contract::<()>(
        contract_hash,
        SET_INTERFACE_OF_EXTERNAL_ENTRY_POINT,
        registry_args,
    );

}