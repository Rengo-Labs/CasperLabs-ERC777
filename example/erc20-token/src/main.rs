#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc20::{
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME,
        NAME_RUNTIME_ARG_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME, SYMBOL_RUNTIME_ARG_NAME, TOTAL_SUPPLY_RUNTIME_ARG_NAME,
        DATA_RUNTIME_ARG_NAME, GRANULARITY_RUNTIME_ARG_NAME,
        OPERATOR_DATA_RUNTIME_ARG_NAME, OPERATOR_RUNTIME_ARG_NAME,
        SENDER_RUNTIME_ARG_NAME, TOKEN_HOLDER_RUNTIME_ARG_NAME
    },
    Address, ERC20,
};
use casper_types::{CLValue, U256};
use casper_types::bytesrepr::Bytes;
use casper_erc20::constants::OPERATORS_RUNTIME_ARG_NAME;

#[no_mangle]
pub extern "C" fn name() {
    let name = ERC20::default().name();
    runtime::ret(CLValue::from_t(name).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn symbol() {
    let symbol = ERC20::default().symbol();
    runtime::ret(CLValue::from_t(symbol).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn decimals() {
    let decimals = ERC20::default().decimals();
    runtime::ret(CLValue::from_t(decimals).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn total_supply() {
    let total_supply = ERC20::default().total_supply();
    runtime::ret(CLValue::from_t(total_supply).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn balance_of() {
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);
    let balance = ERC20::default().balance_of(address);
    runtime::ret(CLValue::from_t(balance).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default()
        .transfer(recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn approve() {
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);

    ERC20::default().approve(spender, amount).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn allowance() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let spender: Address = runtime::get_named_arg(SPENDER_RUNTIME_ARG_NAME);
    let val = ERC20::default().allowance(owner, spender);
    runtime::ret(CLValue::from_t(val).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn transfer_from() {
    let owner: Address = runtime::get_named_arg(OWNER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    ERC20::default()
        .transfer_from(owner, recipient, amount)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn granularity() {
    let granularity = ERC20::default().granularity();
    runtime::ret(CLValue::from_t(granularity).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn burn() {
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let data: Bytes = runtime::get_named_arg(DATA_RUNTIME_ARG_NAME);

    ERC20::default()
        .burn(amount, data)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn send() {
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let data: Bytes = runtime::get_named_arg(DATA_RUNTIME_ARG_NAME);

    ERC20::default()
        .send(recipient, amount, data)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn is_operator_for() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let token: Address = runtime::get_named_arg(TOKEN_HOLDER_RUNTIME_ARG_NAME);
    let is_operator = ERC20::default()
        .is_operator_for(operator, token)
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(is_operator).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn authorize_operator() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    ERC20::default()
        .authorize_operator(operator)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn revoke_operator() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    ERC20::default()
        .revoke_operator(operator)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn default_operators() {
    let operators = ERC20::default()
        .default_operators()
        .unwrap_or_revert();
    runtime::ret(CLValue::from_t(operators).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn operator_send() {
    let sender: Address = runtime::get_named_arg(SENDER_RUNTIME_ARG_NAME);
    let recipient: Address = runtime::get_named_arg(RECIPIENT_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let data: Bytes = runtime::get_named_arg(DATA_RUNTIME_ARG_NAME);
    let operator_data: Bytes = runtime::get_named_arg(OPERATOR_DATA_RUNTIME_ARG_NAME);

    ERC20::default()
        .operator_send(sender, recipient, amount, data, operator_data)
        .unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn operator_burn() {
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let amount: U256 = runtime::get_named_arg(AMOUNT_RUNTIME_ARG_NAME);
    let data: Bytes = runtime::get_named_arg(DATA_RUNTIME_ARG_NAME);
    let operator_data: Bytes = runtime::get_named_arg(OPERATOR_DATA_RUNTIME_ARG_NAME);

    ERC20::default()
        .operator_burn(operator, amount, data, operator_data)
        .unwrap_or_revert();
}

#[no_mangle]
fn call() {
    let name: String = runtime::get_named_arg(NAME_RUNTIME_ARG_NAME);
    let symbol: String = runtime::get_named_arg(SYMBOL_RUNTIME_ARG_NAME);
    let granularity = runtime::get_named_arg(GRANULARITY_RUNTIME_ARG_NAME);
    let total_supply = runtime::get_named_arg(TOTAL_SUPPLY_RUNTIME_ARG_NAME);
    let operators: Vec<Address> = runtime::get_named_arg(OPERATORS_RUNTIME_ARG_NAME);

    let _token = ERC20::install(
        name,
        symbol,
        operators,
        granularity,
        total_supply
    ).unwrap_or_revert();
}