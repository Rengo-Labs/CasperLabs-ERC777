use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash,
    bytesrepr::{ToBytes},
    runtime_args, AsymmetricType, ContractHash, Key, PublicKey, RuntimeArgs, U512,
};
use casper_types::bytesrepr::Bytes;

const CONTRACT_ERC1820_REGISTRY: &str = "erc1820_registry.wasm";
const CONTRACT_KEY_NAME: &str = "erc1820_registry";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestERC1820 {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl TestERC1820 {

    pub fn install_contract() -> TestERC1820 {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(joe.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_ERC1820_REGISTRY);
        let session_args = runtime_args! {};

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        TestERC1820 {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash()
        }
    }

    fn contract_hash(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(CONTRACT_KEY_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn set_interface_implementer(&mut self, account: Key, i_hash: Bytes, implementer: Key, sender: Sender) {
        self.call(
            sender,
            casper_erc1820::constants::SET_INTERFACE_ENTRY_POINT,
            runtime_args! {
                casper_erc1820::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
                casper_erc1820::constants::I_HASH_RUNTIME_ARG_NAME => i_hash,
                casper_erc1820::constants::IMPLEMENTER_RUNTIME_ARG_NAME => implementer
            },
        );
    }

    pub fn set_manager(&mut self, account: Key, new_manager: Key, sender: Sender) {
        self.call(
            sender,
            casper_erc1820::constants::SET_MANAGER_ENTRY_POINT,
            runtime_args! {
                casper_erc1820::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
                casper_erc1820::constants::NEW_MANAGER_RUNTIME_ARG_NAME => new_manager
            },
        );
    }

    pub fn get_interface_implementer(&self, account: Key, tag: Bytes) -> Option<Key> {
        let mut preimage = Vec::new();
        preimage.append(&mut account.to_bytes().unwrap());
        preimage.append(&mut tag.to_vec());
        let key_bytes = blake2b256(&preimage);
        let item_key = hex::encode(&key_bytes);

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(casper_erc1820::constants::IMPLEMENTERS_REGISTRY_KEY_NAME.to_string()),
                item_key
            ).ok()?;

        Some(value.into_t::<Key>().unwrap())
    }

    pub fn get_manager(&self, account: Key) -> Option<Key> {
        let key_bytes = account.to_bytes().unwrap();
        let hash = blake2b256(&key_bytes);
        let manager_item_key = hex::encode(&hash);

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(casper_erc1820::constants::MANAGERS_REGISTRY_KEY_NAME.to_string()),
                manager_item_key,
            ).ok()?;
        Some(value.into_t::<Key>().unwrap())
    }
}
