//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};

use casper_types::{
    {CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, Key},
    bytesrepr::Bytes, U256
};

use crate::{
    address::Address,
};
use crate::constants::{
    TOKENS_TO_SEND_ENTRY_POINT,
    OPERATOR_RUNTIME_ARG_NAME, FROM_RUNTIME_ARG_NAME, TO_RUNTIME_ARG_NAME,
    AMOUNT_RUNTIME_ARG_NAME, USER_DATA_RUNTIME_ARG_NAME, OPERATOR_DATA_RUNTIME_ARG_NAME
};

pub fn tokens_to_send() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(OPERATOR_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(FROM_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(TO_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(AMOUNT_RUNTIME_ARG_NAME, U256::cl_type()),
            Parameter::new(USER_DATA_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(OPERATOR_DATA_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        Address::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(tokens_to_send());
    entry_points
}
