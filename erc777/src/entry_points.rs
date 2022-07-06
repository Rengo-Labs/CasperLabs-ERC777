//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};
use alloc::boxed::Box;

use casper_types::{CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, U256};
use casper_types::bytesrepr::Bytes;

use crate::{
    address::Address,
    constants::{
        ADDRESS_RUNTIME_ARG_NAME, ALLOWANCE_ENTRY_POINT_NAME, AMOUNT_RUNTIME_ARG_NAME,
        APPROVE_ENTRY_POINT_NAME, BALANCE_OF_ENTRY_POINT_NAME, DECIMALS_ENTRY_POINT_NAME,
        NAME_ENTRY_POINT_NAME, OWNER_RUNTIME_ARG_NAME, RECIPIENT_RUNTIME_ARG_NAME,
        SPENDER_RUNTIME_ARG_NAME, SYMBOL_ENTRY_POINT_NAME, TOTAL_SUPPLY_ENTRY_POINT_NAME,
        TRANSFER_ENTRY_POINT_NAME, TRANSFER_FROM_ENTRY_POINT_NAME, ACCOUNT_RUNTIME_ARG_NAME,
        AUTHORIZE_OPERATOR_ENTRY_POINT_NAME, BURN_ENTRY_POINT_NAME, DATA_RUNTIME_ARG_NAME,
        DEFAULT_OPERATORS_ENTRY_POINT_NAME, GRANULARITY_ENTRY_POINT_NAME,
        IS_OPERATOR_FOR_ENTRY_POINT_NAME, OPERATOR_BURN_ENTRY_POINT_NAME,
        OPERATOR_DATA_RUNTIME_ARG_NAME, OPERATOR_RUNTIME_ARG_NAME, OPERATOR_SEND_ENTRY_POINT_NAME,
        REVOKE_OPERATOR_ENTRY_POINT_NAME, SEND_ENTRY_POINT_NAME,
        SENDER_RUNTIME_ARG_NAME, TOKEN_HOLDER_RUNTIME_ARG_NAME
    },
};

/// Returns the `name` entry point.
pub fn name() -> EntryPoint {
    EntryPoint::new(
        String::from(NAME_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `symbol` entry point.
pub fn symbol() -> EntryPoint {
    EntryPoint::new(
        String::from(SYMBOL_ENTRY_POINT_NAME),
        Vec::new(),
        String::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}
/// Returns the `transfer_from` entry point.
pub fn transfer_from() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_FROM_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `allowance` entry point.
pub fn allowance() -> EntryPoint {
    EntryPoint::new(
        String::from(ALLOWANCE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OWNER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
        ],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `approve` entry point.
pub fn approve() -> EntryPoint {
    EntryPoint::new(
        String::from(APPROVE_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SPENDER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `transfer` entry point.
pub fn transfer() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_ENTRY_POINT_NAME),
        vec![Parameter::new(ADDRESS_RUNTIME_ARG_NAME, Address::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `total_supply` entry point.
pub fn total_supply() -> EntryPoint {
    EntryPoint::new(
        String::from(TOTAL_SUPPLY_ENTRY_POINT_NAME),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `decimals` entry point.
pub fn decimals() -> EntryPoint {
    EntryPoint::new(
        String::from(DECIMALS_ENTRY_POINT_NAME),
        Vec::new(),
        u8::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `granularity` entry point.
pub fn granularity() -> EntryPoint {
    EntryPoint::new(
        String::from(GRANULARITY_ENTRY_POINT_NAME),
        Vec::new(),
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `send` entry point.
pub fn send() -> EntryPoint {
    EntryPoint::new(
        String::from(SEND_ENTRY_POINT_NAME),
        vec![
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `burn` entry point.
pub fn burn() -> EntryPoint {
    EntryPoint::new(
        String::from(BURN_ENTRY_POINT_NAME),
        vec![
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `is_operator_for` entry point.
pub fn is_operator_for() -> EntryPoint {
    EntryPoint::new(
        String::from(IS_OPERATOR_FOR_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TOKEN_HOLDER_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        CLType::Bool,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `authorize_operator` entry point.
pub fn authorize_operator() -> EntryPoint {
    EntryPoint::new(
        String::from(AUTHORIZE_OPERATOR_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `revoke_operator` entry point.
pub fn revoke_operator() -> EntryPoint {
    EntryPoint::new(
        String::from(REVOKE_OPERATOR_ENTRY_POINT_NAME),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `default_operator` entry point.
pub fn default_operators() -> EntryPoint {
    EntryPoint::new(
        String::from(DEFAULT_OPERATORS_ENTRY_POINT_NAME),
        Vec::new(),
        CLType::List(Box::new(Address::cl_type())),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `operator_send` entry point.
pub fn operator_send() -> EntryPoint {
    EntryPoint::new(
        String::from(OPERATOR_SEND_ENTRY_POINT_NAME),
        vec![
            Parameter::new(SENDER_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(RECIPIENT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(DATA_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `operator_burn` entry point.
pub fn operator_burn() -> EntryPoint {
    EntryPoint::new(
        String::from(OPERATOR_BURN_ENTRY_POINT_NAME),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(DATA_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(name());
    entry_points.add_entry_point(symbol());
    entry_points.add_entry_point(decimals());
    entry_points.add_entry_point(total_supply());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(transfer());
    entry_points.add_entry_point(approve());
    entry_points.add_entry_point(allowance());
    entry_points.add_entry_point(transfer_from());
    entry_points.add_entry_point(granularity());
    entry_points.add_entry_point(send());
    entry_points.add_entry_point(burn());
    entry_points.add_entry_point(is_operator_for());
    entry_points.add_entry_point(authorize_operator());
    entry_points.add_entry_point(revoke_operator());
    entry_points.add_entry_point(default_operators());
    entry_points.add_entry_point(operator_send());
    entry_points.add_entry_point(operator_burn());
    entry_points
}
