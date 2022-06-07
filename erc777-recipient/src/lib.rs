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
extern crate casper_contract;
extern crate once_cell;

mod address;
pub mod constants;
pub mod entry_points;
mod error;
mod recipient_notifier;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryInto;

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{bytesrepr::Bytes, {contracts::NamedKeys, EntryPoints, Key, URef}, ContractHash, RuntimeArgs, U256};
use casper_types::account::AccountHash;

pub use address::Address;
use constants::{ERC777_RECIPIENT_CONTRACT_NAME, MOVEMENTS_REGISTRY};
pub use error::Error;

#[derive(Default)]
pub struct ERC777Recipient {
    registry_uref: OnceCell<URef>
}

impl ERC777Recipient {
    fn new(registry_uref: URef) -> Self {
        Self {
            registry_uref: registry_uref.into()
        }
    }

    /// Installs the ERC20 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(
        account: AccountHash
    ) -> Result<ERC777Recipient, Error> {
        let default_entry_points = entry_points::default();
        ERC777Recipient::install_custom(
            account,
            ERC777_RECIPIENT_CONTRACT_NAME,
            default_entry_points,
        )
    }

    /// The movements or creations are performed in a registered account `to`.
    /// The type operation is conveyed by `from` being the zero address or not.
    pub fn tokens_received(
        operator: Address,
        from: Address,
        to: Address, amount: U256,
        data: Bytes,
        operator_data: Bytes
    ) -> Result<(), Error> {
        recipient_notifier::tokens_received(operator, from, to, amount, data, operator_data);
        Ok(())
    }

    /// Installs the ERC777 Recipient contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC20::install`] instead, as it will create the default set
    /// of ERC20 entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        account: AccountHash,
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC777Recipient, Error> {
        let registry_uref = storage::new_dictionary(MOVEMENTS_REGISTRY).unwrap_or_revert();

        let mut named_keys = NamedKeys::new();

        let movement_key = Key::from(registry_uref);

        named_keys.insert(MOVEMENTS_REGISTRY.to_string(), movement_key);

        let (contract_hash, _version) =
            storage::new_locked_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC777Recipient::new(
            registry_uref
        ))
    }
}
