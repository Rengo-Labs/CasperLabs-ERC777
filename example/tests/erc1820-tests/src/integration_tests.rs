#[cfg(test)]
mod test_erc1820;

#[cfg(test)]
mod tests {
    use casper_types::{Key};
    use casper_types::account::AccountHash;
    use casper_types::bytesrepr::{Bytes, ToBytes};

    extern crate base64;
    use crate::test_erc1820::{Sender, TestERC1820};

    pub const HASH_ERC1820_SENDER: &str = "ERC777TokensSender";
    pub const HASH_ERC1820_RECIPIENT: &str = "ERC777TokensRecipient";

    #[test]
    fn should_register_a_recipient() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;
        let joe = fixture.joe;
        let tag_sender = HASH_ERC1820_RECIPIENT.to_string();

        fixture.set_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap()),
            Key::from(joe),
            Sender(ali)
        );

        let implementer = fixture.get_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap())
        );

        assert_eq!(
            Some(Key::from(joe)),
            implementer
        )
    }

    #[test]
    fn should_register_a_sender() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;
        let joe = fixture.joe;
        let tag_sender = HASH_ERC1820_SENDER.to_string();

        fixture.set_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap()),
            Key::from(joe),
            Sender(ali)
        );

        let implementer = fixture.get_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap())
        );

        assert_eq!(
            Some(Key::from(joe)),
            implementer
        )
    }

    #[should_panic(expected = "ApiError::User(1000) [66536]")]
    #[test]
    fn should_not_register_an_implementer() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;
        let joe = fixture.joe;

        let tag_sender = HASH_ERC1820_SENDER.to_string();

        fixture.set_interface_implementer(
            Key::from(joe),
            Bytes::from(tag_sender.clone().to_bytes().unwrap()),
            Key::from(joe),
            Sender(ali)
        );

        let implementer = fixture.get_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap())
        );

        assert_eq!(
            Some(Key::from(ali)),
            implementer
        )
    }


    #[should_panic(expected = "ApiError::User(1001) [66537]")]
    #[test]
    fn should_not_register_an_implementer_with_address_zero() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;

        let tag_sender = HASH_ERC1820_SENDER.to_string();

        fixture.set_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap()),
            Key::from(AccountHash::default()),
            Sender(ali)
        );

        let implementer = fixture.get_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap())
        );

        assert_eq!(
            Some(Key::from(ali)),
            implementer
        )
    }


    #[should_panic(expected = "ApiError::User(1001) [66537]")]
    #[test]
    fn should_not_register_an_implementer_with_same_caller_address() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;

        let tag_sender = HASH_ERC1820_SENDER.to_string();

        fixture.set_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap()),
            Key::from(ali),
            Sender(ali)
        );

        let implementer = fixture.get_interface_implementer(
            Key::from(ali),
            Bytes::from(tag_sender.clone().to_bytes().unwrap())
        );

        assert_eq!(
            Some(Key::from(ali)),
            implementer
        )
    }

    #[test]
    fn should_set_manager() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;

        fixture.set_manager(
            Key::from(ali),
            Key::from(ali),
            Sender(ali)
        );

        let manager = fixture.get_manager(Key::from(ali));

        assert_eq!(
            Some(Key::from(ali)),
            manager
        )
    }

    #[should_panic(expected = "ApiError::User(999) [66535]")]
    #[test]
    fn should_not_set_manager() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;
        let joe = fixture.joe;

        fixture.set_manager(
            Key::from(joe),
            Key::from(ali),
            Sender(ali)
        );

        let manager = fixture.get_manager(Key::from(ali));

        assert_eq!(
            Some(Key::from(ali)),
            manager
        )
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
