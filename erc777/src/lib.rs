//! A library for developing ERC20 tokens for the Casper network.
//!
//! The main functionality is provided via the [`ERC20`] struct, and is intended to be consumed by a
//! smart contract written to be deployed on the Casper network.
//!
//! To create an example ERC20 contract which uses this library, use the cargo-casper tool:
//!
//! ```bash
//! cargo install cargo-casper
//! cargo casper --erc777 <PATH TO NEW PROJECT>
//! ```

#![warn(missing_docs)]
#![no_std]

extern crate alloc;

mod address;
mod allowances;
mod balances;
pub mod constants;
mod detail;
pub mod entry_points;
mod error;
mod total_supply;
mod operators;
mod external_contracts;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::TryInto;

use once_cell::unsync::OnceCell;

use casper_contract::{
    contract_api::{runtime, storage},
    unwrap_or_revert::UnwrapOrRevert,
};
use casper_types::{
    {contracts::NamedKeys, EntryPoints, Key, URef, U256},
    ContractHash
};

pub use address::Address;
use constants::{
    ALLOWANCES_KEY_NAME, BALANCES_KEY_NAME, DECIMALS_KEY_NAME, ERC20_TOKEN_CONTRACT_NAME,
    NAME_KEY_NAME, SYMBOL_KEY_NAME, TOTAL_SUPPLY_KEY_NAME,
    DECIMALS_KEY_VALUE, GRANULARITY_KEY_NAME, OPERATORS_KEY_NAME, REGISTRY_CONTRACT_NAME
};
pub use error::Error;

/// Implementation of ERC20 standard functionality.
#[derive(Default)]
pub struct ERC777 {
    balances_uref: OnceCell<URef>,
    allowances_uref: OnceCell<URef>,
    total_supply_uref: OnceCell<URef>,
    operators_uref: OnceCell<URef>,
    registry_uref: OnceCell<URef>
}

impl ERC777 {
    fn new(balances_uref: URef, allowances_uref: URef, total_supply_uref: URef, operators_uref: URef, registry_uref: URef) -> Self {
        Self {
            balances_uref: balances_uref.into(),
            allowances_uref: allowances_uref.into(),
            total_supply_uref: total_supply_uref.into(),
            operators_uref: operators_uref.into(),
            registry_uref: registry_uref.into()
        }
    }

    fn total_supply_uref(&self) -> URef {
        *self
            .total_supply_uref
            .get_or_init(total_supply::total_supply_uref)
    }

    fn read_total_supply(&self) -> U256 {
        total_supply::read_total_supply_from(self.total_supply_uref())
    }

    fn write_total_supply(&self, total_supply: U256) {
        total_supply::write_total_supply_to(self.total_supply_uref(), total_supply)
    }

    fn balances_uref(&self) -> URef {
        *self.balances_uref.get_or_init(balances::get_balances_uref)
    }

    fn registry_uref(&self) -> URef {
        *self.registry_uref.get_or_init(external_contracts::get_registry_uref)
    }

    fn read_balance(&self, owner: Address) -> U256 {
        balances::read_balance_from(self.balances_uref(), owner)
    }

    fn write_balance(&mut self, owner: Address, amount: U256) {
        balances::write_balance_to(self.balances_uref(), owner, amount)
    }

    fn allowances_uref(&self) -> URef {
        *self
            .allowances_uref
            .get_or_init(allowances::allowances_uref)
    }

    fn read_allowance(&self, owner: Address, spender: Address) -> U256 {
        allowances::read_allowance_from(self.allowances_uref(), owner, spender)
    }

    fn write_allowance(&mut self, owner: Address, spender: Address, amount: U256) {
        allowances::write_allowance_to(self.allowances_uref(), owner, spender, amount)
    }

    fn operators_uref(&self) -> URef {
        *self.operators_uref
            .get_or_init(operators::operators_uref)
    }

    fn transfer_balance(
        &mut self,
        sender: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        balances::transfer_balance(self.balances_uref(), sender, recipient, amount)
    }

    /// Installs the ERC20 contract with the default set of entry points.
    ///
    /// This should be called from within `fn call()` of your contract.
    pub fn install(
        name: String,
        symbol: String,
        granularity: U256,
        initial_supply: U256,
        erc1820_hash: ContractHash
    ) -> Result<ERC777, Error> {
        let default_entry_points = entry_points::default();
        ERC777::install_custom(
            name,
            symbol,
            granularity,
            initial_supply,
            ERC20_TOKEN_CONTRACT_NAME,
            default_entry_points,
            erc1820_hash,
        )
    }

    /// Returns the name of the token.
    pub fn name(&self) -> String {
        detail::read_from(NAME_KEY_NAME)
    }

    /// Returns the symbol of the token.
    pub fn symbol(&self) -> String {
        detail::read_from(SYMBOL_KEY_NAME)
    }

    /// Returns the decimals of the token.
    pub fn decimals(&self) -> u8 {
        detail::read_from(DECIMALS_KEY_NAME)
    }

    /// Returns the total supply of the token.
    pub fn total_supply(&self) -> U256 {
        self.read_total_supply()
    }

