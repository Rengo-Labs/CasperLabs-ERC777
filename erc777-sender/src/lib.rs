//! A library for developing ERC20 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC20`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.
//!
//! To create an example ERC20 contract which uses this library, use the cargo-casper tool:
//!
//! ```bash
//! cargo install cargo-casper
//! cargo casper --erc20 <PATH TO NEW PROJECT>
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;
extern crate casper_types;

mod address;
pub mod constants;
pub mod entry_points;
mod error;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryInto;

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::Bytes, {contracts::NamedKeys, EntryPoints, Key, URef}, ContractHash, RuntimeArgs};

pub use address::Address;
use constants::{ERC820_REGISTRY_CONTRACT_NAME, NAME_KEY_NAME};
pub use error::Error;

#[derive(Default)]
pub struct ERC820 {
    registry_uref: OnceCell<URef>
}

impl ERC820 {
    fn new(registry_uref: URef) -> Self {
        Self {
            registry_uref: registry_uref.into()
        }
    }

    fn initialization() {
        let token_contract: Key = runtime::get_key(REGISTRY_CONTRACT_NAME)
            .unwrap();

        runtime::call_contract(
            ContractHash(token_contract.into_hash().unwrap()),
            casper_erc20::constants::TOTAL_SUPPLY_ENTRY_POINT_NAME,
            RuntimeArgs::default(),
        );
    }

    /// Installs the ERC20 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(
        name: String
    ) -> Result<ERC820, Error> {
        let default_entry_points = entry_points::default();
        ERC20::install_custom(
            name,
            ERC820_REGISTRY_CONTRACT_NAME,
            default_entry_points,
        )
    }

    /// Returns the name of the token.
    pub fn set_interface_implementer(&self) -> String {
        detail::read_from(NAME_KEY_NAME)
    }

    /// Returns the symbol of the token.
    pub fn get_interface_implementer(&self) -> String {
        detail::read_from(NAME_KEY_NAME)
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
        name: String,
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC820, Error> {
        let registry_uref = storage::new_dictionary(NAME_KEY_NAME).unwrap_or_revert();

        let mut named_keys = NamedKeys::new();

        let name_key = {
            let name_uref = storage::new_uref(name).into_read();
            Key::from(name_uref)
        };

        named_keys.insert(NAME_KEY_NAME.to_string(), name_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC820::new(
            registry_uref
        ))
    }
}
