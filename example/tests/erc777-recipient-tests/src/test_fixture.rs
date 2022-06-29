use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, bytesrepr::{FromBytes, ToBytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U512, U256, HashAddr
};

const ERC1820_CONTRACT_WASM: &str = "erc1820_registry.wasm";
const ERC777_CONTRACT_WASM: &str = "erc777_token.wasm";
const ERC777_RECIPIENT_CONTRACT_WASM: &str = "erc777_recipient.wasm";
const ERC1820_CONTRACT_NAME: &str = "erc1820_registry";
const ERC20_CONTRACT_NAME: &str = "erc777_token_contract";
const ERC777_RECIPIENT_CONTRACT_NAME: &str = "erc777_recipient_contract";


fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestFixture {
    pub context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl TestFixture {
    pub const TOKEN_NAME: &'static str = "Test ERC777";
    pub const TOKEN_SYMBOL: &'static str = "TERC";
    const TOKEN_TOTAL_SUPPLY_AS_U64: u64 = 10000;

    pub fn install_contract() -> TestFixture {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(joe.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        TestFixture {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash()
        }
    }

    pub fn add_erc1820_context(&mut self) {
        println!("Install of erc1820");
        //----- ERC1820 REGISTRY
        let session_code = Code::from(ERC1820_CONTRACT_WASM);
        let session_args = runtime_args! {};

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(self.ali)
            .with_authorization_keys(&[self.ali])
            .build();

        self.context.run(session);
    }

    pub fn add_erc20_context(&mut self) {
        println!("Install of erc777");
        //----- ERC777
        let contract_hash = self.contract_hash_erc1820();
        let session_code = Code::from(ERC777_CONTRACT_WASM);
        let session_args = runtime_args! {
            casper_erc777::constants::NAME_RUNTIME_ARG_NAME => TestFixture::TOKEN_NAME,
            casper_erc777::constants::SYMBOL_RUNTIME_ARG_NAME => TestFixture::TOKEN_SYMBOL,
            casper_erc777::constants::GRANULARITY_RUNTIME_ARG_NAME => U256::one(),
            casper_erc777::constants::TOTAL_SUPPLY_RUNTIME_ARG_NAME => TestFixture::token_total_supply(),
            casper_erc777::constants::HASH_ERC1820_RUNTIME_ARG_NAME => contract_hash
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(self.ali)
            .with_authorization_keys(&[self.ali])
            .build();

        self.context.run(session);
    }

    pub fn add_erc777_recipient_context(&mut self) {
        println!("Install of erc777 recipient");
        //----- ERC777 RECIPIENT
        let erc1820_contract_hash = self.contract_hash_erc1820();
        let erc777_contract_hash = self.contract_hash_erc20();
        let session_code = Code::from(ERC777_RECIPIENT_CONTRACT_WASM);
        let session_args = runtime_args! {
            casper_erc777_recipient::constants::ERC1820_CONTRACT_ARG_NAME => erc1820_contract_hash,
            casper_erc777_recipient::constants::ERC777_CONTRACT_ARG_NAME => erc777_contract_hash
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(self.ali)
            .with_authorization_keys(&[self.ali])
            .build();

        self.context.run(session);
    }

    pub fn contract_hash_erc1820(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(ERC1820_CONTRACT_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn contract_hash_erc20(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(ERC20_CONTRACT_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    pub fn contract_hash_erc777_recipient(&self) -> ContractHash {
        self.context
            .get_account(self.ali)
            .unwrap()
            .named_keys()
            .get(ERC777_RECIPIENT_CONTRACT_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }

    fn call(&mut self, sender: Sender, hash_addr: HashAddr, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(hash_addr, method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    //---- ERC777 Calls
    fn query_contract_erc20<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[ERC20_CONTRACT_NAME.to_string(), name.to_string()])
        {
            Err(_) => None,
            Ok(maybe_value) => {
                let value = maybe_value
                    .into_t()
                    .unwrap_or_else(|_| panic!("{} is not expected type.", name));
                Some(value)
            }
        }
    }

    pub fn token_total_supply() -> U256 {
        Self::TOKEN_TOTAL_SUPPLY_AS_U64.into()
    }

    pub fn token_name(&self) -> String {
        self.query_contract_erc20(casper_erc777::constants::NAME_RUNTIME_ARG_NAME).unwrap()
    }

    pub fn token_symbol(&self) -> String {
        self.query_contract_erc20(casper_erc777::constants::SYMBOL_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn token_granularity(&self) -> U256 {
        self.query_contract_erc20(casper_erc777::constants::GRANULARITY_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn token_erc1820(&self) -> Key {
        self.query_contract_erc20(casper_erc777::constants::REGISTRY_CONTRACT_NAME).unwrap()
    }

    pub fn balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash_erc20().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(casper_erc777::constants::BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn operators(&self, owner: Key) -> Option<String>{
        let key_bytes = owner.to_bytes().unwrap();
        let hash = blake2b256(&key_bytes);
        let operators_item_key = hex::encode(&hash);

        let key = Key::Hash(self.contract_hash_erc20().value());
        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(casper_erc777::constants::OPERATORS_KEY_NAME.to_string()),
                operators_item_key,
            ).ok()?;
        Some(value.into_t::<String>().unwrap())
    }

    pub fn burn(&mut self, amount: U256, data: String, sender: Sender) {
        self.call(
            sender,
            self.contract_hash_erc20().value(),
            casper_erc777::constants::BURN_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc777::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc777::constants::DATA_RUNTIME_ARG_NAME => data
            },
        );
    }

    pub fn authorize_operator(&mut self, operator: Key, sender: Sender) {
        self.call(
            sender,
            self.contract_hash_erc20().value(),
            casper_erc777::constants::AUTHORIZE_OPERATOR_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc777::constants::OPERATOR_RUNTIME_ARG_NAME => operator
            },
        );
    }

    pub fn revoke_operator(&mut self, operator: Key, sender: Sender) {
        self.call(
            sender,
            self.contract_hash_erc20().value(),
            casper_erc777::constants::REVOKE_OPERATOR_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc777::constants::OPERATOR_RUNTIME_ARG_NAME => operator
            },
        );
    }

    pub fn send(
        &mut self,
        recipient: Key,
        amount: U256,
        data: String,
        sender: Sender
    ) {
        self.call(
            sender,
            self.contract_hash_erc20().value(),
            casper_erc777::constants::SEND_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc777::constants::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                casper_erc777::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc777::constants::DATA_RUNTIME_ARG_NAME => data
            },
        );
    }

    pub fn operator_send(
        &mut self,
        sender: Key,
        recipient: Key,
        amount: U256,
        data: String,
        operator_data: String,
        operator: Sender
    ) {
        self.call(
            operator,
            self.contract_hash_erc20().value(),
            casper_erc777::constants::OPERATOR_SEND_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc777::constants::SENDER_RUNTIME_ARG_NAME => sender,
                casper_erc777::constants::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                casper_erc777::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc777::constants::DATA_RUNTIME_ARG_NAME => data,
                casper_erc777::constants::OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
            },
        );
    }

    pub fn operator_burn(
        &mut self,
        account: Key,
        amount: U256,
        data: String,
        operator_data: String,
        operator: Sender
    ) {
        self.call(
            operator,
            self.contract_hash_erc20().value(),
            casper_erc777::constants::OPERATOR_BURN_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc777::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
                casper_erc777::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc777::constants::DATA_RUNTIME_ARG_NAME => data,
                casper_erc777::constants::OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
            },
        );
    }

    pub fn transfer_from_erc777_recipient(
        &mut self,
        from: Key,
        to: Key,
        amount: U256,
        user_data: String,
        operator_data: String,
        sender: Sender
    ) {
        self.call(
            sender,
            self.contract_hash_erc777_recipient().value(),
            casper_erc777_recipient::constants::TRANSFER_ENTRY_POINT,
            runtime_args! {
                casper_erc777_recipient::constants::FROM_RUNTIME_ARG_NAME => from,
                casper_erc777_recipient::constants::TO_RUNTIME_ARG_NAME => to,
                casper_erc777_recipient::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc777_recipient::constants::USER_DATA_RUNTIME_ARG_NAME => user_data,
                casper_erc777_recipient::constants::OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
            },
        );
    }

    pub fn burn_from_erc777_recipient(
        &mut self,
        account: Key,
        amount: U256,
        user_data: String,
        operator_data: String,
        sender: Sender
    ) {
        self.call(
            sender,
            self.contract_hash_erc777_recipient().value(),
            casper_erc777_recipient::constants::BURN_ENTRY_POINT,
            runtime_args! {
                casper_erc777_recipient::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
                casper_erc777_recipient::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc777_recipient::constants::USER_DATA_RUNTIME_ARG_NAME => user_data,
                casper_erc777_recipient::constants::OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
            },
        );
    }
}
