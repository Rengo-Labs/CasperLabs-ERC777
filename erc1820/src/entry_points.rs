//! Contains definition of the entry points.
use alloc::{string::String, vec};

use casper_types::{
    {CLType, CLTyped, EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, Parameter, Key}
};
use casper_types::bytesrepr::Bytes;

use crate::constants::{
    ACCOUNT_RUNTIME_ARG_NAME, GET_INTERFACE_ENTRY_POINT, I_HASH_RUNTIME_ARG_NAME,
    IMPLEMENTER_RUNTIME_ARG_NAME, SET_INTERFACE_ENTRY_POINT, SET_MANAGER_ENTRY_POINT,
    GET_MANAGER_ENTRY_POINT, NEW_MANAGER_RUNTIME_ARG_NAME
};

/// `get_manager`
pub fn get_manager() -> EntryPoint {
    EntryPoint::new(
        String::from(GET_MANAGER_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Key::cl_type())
        ],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `set_manager`
pub fn set_manager() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_MANAGER_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(NEW_MANAGER_RUNTIME_ARG_NAME, Key::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `get_interface_implementer`
pub fn get_interface_implementer() -> EntryPoint {
    EntryPoint::new(
        String::from(GET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, Bytes::cl_type())
        ],
        Key::cl_type(),
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// `set_interface_implementer`
pub fn set_interface_implementer() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(IMPLEMENTER_RUNTIME_ARG_NAME, Key::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

pub fn interfaceHash() -> EntryPoint {
    EntryPoint::new(
        String::from(SET_INTERFACE_ENTRY_POINT),
        vec![
            Parameter::new(ACCOUNT_RUNTIME_ARG_NAME, Key::cl_type()),
            Parameter::new(I_HASH_RUNTIME_ARG_NAME, Bytes::cl_type()),
            Parameter::new(IMPLEMENTER_RUNTIME_ARG_NAME, Key::cl_type())
        ],
        CLType::Key,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    )
}

/// Returns the default set of ERC20 token entry points.
pub fn default() -> EntryPoints {
    let mut entry_points = EntryPoints::new();
    entry_points.add_entry_point(set_interface_implementer());
    entry_points.add_entry_point(get_interface_implementer());
    entry_points.add_entry_point(set_manager());
    entry_points.add_entry_point(get_manager());
    entry_points
}
