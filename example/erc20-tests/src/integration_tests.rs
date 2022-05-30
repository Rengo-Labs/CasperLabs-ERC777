#[cfg(test)]
mod test_fixture;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};
    use casper_types::bytesrepr::{Bytes};
    extern crate base64;
    use crate::test_fixture::{Sender, TestFixture};

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        //assert_eq!(fixture.token_decimals(), TestFixture::TOKEN_DECIMALS);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
    }

    #[test]
    fn should_transfer() {
        let mut fixture = TestFixture::install_contract();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
        let transfer_amount_1 = U256::from(42);
        fixture.transfer(
            Key::from(fixture.bob),
            transfer_amount_1,
            Sender(fixture.ali),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - transfer_amount_1)
        );

        let transfer_amount_2 = U256::from(20);
        fixture.transfer(
            Key::from(fixture.ali),
            transfer_amount_2,
            Sender(fixture.bob),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply() - transfer_amount_1 + transfer_amount_2),
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(transfer_amount_1 - transfer_amount_2)
        );
    }

    #[test]
    fn should_transfer_full_amount() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance,
            Sender(fixture.ali),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(initial_ali_balance)
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(U256::zero())
        );

        fixture.transfer(
            Key::from(fixture.ali),
            initial_ali_balance,
            Sender(fixture.bob),
        );

        assert_eq!(
            fixture.balance_of(Key::from(fixture.bob)),
            Some(U256::zero())
        );
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(initial_ali_balance)
        );
    }

    #[should_panic(expected = "ApiError::User(65534) [131070]")]
    #[test]
    fn should_not_transfer_with_insufficient_balance() {
        let mut fixture = TestFixture::install_contract();

        let initial_ali_balance = fixture.balance_of(Key::from(fixture.ali)).unwrap();
        assert_eq!(fixture.balance_of(Key::from(fixture.bob)), None);

        fixture.transfer(
            Key::from(fixture.bob),
            initial_ali_balance + U256::one(),
            Sender(fixture.ali),
        );
    }

    #[test]
    fn should_transfer_from() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        let owner_balance_before = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");
        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            transfer_amount,
            Sender(spender),
        );

        assert_eq!(
            fixture.balance_of(Key::from(owner)),
            Some(owner_balance_before - transfer_amount),
            "should decrease balance of the owner"
        );
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount - transfer_amount),
            "should decrease allowance of the spender"
        );
        assert_eq!(
            fixture.balance_of(Key::from(recipient)),
            Some(transfer_amount),
            "recipient should receive tokens"
        );
    }

    #[should_panic(expected = "ApiError::User(65533) [131069]")]
    #[test]
    fn should_not_transfer_from_more_than_approved() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);
        assert!(approve_amount > transfer_amount);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;
        let recipient = fixture.joe;

        fixture.approve(Key::from(spender), approve_amount, Sender(owner));
        assert_eq!(
            fixture.allowance(Key::from(owner), Key::from(spender)),
            Some(approve_amount)
        );

        fixture.transfer_from(
            Key::from(owner),
            Key::from(recipient),
            approve_amount + U256::one(),
            Sender(spender),
        );
    }

    #[test]
    fn should_transfer_and_burn() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);

        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let spender = fixture.bob;

        let owner_balance_before = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");

        fixture.approve(Key::from(owner), approve_amount, Sender(owner));
        fixture.transfer_from(
            Key::from(owner),
            Key::from(spender),
            transfer_amount,
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
            Sender(owner),
        );

        fixture.burn(
            U256::one(),
            Sender(spender),
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
    fn should_send_behalf_of() {
        let transfer_amount = U256::from(42);
        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let operator = fixture.bob;
        let recipient = fixture.joe;

        //let mut data = base64::encode(operator.to_bytes().unwrap());
        //data.push_str(base64::encode(operator.to_bytes().unwrap()).as_str());
        //let decrypt = AccountHash::from_vec(base64::decode(nuevo.next().unwrap()).unwrap()).unwrap();

        fixture.authorize_operator(Key::from(operator), Sender(owner));
        let added_operator = fixture.operators(Key::from(owner));
        let mut expected_operator = operator.to_string().clone();
        expected_operator.push('|');
        assert_eq!(
            added_operator,
            Some(expected_operator)
        );

        fixture.operator_send(
            Key::from(owner),
            Key::from(recipient),
            transfer_amount,
            Bytes::new(),
            Bytes::new(),
            Sender(owner)
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
    fn should_burn_behalf_of() {
        let mut fixture = TestFixture::install_contract();

        let owner = fixture.ali;
        let operator = fixture.bob;

        let owner_balance_before_burning = fixture
            .balance_of(Key::from(owner))
            .expect("owner should have balance");

        fixture.authorize_operator(Key::from(operator), Sender(owner));
        let added_operator = fixture.operators(Key::from(owner));
        let mut expected_operator = operator.to_string().clone();
        expected_operator.push('|');
        assert_eq!(
            added_operator,
            Some(expected_operator)
        );

        fixture.operator_burn(
            Key::from(owner),
            U256::one(),
            Bytes::new(),
            Bytes::new(),
            Sender(operator)
        );
        let balance_after_burning = fixture.balance_of(Key::from(owner));
        assert_eq!(
            owner_balance_before_burning - U256::one(),
            balance_after_burning.unwrap(),
            "tokens owner before burning"
        );
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
