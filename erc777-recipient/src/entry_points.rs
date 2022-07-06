//! Contains definition of the entry points.
use alloc::{string::String, vec};

use casper_types::{{CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter}, Key, U256};
use casper_types::bytesrepr::Bytes;
use constants::{ACCOUNT_RUNTIME_ARG_NAME, BALANCE_OF_ENTRY_POINT_NAME, TOKENS_RECEIVED_ENTRY_POINT, OPERATOR_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME, TO_RUNTIME_ARG_NAME, AMOUNT_RUNTIME_ARG_NAME, USER_DATA_RUNTIME_ARG_NAME, OPERATOR_DATA_RUNTIME_ARG_NAME, TRANSFER_ENTRY_POINT, BURN_ENTRY_POINT};

/// Returns the `balance_of` entry point.
pub fn balance_of() -> EntryPoint {
    EntryPoint::new(
        String::from(BALANCE_OF_ENTRY_POINT_NAME),
        vec![Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Key::cl_type())],
        U256::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Entry Point `tokens_received`
pub fn tokens_received() -> EntryPoint {
    EntryPoint::new(
        String::from(TOKENS_RECEIVED_ENTRY_POINT),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(FROM_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(TO_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(USER_DATA_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `transfer` Entry Point
pub fn transfer() -> EntryPoint {
    EntryPoint::new(
        String::from(TRANSFER_ENTRY_POINT),
        vec![
            Parameter::new(FROM_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(TO_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(USER_DATA_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the `burn` entry point.
pub fn burn() -> EntryPoint {
    EntryPoint::new(
        String::from(BURN_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(USER_DATA_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC777 Recipient token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(tokens_received());
    entry_points.add_entry_point(balance_of());
    entry_points.add_entry_point(transfer());
    entry_points.add_entry_point(burn());
    entry_points
}
