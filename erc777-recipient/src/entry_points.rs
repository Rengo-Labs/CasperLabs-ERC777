//! Contains definition of the entry points.
use alloc::{string::String, vec};

use casper_types::{
    {CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter},
    bytesrepr::Bytes, U256
};

use crate::{
    address::Address,
};
use crate::constants::{
    TOKENS_RECEIVED_ENTRY_POINT,
    OPERATOR_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME, TO_RUNTIME_ARG_NAME,
    AMOUNT_RUNTIME_ARG_NAME, USER_DATA_RUNTIME_ARG_NAME, OPERATOR_DATA_RUNTIME_ARG_NAME
};

/// Entry Point `tokens_received`
pub fn tokens_received() -> EntryPoint {
    EntryPoint::new(
        String::from(TOKENS_RECEIVED_ENTRY_POINT),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(FROM_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TO_RUNTIME_ARG_NAME, Address::cl_type()),
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
    entry_points
}