    /// Returns the granularity of the token.
    pub fn granularity(&self) -> U256 {
        detail::read_from(GRANULARITY_KEY_NAME)
    }

    /// Returns the balance of `owner`.
    pub fn balance_of(&self, owner: Address) -> U256 {
        self.read_balance(owner)
    }

    /// Transfers `amount` of tokens from the direct caller to `recipient`.
    pub fn transfer(&mut self, recipient: Address, amount: U256) -> Result<(), Error> {
        let sender = detail::get_immediate_caller_address()?;
        self.transfer_balance(sender, recipient, amount)
    }

    /// Transfers `amount` of tokens from `owner` to `recipient` if the direct caller has been
    /// previously approved to spend the specified amount on behalf of the owner.
    pub fn transfer_from(
        &mut self,
        owner: Address,
        recipient: Address,
        amount: U256,
    ) -> Result<(), Error> {
        let spender = detail::get_immediate_caller_address()?;
        if amount.is_zero() {
            return Ok(());
        }
        let spender_allowance = self.read_allowance(owner, spender);
        let new_spender_allowance = spender_allowance
            .checked_sub(amount)
            .ok_or(Error::InsufficientAllowance)?;
        self.transfer_balance(owner, recipient, amount)?;
        self.write_allowance(owner, spender, new_spender_allowance);
        Ok(())
    }

    /// Allows `spender` to transfer up to `amount` of the direct caller's tokens.
    pub fn approve(&mut self, spender: Address, amount: U256) -> Result<(), Error> {
        let owner = detail::get_immediate_caller_address()?;
        self.write_allowance(owner, spender, amount);
        Ok(())
    }

    /// Returns the amount of `owner`'s tokens allowed to be spent by `spender`.
    pub fn allowance(&self, owner: Address, spender: Address) -> U256 {
        self.read_allowance(owner, spender)
    }

