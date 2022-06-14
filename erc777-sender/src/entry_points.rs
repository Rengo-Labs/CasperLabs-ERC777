//! Contains definition of the entry points.
use alloc::{string::String, vec};

use casper_types::{{CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter}, U256, Key};
use constants::{ACCOUNT_RUNTIME_ARG_NAME, BURN_ENTRY_POINT};

use crate::constants::{
    TOKENS_TO_SEND_ENTRY_POINT, TRANSFER_ENTRY_POINT,
    OPERATOR_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME, TO_RUNTIME_ARG_NAME,
    AMOUNT_RUNTIME_ARG_NAME, USER_DATA_RUNTIME_ARG_NAME, OPERATOR_DATA_RUNTIME_ARG_NAME
};

/// `tokens_to_send` Entry Point
pub fn tokens_to_send() -> EntryPoint {
    EntryPoint::new(
        String::from(TOKENS_TO_SEND_ENTRY_POINT),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(FROM_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(TO_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(USER_DATA_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, String::cl_type())
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
            Parameter::new(USER_DATA_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, String::cl_type())
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
            Parameter::new(USER_DATA_RUNTIME_ARG_NAME, String::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, String::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC777 Sender token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(tokens_to_send());
    entry_points.add_entry_point(transfer());
    entry_points.add_entry_point(burn());
    entry_points
}
