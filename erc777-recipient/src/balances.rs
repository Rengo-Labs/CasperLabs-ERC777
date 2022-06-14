//! Implementation of balances.

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{
    Key, runtime_args, RuntimeArgs, ContractHash
};
use crate::{
    constants::{
        ACCOUNT_RUNTIME_ARG_NAME, BALANCE_OF_ENTRY_POINT_NAME
    }
};

pub(crate) fn get_balance_of(account: Key, contract_hash: ContractHash) {
    let registry_args = runtime_args! {
        ACCOUNT_RUNTIME_ARG_NAME => account
    };
    runtime::call_contract::<()>(
        contract_hash,
        BALANCE_OF_ENTRY_POINT_NAME,
        registry_args,
    );
}