#[cfg(test)]
mod test_erc1820;

#[cfg(test)]
mod tests {
    use casper_contract::unwrap_or_revert::UnwrapOrRevert;
    use casper_types::{Key};
    use casper_types::bytesrepr::{Bytes, ToBytes};

    extern crate base64;
    use crate::test_erc1820::{Sender, TestERC1820};

    pub const HASH_ERC1820_SENDER: &str = "ERC777TokensSender";
    pub const HASH_ERC1820_RECIPIENT: &str = "ERC777TokensRecipient";

    #[test]
    fn should_register_a_recipient() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;
        let tag_sender = HASH_ERC1820_RECIPIENT.to_string();

        fixture.set_interface_implementer(
            Key::from(ali),
            tag_sender.clone(),
            Key::from(ali),
            Sender(ali)
        );

        let implementer = fixture.get_interface_implementer(
            Key::from(ali),
            tag_sender.clone()
        );

        assert_eq!(
            Some(Key::from(ali)),
            implementer
        )
    }

    #[test]
    fn should_register_a_sender() {
        let mut fixture = TestERC1820::install_contract();
        let ali = fixture.ali;
        let tag_sender = HASH_ERC1820_SENDER.to_string();

        fixture.set_interface_implementer(
            Key::from(ali),
            tag_sender.clone(),
            Key::from(ali),
            Sender(ali)
        );

        let implementer = fixture.get_interface_implementer(
            Key::from(ali),
            tag_sender.clone()
        );

        assert_eq!(
            Some(Key::from(ali)),
            implementer
        )
    }

    #[test]
    fn should_install_and_set_manager() {
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


}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
