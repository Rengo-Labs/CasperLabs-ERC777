//! A library for developing ERC1820 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC1820`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.

#![warn(missing_docs)]
#![no_std]

extern crate alloc;
extern crate casper_types;
extern crate casper_contract;
extern crate once_cell;

mod address;
pub mod constants;
pub mod entry_points;
mod implementers_registry;
mod managers_registry;
mod detail;

use alloc::string::{String, ToString};
use core::convert::TryInto;

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{{contracts::NamedKeys, EntryPoints, Key, URef}, ApiError};

use constants::{
    ERC1820_REGISTRY_CONTRACT_NAME, IMPLEMENTERS_REGISTRY_KEY_NAME, MANAGERS_REGISTRY_KEY_NAME
};

/// Struct
#[derive(Default)]
pub struct ERC1820 {
    implementer_uref: OnceCell<URef>,
    manager_uref: OnceCell<URef>
}

impl ERC1820 {
    fn new(implementer_uref: URef, manager_uref: URef) -> Self {
        Self {
            implementer_uref: implementer_uref.into(),
            manager_uref: manager_uref.into()
        }
    }

    fn implementer_registry_uref(&self) -> URef {
        *self.implementer_uref.get_or_init(implementers_registry::implementers_registry)
    }

    fn managers_registry_uref(&self) -> URef {
        *self.manager_uref.get_or_init(managers_registry::managers_registry)
    }

    /// Returns the name of the token.
    pub fn set_interface_implementer(
        &self,
        account: Key,
        i_hash: String,
        implementer: Key
    ) -> Result<(), ApiError> {
        implementers_registry::create_or_update_implementer(
            self.implementer_registry_uref(),
            account,
            i_hash,
            implementer
        );
        Ok(())
    }

    /// Returns the symbol of the token.
    pub fn get_interface_implementer(&self, account: Key, i_hash: String) -> Result<Key, ApiError> {
        let result = implementers_registry::get_implementer(
            self.implementer_registry_uref(),
            account,
            i_hash
        );

        Ok(result)
    }

    /// it adds a new manager for performing operations
    pub fn set_manager(&self, account: Key, new_manager: Key) -> Result<(), ApiError> {
        managers_registry::set_manager(
            self.managers_registry_uref(),
            account,
            new_manager
        );

        Ok(())
    }

    /// it returns a manager for the parameter account
    pub fn get_manager(&self, account: Key) -> Result<Key, ApiError> {
        let manager = managers_registry::get_manager(
            self.implementer_registry_uref(),
            account
        );

        Ok(manager)
    }

    /// Installs the ERC1820 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install() -> Result<ERC1820, ApiError> {
        let default_entry_points = entry_points::default();
        ERC1820::install_custom(
            ERC1820_REGISTRY_CONTRACT_NAME,
            default_entry_points,
        )
    }

    /// Installs the ERC20 contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC20::install`] instead, as it will create the default set
    /// of ERC20 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC1820, ApiError> {

        let mut implementer_uref: URef = URef::default();
        if runtime::get_key(IMPLEMENTERS_REGISTRY_KEY_NAME).is_some() {
            implementer_uref = runtime::get_key(IMPLEMENTERS_REGISTRY_KEY_NAME)
                .unwrap_or_revert()
                .try_into()
                .unwrap_or_revert();
        } else {
            implementer_uref = storage::new_dictionary(IMPLEMENTERS_REGISTRY_KEY_NAME)
                .unwrap_or_revert();
        }

        let mut manager_uref: URef = URef::default();
        if runtime::get_key(MANAGERS_REGISTRY_KEY_NAME).is_some() {
            manager_uref = runtime::get_key(MANAGERS_REGISTRY_KEY_NAME)
                .unwrap_or_revert()
                .try_into()
                .unwrap_or_revert();
        } else {
            manager_uref = storage::new_dictionary(MANAGERS_REGISTRY_KEY_NAME)
                .unwrap_or_revert();
        }

        let mut named_keys = NamedKeys::new();

        let implementer_key = {
            runtime::remove_key(IMPLEMENTERS_REGISTRY_KEY_NAME);
            Key::from(implementer_uref)
        };

        let manager_key = {
            runtime::remove_key(MANAGERS_REGISTRY_KEY_NAME);
            Key::from(manager_uref)
        };

        named_keys.insert(IMPLEMENTERS_REGISTRY_KEY_NAME.to_string(), implementer_key);
        named_keys.insert(MANAGERS_REGISTRY_KEY_NAME.to_string(), manager_key);

        let (contract_hash, _version) =
            storage::new_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC1820::new(
            implementer_uref,
            manager_uref
        ))
    }
}
