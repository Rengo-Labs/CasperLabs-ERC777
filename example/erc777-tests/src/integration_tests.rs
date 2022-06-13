#[cfg(test)]
mod test_fixture;
mod test_erc1820;

#[cfg(test)]
mod tests {
    use casper_types::{Key, U256};
    use casper_types::bytesrepr::{Bytes};
    extern crate base64;
    use crate::test_fixture::{Sender, TestFixture};
    use crate::test_erc1820::{TestERC1820};

    #[test]
    fn should_install() {
        let erc1820 = TestERC1820::install_contract();
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        assert_eq!(fixture.token_granularity(), U256::one());
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
    }

    #[test]
    fn should_transfer_and_burn() {
        let approve_amount = U256::from(100);
        let transfer_amount = U256::from(42);

        let mut erc1820 = TestERC1820::install_contract();
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

        let erc1820 = TestERC1820::install_contract();
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
        let erc1820 = TestERC1820::install_contract();
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
