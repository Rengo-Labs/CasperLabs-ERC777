#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{ContractHash, Key, U256, bytesrepr::Bytes};
use casper_erc777_sender::{constants::{
    AMOUNT_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME, USER_DATA_RUNTIME_ARG_NAME,
    OPERATOR_DATA_RUNTIME_ARG_NAME, OPERATOR_RUNTIME_ARG_NAME, TO_RUNTIME_ARG_NAME,
    ERC1820_CONTRACT_ARG_NAME, ACCOUNT_RUNTIME_ARG_NAME, ERC777_CONTRACT_ARG_NAME
}, ERC777Sender};

#[no_mangle]
pub extern "C" fn tokens_to_send() {
    let operator: Key = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let from: Key = runtime::get_named_arg(FROM_RUNTIME_ARG_NAME);
    let to: Key = runtime::get_named_arg(TO_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let data: Bytes = runtime::get_named_arg(USER_DATA_RUNTIME_ARG_NAME);
    let operator_data: Bytes = runtime::get_named_arg(OPERATOR_DATA_RUNTIME_ARG_NAME);

    ERC777Sender::default()
        .tokens_to_send(operator, from, to, amount, data, operator_data)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn transfer() {
    let from: Key = runtime::get_named_arg(FROM_RUNTIME_ARG_NAME);
    let to: Key = runtime::get_named_arg(TO_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let data: Bytes = runtime::get_named_arg(USER_DATA_RUNTIME_ARG_NAME);
    let operator_data: Bytes = runtime::get_named_arg(OPERATOR_DATA_RUNTIME_ARG_NAME);

    ERC777Sender::default()
        .transfer(from, to, amount, data, operator_data)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn burn() {
    let account: Key = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let data: Bytes = runtime::get_named_arg(USER_DATA_RUNTIME_ARG_NAME);
    let operator_data: Bytes = runtime::get_named_arg(OPERATOR_DATA_RUNTIME_ARG_NAME);

    ERC777Sender::default()
        .burn(account, amount, data, operator_data)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let erc1820_contract: ContractHash = runtime::get_named_arg(ERC1820_CONTRACT_ARG_NAME);
    let erc777_contract: ContractHash = runtime::get_named_arg(ERC777_CONTRACT_ARG_NAME);

    ERC777Sender::install(erc1820_contract, erc777_contract).unwrap_or_revert();
}
