//! A library for developing ERC20 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC777Sender`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.

#![warn(missing_docs)]
#![no_std]

extern crate alloc;
extern crate casper_types;
extern crate casper_contract;
extern crate once_cell;

pub mod constants;
pub mod entry_points;
mod error;
mod erc1820_registry;
mod erc777_registry;
mod register_movements;

use alloc::string::{ToString};

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{{contracts::NamedKeys, EntryPoints, Key, URef}, U256, ContractHash};
use casper_types::bytesrepr::{Bytes, ToBytes};

use constants::{ERC777_REGISTRY_KEY_NAME, ERC777_SENDER_CONTRACT_NAME, HASH_ERC1820_SENDER, MOVEMENTS_REGISTRY_KEY_NAME};
pub use error::Error;

/// Struct
#[derive(Default)]
pub struct ERC777Sender {
    registry_uref: OnceCell<URef>,
    erc777_uref: OnceCell<URef>
}

impl ERC777Sender {
    fn new(registry_uref: URef, erc777_uref: URef) -> Self {
        Self {
            registry_uref: registry_uref.into(),
            erc777_uref: erc777_uref.into()
        }
    }

    /// it loads uref of the erc777 namekey
    fn erc777_uref(&self) -> URef {
        *self.erc777_uref.get_or_init(erc777_registry::get_erc777_uref)
    }

    /// it loads uref of the registry namekey
    fn registry_uref(&self) -> URef {
        *self.registry_uref.get_or_init(register_movements::get_registry_uref)
    }

    /// Installs the ERC20 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(erc1820_hash: ContractHash, erc777_hash: ContractHash) -> Result<ERC777Sender, Error> {
        let default_entry_points = entry_points::default();
        ERC777Sender::install_custom(
            erc1820_hash,
            erc777_hash,
            ERC777_SENDER_CONTRACT_NAME,
            default_entry_points,
        )
    }

    /// This call occurs _before_ the token contract's state is updated, so
    /// it can be used to query the pre-operation state.
    /// This function may revert to prevent the operation from being executed.
    pub fn tokens_to_send(
        self,
        operator: Key,
        from: Key,
        to: Key,
        amount: U256,
        user_data: Bytes,
        operator_data: Bytes
    ) -> Result<(), Error> {

        register_movements::tokens_received(self.registry_uref(), operator, from, to, amount, user_data, operator_data);
        Ok(())
    }

    /// it transfers tokens from erc777's operator_send
    pub fn transfer(
        self,
        from: Key,
        to: Key,
        amount: U256,
        user_data: Bytes,
        operator_data: Bytes
    ) -> Result<(), Error>{
        erc777_registry::transfer(
            self.erc777_uref(),
            from,
            to,
            amount,
            user_data,
            operator_data
        );
        Ok(())
    }

    /// Allows burning a ´amount´ tokens on behalf of the tokens' owner.
    pub fn burn(
        self,
        account: Key,
        amount: U256,
        user_data: Bytes,
        operator_data: Bytes
    ) -> Result<(), Error>{
        erc777_registry::burn(
            self.erc777_uref(),
            account,
            amount,
            user_data,
            operator_data
        );
        Ok(())
    }

    /// Installs the ERC777 Sender contract with a custom set of entry points.
    ///
    /// # Warning
    ///
    /// Contract developers should use [`ERC777Sender::install`] instead, as it will create the default set
    /// of ERC777 Sender entry points. Using `install_custom` with a different set of entry points might
    /// lead to problems with integrators such as wallets, and exchanges.
    #[doc(hidden)]
    pub fn install_custom(
        erc1820_hash: ContractHash,
        erc777_hash: ContractHash,
        contract_key_name: &str,
        entry_points: EntryPoints,
    ) -> Result<ERC777Sender, Error> {
        let registry_uref = storage::new_dictionary(MOVEMENTS_REGISTRY_KEY_NAME).unwrap_or_revert();
        let erc777_uref = storage::new_dictionary(ERC777_REGISTRY_KEY_NAME).unwrap_or_revert();

        let mut named_keys = NamedKeys::new();

        let movement_key = {
            runtime::remove_key(MOVEMENTS_REGISTRY_KEY_NAME);
            Key::from(registry_uref)
        };

        let erc777_key = {
            erc777_registry::save_erc777_contract(erc777_uref, erc777_hash);
            runtime::remove_key(ERC777_REGISTRY_KEY_NAME);
            Key::from(erc777_uref)
        };

        named_keys.insert(MOVEMENTS_REGISTRY_KEY_NAME.to_string(), movement_key);
        named_keys.insert(ERC777_REGISTRY_KEY_NAME.to_string(), erc777_key);

        let (contract_hash, _version) =
            storage::new_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        let contract_key = Key::from(contract_hash);
        runtime::put_key(contract_key_name, contract_key);

        let account_hash = runtime::get_caller();
        erc1820_registry::set_implementer(
            Key::from(account_hash),
            Bytes::from(HASH_ERC1820_SENDER.to_bytes().unwrap()),
            contract_key,
            erc1820_hash
        );

        Ok(ERC777Sender::new(
            registry_uref,
            erc777_uref
        ))
    }
}