    /// Mints `amount` new tokens and adds them to `owner`'s balance and to the token total supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn mint(&mut self, owner: Address, amount: U256) -> Result<(), Error> {
        let new_balance = {
            let balance = self.read_balance(owner);
            balance.checked_add(amount).ok_or(Error::Overflow)?
        };
        let new_total_supply = {
            let total_supply: U256 = self.read_total_supply();
            total_supply.checked_add(amount).ok_or(Error::Overflow)?
        };
        self.write_balance(owner, new_balance);
        self.write_total_supply(new_total_supply);
        Ok(())
    }

    /// Allows burning a ´amount´ tokens straight of the caller's tokens.
    pub fn burn(&mut self, amount: U256, data: String) -> Result<(), Error> {
        let owner: Address = detail::get_immediate_caller_address()?;

        let new_total_supply: U256 = balances::burn(
            self.balances_uref(),
            self.registry_uref(),
            owner,
            amount,
            self.read_total_supply(),
            data,
            String::from(""),
            true
        ).unwrap_or_revert();

        self.write_total_supply(new_total_supply);

        Ok(())
    }

    /// Burns (i.e. subtracts) `amount` of tokens from `owner`'s balance and from the token total
    /// supply.
    ///
    /// # Security
    ///
    /// This offers no security whatsoever, hence it is advised to NOT expose this method through a
    /// public entry point.
    pub fn _burn(&mut self, owner: Address, amount: U256) -> Result<(), Error> {

        let new_total_supply: U256 = balances::burn(
            self.balances_uref(),
            self.registry_uref(),
            owner,
            amount,
            self.read_total_supply(),
            String::from(""),
            String::from(""),
            true
        ).unwrap_or_revert();

        self.write_total_supply(new_total_supply);

        Ok(())
    }

    //todo event
    /// Allows sending a ´amount´ tokens to a ´recipient´ of the caller's tokens.
    pub fn send(&mut self, recipient: Address, amount: U256, data: String) -> Result<(), Error> {
        let caller: Address = detail::get_immediate_caller_address()?;

        balances::send_balance(
            self.balances_uref(),
            self.registry_uref(),
            caller,
            recipient,
            amount,
            data,
            String::from(""),
            true
        )
    }

    /// Check up if the ´operator´ exists for this account.
    pub fn is_operator_for(&mut self, operator: Address, token: Address) -> Result<bool, Error> {
        let caller: Address = detail::get_immediate_caller_address()?;

        let result = operators::check_if_exists(self.operators_uref(), caller, operator)?;
        Ok(result)
    }

    /// Grant permission to an ´operator´ to send and burn tokens in behalf of the owner.
    pub fn authorize_operator(&mut self, operator: Address) -> Result<(), Error> {
        let caller: Address = detail::get_immediate_caller_address()?;
        operators::concat_in_string(
            self.operators_uref(),
            caller,
            operator
        );
        Ok(())
    }

    /// Delete an ´operator´ for this account
    pub fn revoke_operator(&mut self, operator: Address) -> Result<(), Error> {
        let caller: Address = detail::get_immediate_caller_address()?;
        operators::get_rid_of(
            self.operators_uref(),
            caller,
            operator
        );
        Ok(())
    }

    /// Return a list of Operator's Address
    pub fn default_operators(&mut self) -> Result<Vec<Address>, Error> {
        let caller: Address = detail::get_immediate_caller_address()?;

        let addresses = operators::make_array(
            self.operators_uref(),
            caller
        );

        Ok(addresses)
    }

    /// Allows sending a ´amount´ tokens to a ´recipient´ in behalf of the caller's tokens.
    pub fn operator_send(
        &mut self,
        sender: Address,
        recipient: Address,
        amount: U256,
        data: String,
        operator_data: String
    ) -> Result<(), Error> {
        let caller = runtime::get_caller();

        let result = operators::check_if_exists(self.operators_uref(), sender,Address::Account(caller))?;

        balances::send_balance(
            self.balances_uref(),
            self.registry_uref(),
            sender,
            recipient,
            amount,
            data,
            operator_data,
            result
        )
    }

    /// Allows burning a ´amount´ tokens in behalf of the tokens' owner.
    pub fn operator_burn(
        &mut self,
        account: Address,
        amount: U256,
        data: String,
        operator_data: String
    ) -> Result<(), Error>{
        let owner= runtime::get_caller();

        let new_total_supply: U256 = balances::burn(
            self.balances_uref(),
            self.registry_uref(),
            account,
            amount,
            self.read_total_supply(),
            data,
            operator_data,
            operators::check_if_exists(self.operators_uref(), account,Address::Account(owner)).unwrap_or_default()
        ).unwrap_or_revert();

        self.write_total_supply(new_total_supply);
        Ok(())
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
        symbol: String,
        granularity: U256,
        initial_supply: U256,
        contract_key_name: &str,
        entry_points: EntryPoints,
        erc1820_hash: ContractHash
    ) -> Result<ERC777, Error> {
        let balances_uref = storage::new_dictionary(BALANCES_KEY_NAME).unwrap_or_revert();
        let allowances_uref = storage::new_dictionary(ALLOWANCES_KEY_NAME).unwrap_or_revert();
        let operators_uref = storage::new_dictionary(OPERATORS_KEY_NAME).unwrap_or_revert();
        let registry_uref = storage::new_dictionary(REGISTRY_CONTRACT_NAME).unwrap_or_revert();
        // We need to hold on a RW access rights because tokens can be minted or burned.
        let total_supply_uref = storage::new_uref(initial_supply).into_read_write();

        let decimals_key = {
            let decimals_uref = storage::new_uref(DECIMALS_KEY_VALUE).into_read();
            Key::from(decimals_uref)
        };

        let granularity_key = {
            let granularity_uref = storage::new_uref(granularity).into_read();
            Key::from(granularity_uref)
        };

        let mut named_keys = NamedKeys::new();

        let name_key = {
            let name_uref = storage::new_uref(name).into_read();
            Key::from(name_uref)
        };

        let symbol_key = {
            let symbol_uref = storage::new_uref(symbol).into_read();
            Key::from(symbol_uref)
        };

        let total_supply_key = Key::from(total_supply_uref);

        let balances_dictionary_key = {
            // Sets up initial balance for the caller - either an account, or a contract.
            let caller = detail::get_caller_address()?;
            balances::write_balance_to(balances_uref, caller, initial_supply);

            runtime::remove_key(BALANCES_KEY_NAME);

            Key::from(balances_uref)
        };

        let allowances_dictionary_key = {
            runtime::remove_key(ALLOWANCES_KEY_NAME);

            Key::from(allowances_uref)
        };

        let operators_dictionary_key = {
            runtime::remove_key(OPERATORS_KEY_NAME);
            Key::from(operators_uref)
        };

        let registry_key = {
            if ContractHash::default().ne(&erc1820_hash) {
                external_contracts::set_registry(
                    registry_uref,
                    erc1820_hash
                );
            }

            runtime::remove_key(REGISTRY_CONTRACT_NAME);
            Key::from(registry_uref)
        };

        named_keys.insert(NAME_KEY_NAME.to_string(), name_key);
        named_keys.insert(SYMBOL_KEY_NAME.to_string(), symbol_key);
        named_keys.insert(DECIMALS_KEY_NAME.to_string(), decimals_key);
        named_keys.insert(GRANULARITY_KEY_NAME.to_string(), granularity_key);
        named_keys.insert(BALANCES_KEY_NAME.to_string(), balances_dictionary_key);
        named_keys.insert(ALLOWANCES_KEY_NAME.to_string(), allowances_dictionary_key);
        named_keys.insert(TOTAL_SUPPLY_KEY_NAME.to_string(), total_supply_key);
        named_keys.insert(OPERATORS_KEY_NAME.to_string(), operators_dictionary_key);
        named_keys.insert(REGISTRY_CONTRACT_NAME.to_string(), registry_key);

        let (contract_hash, _version) =
            storage::new_contract(entry_points, Some(named_keys), None, None);

        // Hash of the installed contract will be reachable through named keys.
        runtime::put_key(contract_key_name, Key::from(contract_hash));

        Ok(ERC777::new(
            balances_uref,
            allowances_uref,
            total_supply_uref,
            operators_uref,
            registry_uref
        ))
    }
}
