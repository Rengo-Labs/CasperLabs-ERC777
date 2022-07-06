#![feature(once_cell)]

#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};
    use casper_types::bytesrepr::Bytes;

    extern crate base64;
    use crate::test_fixture::{Sender, TestFixture};

    #[test]
    fn should_install() {
        let mut fixture = TestFixture::install_contract();
        fixture.add_erc1820_context();
        fixture.add_erc20_context();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        assert_eq!(fixture.token_granularity(), U256::one());
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
    }

    #[test]
    fn should_send_and_burn() {
        let transfer_amount = U256::from(42);

        let mut fixture = TestFixture::install_contract();
        fixture.add_erc1820_context();
        fixture.add_erc20_context();

        let owner = fixture.ali;
        let spender = fixture.bob;

        let owner_balance_before = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");

        fixture.send(
            Key::from(spender),
            transfer_amount,
            Bytes::default(),
            Sender(owner),
        );

        let balance_owner = fixture.balance_of(Key::from(owner));
        let balance_spender = fixture.balance_of(Key::from(spender));

        assert_eq!(
            balance_owner.unwrap(),
            owner_balance_before - transfer_amount,
            "tokens owner before burning"
        );

        assert_eq!(
            balance_spender.unwrap(),
            transfer_amount,
            "tokens spender before burning"
        );

        fixture.burn(
            U256::one(),
            Bytes::default(),
            Sender(owner)
        );

        fixture.burn(
            U256::one(),
            Bytes::default(),
            Sender(spender)
        );

        let balance_owner_after = fixture.balance_of(Key::from(owner));
        let balance_spender_after = fixture.balance_of(Key::from(spender));

        assert_eq!(
            balance_owner_after.unwrap(),
            balance_owner.unwrap() - U256::one(),
            "owner should sub tokens"
        );

        assert_eq!(
            balance_spender_after.unwrap(),
            balance_spender.unwrap() - U256::one(),
            "owner should sub tokens"
        );
    }

    #[test]
    fn should_send_on_behalf_of() {
        let transfer_amount = U256::from(42);

        let mut fixture = TestFixture::install_contract();
        fixture.add_erc1820_context();
        fixture.add_erc20_context();

        let owner = fixture.ali;
        let operator = fixture.bob;
        let recipient = fixture.joe;

        fixture.authorize_operator(Key::from(operator), Sender(owner));
        let added_operator = fixture.operators(Key::from(owner));
        let mut expected_operator = operator.to_string().clone();
        expected_operator.push('|');
        assert_eq!(
            added_operator,
            Some(base64::encode(expected_operator))
        );

        println!("operator_send operator");
        fixture.operator_send(
            Key::from(owner),
            Key::from(recipient),
            transfer_amount,
            Bytes::default(),
            Bytes::default(),
            Sender(operator)
        );
        let balance_recipient_after = fixture.balance_of(Key::from(recipient));
        assert_eq!(
            balance_recipient_after.unwrap(),
            transfer_amount,
            "tokens owner before burning"
        );

        fixture.revoke_operator(Key::from(operator), Sender(owner));
        let new_list = fixture.operators(Key::from(owner));
        assert_eq!(
            new_list,
            Some("".to_string())
        );
    }

    #[test]
    fn should_burn_on_behalf_of() {
        let mut fixture = TestFixture::install_contract();
        fixture.add_erc1820_context();
        fixture.add_erc20_context();

        let owner = fixture.ali;
        let operator = fixture.bob;
        let recipient = fixture.joe;

        let owner_balance_before_burning = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");

        fixture.authorize_operator(Key::from(operator), Sender(owner));
        let added_operator = fixture.operators(Key::from(owner));
        let mut expected_operator = operator.to_string().clone();
        expected_operator.push('|');
        assert_eq!(
            added_operator,
            Some(base64::encode(expected_operator))
        );

        fixture.operator_burn(
            Key::from(owner),
            U256::one(),
            Bytes::default(),
            Bytes::default(),
            Sender(operator)
        );

        fixture.operator_send(
            Key::from(owner),
            Key::from(recipient),
            U256::one(),
            Bytes::default(),
            Bytes::default(),
            Sender(operator)
        );

        let recipient_balance_before_burning = fixture
            .balance_of(Key::from(recipient))
            .expect("operator should have balance");

        fixture.operator_burn(
            Key::from(recipient),
            U256::one(),
            Bytes::default(),
            Bytes::default(),
            Sender(recipient)
        );

        let owner_balance_after_burning = fixture.balance_of(Key::from(owner));
        assert_eq!(
            owner_balance_before_burning - U256::one() - U256::one(),
            owner_balance_after_burning.unwrap(),
            "tokens owner before burning"
        );

        let recipient_balance_after_burning = fixture.balance_of(Key::from(recipient));
        assert_eq!(
            recipient_balance_before_burning - U256::one(),
            recipient_balance_after_burning.unwrap(),
            "tokens owner before burning"
        );
    }

    #[should_panic(expected = "ApiError::User(65530) [131066]")]
    #[test]
    fn should_throw_an_exception_by_not_having_assigned_operator_to_send_tokens() {
        let mut fixture = TestFixture::install_contract();
        fixture.add_erc1820_context();
        fixture.add_erc20_context();

        let owner = fixture.ali;
        let operator = fixture.bob;
        let recipient = fixture.joe;

        fixture.operator_send(
            Key::from(owner),
            Key::from(recipient),
            U256::one(),
            Bytes::default(),
            Bytes::default(),
            Sender(operator)
        );
    }

    #[should_panic(expected = "ApiError::User(65530) [131066]")]
    #[test]
    fn should_throw_an_exception_by_not_having_assigned_operator_to_burn_tokens() {
        let mut fixture = TestFixture::install_contract();
        fixture.add_erc1820_context();
        fixture.add_erc20_context();

        let owner = fixture.ali;
        let operator = fixture.bob;

        fixture.operator_burn(
            Key::from(owner),
            U256::one(),
            Bytes::default(),
            Bytes::default(),
            Sender(operator)
        );
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
