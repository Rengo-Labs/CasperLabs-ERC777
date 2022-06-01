//! Contains definition of the entry points.
use alloc::{string::String, vec, vec::Vec};

use casper_types::{
    {CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, Key},
    bytesrepr::Bytes
};

use crate::{
    address::Address,
};
use crate::constants::{
    ACCOUNT_RUNTIME_ARG_NAME, GET_INTERFACE_ENTRY_POINT, I_HASH_RUNTIME_ARG_NAME,
    IMPLEMENTER_RUNTIME_ARG_NAME, SET_INTERFACE_ENTRY_POINT
};

pub fn get_interface_implementer() -> EntryPoint {
    EntryPoint::new(
        String::from(GET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn set_interface_implementer() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Address::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(IMPLEMENTER_RUNTIME_ARG_NAME, Address::cl_type())
        ],
        Address::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(set_interface_implementer());
    entry_points.add_entry_point(get_interface_implementer());
    entry_points
}
