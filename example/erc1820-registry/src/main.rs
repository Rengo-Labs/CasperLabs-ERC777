#![no_std]
#![no_main]

#[cfg(not(target_arch = "wasm32"))]
compile_error!("target arch should be wasm32: compile with '--target wasm32-unknown-unknown'");

extern crate alloc;

use alloc::string::String;

use casper_contract::{contract_api::runtime, unwrap_or_revert::UnwrapOrRevert};
use casper_erc1820::{
    constants::{
        ACCOUNT_RUNTIME_ARG_NAME, NEW_MANAGER_RUNTIME_ARG_NAME,
        I_HASH_RUNTIME_ARG_NAME, I_NAME_RUNTIME_ARG_NAME,
        IMPLEMENTER_RUNTIME_ARG_NAME, I_ID_RUNTIME_ARG_NAME
    },
    Address, ERC1820,
};
use casper_types::{CLValue};
use casper_types::bytesrepr::Bytes;


#[no_mangle]
pub extern "C" fn set_interface_implementer() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let interface_hash: String = runtime::get_named_arg(I_HASH_RUNTIME_ARG_NAME);
    let implementer: Address = runtime::get_named_arg(IMPLEMENTER_RUNTIME_ARG_NAME);

    ERC1820::default().set_interface_implementer(
        account,
        interface_hash,
        implementer
    ).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn get_interface_implementer() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let interface_hash: String = runtime::get_named_arg(I_HASH_RUNTIME_ARG_NAME);

    let implementer = ERC1820::default().get_interface_implementer(
        account,
        interface_hash
    ).unwrap_or_revert();
    runtime::ret(CLValue::from_t(implementer).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn set_manager() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    let new_manager: Address = runtime::get_named_arg(NEW_MANAGER_RUNTIME_ARG_NAME);

    ERC1820::default()
        .set_manager(
            account,
            new_manager
        ).unwrap_or_revert();
}

#[no_mangle]
pub extern "C" fn get_manager() {
    let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);

    let manager = ERC1820::default()
        .get_manager(account)
        .unwrap_or_revert();

    runtime::ret(CLValue::from_t(manager).unwrap_or_revert());
}

#[no_mangle]
pub extern "C" fn interface_hash() {
    //let account: Address = runtime::get_named_arg(ACCOUNT_RUNTIME_ARG_NAME);
    //let interface_name: String = runtime::get_named_arg(I_NAME_RUNTIME_ARG_NAME);

}

#[no_mangle]
fn call() {
    ERC1820::install().unwrap_or_revert();
}
