//! Implementation of balances.
use alloc::string::{String};

use casper_contract::{contract_api::storage, unwrap_or_revert::UnwrapOrRevert};
use casper_types::{bytesrepr::{ToBytes}, URef, U256};
use casper_types::account::AccountHash;
use casper_types::bytesrepr::Bytes;

use crate::{constants::{BALANCES_KEY_NAME, HASH_ERC1820_RECIPIENT, HASH_ERC1820_SENDER}, detail, error::Error, Address};
use crate::Address::Account;
use crate::external_contracts::{get_interface, tokens_received, tokens_to_send};

/// Creates a dictionary item key for a dictionary item.
#[inline]
fn make_dictionary_item_key(owner: Address) -> String {
    let preimage = owner.to_bytes().unwrap_or_revert();
    base64::encode(&preimage)
}

pub(crate) fn get_balances_uref() -> URef {
    detail::get_uref(BALANCES_KEY_NAME)
}

/// Writes token balance of a specified account into a dictionary.
pub(crate) fn write_balance_to(balances_uref: URef, address: Address, amount: U256) {
    let dictionary_item_key = make_dictionary_item_key(address);
    storage::dictionary_put(balances_uref, &dictionary_item_key, amount);
}

/// Reads token balance of a specified account.
///
/// If a given account does not have balances in the system, then a 0 is returned.
pub(crate) fn read_balance_from(balances_uref: URef, address: Address) -> U256 {
    let dictionary_item_key = make_dictionary_item_key(address);

    storage::dictionary_get(balances_uref, &dictionary_item_key)
        .unwrap_or_revert()
        .unwrap_or_default()
}

/// Transfer tokens from the `sender` to the `recipient`.
///
/// This function should not be used directly by contract's entrypoint as it does not validate the
/// sender.
pub(crate) fn transfer_balance(
    balances_uref: URef,
    sender: Address,
    recipient: Address,
    amount: U256,
) -> Result<(), Error> {
    if sender == recipient || amount.is_zero() {
        return Ok(());
    }

    let new_sender_balance = {
        let sender_balance = read_balance_from(balances_uref, sender);
        sender_balance
            .checked_sub(amount)
            .ok_or(Error::InsufficientBalance)?
    };

    let new_recipient_balance = {
        let recipient_balance = read_balance_from(balances_uref, recipient);
        recipient_balance
            .checked_add(amount)
            .ok_or(Error::Overflow)?
    };

    write_balance_to(balances_uref, sender, new_sender_balance);
    write_balance_to(balances_uref, recipient, new_recipient_balance);

    Ok(())
}

pub(crate) fn send_balance(
    balances_uref: URef,
    registry_uref: URef,
    sender: Address,
    recipient: Address,
    amount: U256,
    data: Bytes,
    operator_data: Bytes,
    is_operator: bool
) -> Result<(), Error> {

    if is_operator == false {
        return Err(Error::InvalidOperator);
    }

    let implementer = get_interface(
        registry_uref,
        sender,
        Bytes::from(HASH_ERC1820_SENDER.to_bytes().unwrap())
    );

    if implementer.into_hash().is_some() {
        tokens_to_send(
            sender,
            sender,
            recipient,
            amount,
            data.clone(),
            operator_data.clone(),
            implementer
        );
    }

    let result = transfer_balance(balances_uref, sender, recipient, amount);
    if result.is_err() {
        return result;
    }

    let implementer = get_interface(
        registry_uref,
        sender,
        Bytes::from(HASH_ERC1820_RECIPIENT.to_bytes().unwrap())
    );

    if implementer.into_hash().is_some() {
        tokens_received(
            sender,
            sender,
            recipient,
            amount,
            data.clone(),
            operator_data.clone(),
            implementer
        );
    }

    Ok(())
}

pub fn _mint(
    balances_uref: URef,
    registry_uref: URef,
    owner: Address,
    amount: U256,
    total_supply: U256
) -> Result<U256, Error> {
    let new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance.checked_add(amount).ok_or(Error::Overflow)?
    };
    let new_total_supply = {
        total_supply.checked_add(amount).ok_or(Error::Overflow)?
    };

    write_balance_to(balances_uref, owner, new_balance);

    let implementer = get_interface(
        registry_uref,
        owner,
        Bytes::from(HASH_ERC1820_RECIPIENT.to_bytes().unwrap())
    );

    if implementer.into_hash().is_some() {
        tokens_received(
            Account(AccountHash::default()),
            Account(AccountHash::default()),
            owner,
            amount,
            Bytes::default(),
            Bytes::default(),
            implementer
        );
    }

    Ok(new_total_supply)
}

pub fn burn(
    balances_uref: URef,
    registry_uref: URef,
    owner: Address,
    amount: U256,
    total_supply: U256,
    data: Bytes,
    operator_data: Bytes,
    is_operator: bool
) -> Result<U256, Error> {
    if is_operator == false {
        return Err(Error::InvalidOperator);
    }

    let implementer = get_interface(
        registry_uref,
        owner,
        Bytes::from(HASH_ERC1820_SENDER.to_bytes().unwrap())
    );

    if implementer.into_hash().is_some() {
        tokens_to_send(
            owner,
            owner,
            Account(AccountHash::default()),
            amount,
            data.clone(),
            operator_data.clone(),
            implementer
        );
    }

    let new_balance = {
        let balance = read_balance_from(balances_uref, owner);
        balance
            .checked_sub(amount)
            .ok_or(Error::InsufficientBalance)?
    };
    let new_total_supply = {
        total_supply.checked_sub(amount).ok_or(Error::Overflow)?
    };

    write_balance_to(balances_uref, owner, new_balance);

    Ok(new_total_supply)
}