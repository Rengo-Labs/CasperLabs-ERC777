#![no_std]
#![no_main]

extern crate alloc;

use alloc::{
    string::{String, ToString},
    vec,
};

use casper_contract::{
    self,
    contract_api::{runtime, storage},
};
use casper_erc20::{
    constants::{
        AMOUNT_RUNTIME_ARG_NAME, APPROVE_ENTRY_POINT_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        TRANSFER_ENTRY_POINT_NAME, TRANSFER_FROM_ENTRY_POINT_NAME, BALANCE_OF_ENTRY_POINT_NAME,
        DEFAULT_OPERATORS_ENTRY_POINT_NAME, IS_OPERATOR_FOR_ENTRY_POINT_NAME,
        AUTHORIZE_OPERATOR_ENTRY_POINT_NAME
    },
    Address,
};
use casper_types::{
    bytesrepr::ToBytes, runtime_args, CLTyped, ContractHash, EntryPoint, EntryPointAccess,
    EntryPointType, EntryPoints, Key, Parameter, RuntimeArgs, U256,
};
use casper_erc20::constants::{OPERATOR_RUNTIME_ARG_NAME, REVOKE_OPERATOR_ENTRY_POINT_NAME, TOKEN_HOLDER_RUNTIME_ARG_NAME};

const CHECK_TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "check_total_supply";
const CHECK_BALANCE_OF_ENTRY_POINT_NAME: &str = "check_balance_of";
const TRANSFER_AS_STORED_CONTRACT_ENTRY_POINT_NAME: &str = "transfer_as_stored_contract";
const APPROVE_AS_STORED_CONTRACT_ENTRY_POINT_NAME: &str = "approve_as_stored_contract";
const TRANSFER_FROM_AS_STORED_CONTRACT_ENTRY_POINT_NAME: &str = "transfer_from_as_stored_contract";
const CHECK_ALLOWANCE_OF_ENTRY_POINT_NAME: &str = "check_allowance_of";
const TOKEN_CONTRACT_RUNTIME_ARG_NAME: &str = "token_contract";
const ADDRESS_RUNTIME_ARG_NAME: &str = "address";
const OWNER_RUNTIME_ARG_NAME: &str = "owner";
const SPENDER_RUNTIME_ARG_NAME: &str = "spender";
const RESULT_KEY: &str = "result";
const ERC777_TEST_CALL_KEY: &str = "erc777_test_call";

fn store_result<T: CLTyped + ToBytes>(result: T) {
    match runtime::get_key(RESULT_KEY) {
        Some(Key::URef(uref)) => storage::write(uref, result),
        Some(_) => unreachable!(),
        None => {
            let new_uref = storage::new_uref(result);
            runtime::put_key(RESULT_KEY, new_uref.into());
        }
    }
}

#[no_mangle]
extern "C" fn check_total_supply() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let total_supply: U256 = runtime::call_contract(
        token_contract,
        casper_erc20::constants::TOTAL_SUPPLY_ENTRY_POINT_NAME,
        RuntimeArgs::default(),
    );
    store_result(total_supply);
}

#[no_mangle]
extern "C" fn check_balance_of() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let address: Address = runtime::get_named_arg(ADDRESS_RUNTIME_ARG_NAME);

    let balance_args = runtime_args! {
        casper_erc20::constants::ADDRESS_RUNTIME_ARG_NAME => address,
    };
    let result: U256 = runtime::call_contract(
        token_contract,
        BALANCE_OF_ENTRY_POINT_NAME,
        balance_args,
    );

    store_result(result);
}

#[no_mangle]
extern "C" fn check_operators_of() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let token_holder: Address = runtime::get_named_arg(TOKEN_HOLDER_RUNTIME_ARG_NAME);

    let operators_args = runtime_args! {
        casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator,
    };
    runtime::call_contract(
        token_contract,
        AUTHORIZE_OPERATOR_ENTRY_POINT_NAME,
        operators_args,
    );

    let query_operator_args = runtime_args! {
        casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator,
        casper_erc20::constants::TOKEN_HOLDER_RUNTIME_ARG_NAME => token_holder
    };
    let result: bool = runtime::call_contract(
        token_contract,
        IS_OPERATOR_FOR_ENTRY_POINT_NAME,
        query_operator_args,
    );

    store_result(result);
}

#[no_mangle]
extern "C" fn revoke_operator_of() {
    let token_contract: ContractHash = runtime::get_named_arg(TOKEN_CONTRACT_RUNTIME_ARG_NAME);
    let operator: Address = runtime::get_named_arg(OPERATOR_RUNTIME_ARG_NAME);
    let token_holder: Address = runtime::get_named_arg(TOKEN_HOLDER_RUNTIME_ARG_NAME);

    let operators_args = runtime_args! {
        casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator,
    };
    runtime::call_contract(
        token_contract,
        AUTHORIZE_OPERATOR_ENTRY_POINT_NAME,
        operators_args,
    );

    let query_operator_args = runtime_args! {
        casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator,
        casper_erc20::constants::TOKEN_HOLDER_RUNTIME_ARG_NAME => token_holder
    };
    let result: bool = runtime::call_contract(
        token_contract,
        IS_OPERATOR_FOR_ENTRY_POINT_NAME,
        query_operator_args,
    );

    store_result(result);

    let query_operator_args = runtime_args! {
        casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator,
    };
    runtime::call_contract(
        token_contract,
        REVOKE_OPERATOR_ENTRY_POINT_NAME,
        query_operator_args,
    );

    let query_operator_args = runtime_args! {
        casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator,
        casper_erc20::constants::TOKEN_HOLDER_RUNTIME_ARG_NAME => token_holder
    };
    let result2: bool = runtime::call_contract(
        token_contract,
        IS_OPERATOR_FOR_ENTRY_POINT_NAME,
        query_operator_args,
    );

    store_result(result2);
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();
    let check_total_supply_entrypoint = EntryPoint::new(
        String::from(CHECK_TOTAL_SUPPLY_ENTRY_POINT_NAME),
        vec![Parameter::new(
            TOKEN_CONTRACT_RUNTIME_ARG_NAME,
            ContractHash::cl_type(),
        )],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_balance_of_entrypoint = EntryPoint::new(
        String::from(CHECK_BALANCE_OF_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(ADDRESS_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );
    let check_operators_of_entrypoint = EntryPoint::new(
        String::from(CHECK_ALLOWANCE_OF_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    let revoke_operator_of_entrypoint = EntryPoint::new(
        String::from(TRANSFER_AS_STORED_CONTRACT_ENTRY_POINT_NAME),
        vec![
            Parameter::new(TOKEN_CONTRACT_RUNTIME_ARG_NAME, ContractHash::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        <()>::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    );

    entry_points.add_entry_point(check_total_supply_entrypoint);
    entry_points.add_entry_point(check_balance_of_entrypoint);
    entry_points.add_entry_point(check_operators_of_entrypoint);
    entry_points.add_entry_point(revoke_operator_of_entrypoint);

    let (_contract_hash, _version) = storage::new_contract(
        entry_points,
        None,
        Some(ERC777_TEST_CALL_KEY.to_string()),
        None,
    );
}
