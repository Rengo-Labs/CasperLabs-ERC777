use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_erc20::{Address, constants as consts};
use casper_types::{
    account::AccountHash,
    bytesrepr::{FromBytes, ToBytes, Bytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512,
};

const CONTRACT_ERC20_TOKEN: &str = "erc20_token.wasm";
const CONTRACT_KEY_NAME: &str = "erc20_token_contract";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

pub struct TestFixture {
    context: TestContext,
    pub ali: AccountHash,
    pub bob: AccountHash,
    pub joe: AccountHash,
}

impl TestFixture {
    pub const TOKEN_NAME: &'static str = "Test ERC20";
    pub const TOKEN_SYMBOL: &'static str = "TERC";
    const TOKEN_TOTAL_SUPPLY_AS_U64: u64 = 1000;

    pub fn token_total_supply() -> U256 {
        Self::TOKEN_TOTAL_SUPPLY_AS_U64.into()
    }

    pub fn install_contract() -> TestFixture {
        let ali = PublicKey::ed25519_from_bytes([3u8; 32]).unwrap();
        let bob = PublicKey::ed25519_from_bytes([6u8; 32]).unwrap();
        let joe = PublicKey::ed25519_from_bytes([9u8; 32]).unwrap();

        let mut context = TestContextBuilder::new()
            .with_public_key(ali.clone(), U512::from(500_000_000_000_000_000u64))
            .with_public_key(bob.clone(), U512::from(500_000_000_000_000_000u64))
            .build();

        let session_code = Code::from(CONTRACT_ERC20_TOKEN);
        let session_args = runtime_args! {
            consts::NAME_RUNTIME_ARG_NAME => TestFixture::TOKEN_NAME,
            consts::SYMBOL_RUNTIME_ARG_NAME => TestFixture::TOKEN_SYMBOL,
            consts::OPERATORS_RUNTIME_ARG_NAME => Vec::<Address>::new(),
            consts::GRANULARITY_RUNTIME_ARG_NAME => U256::one(),
            consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => TestFixture::token_total_supply()
        };

        let session = SessionBuilder::new(session_code, session_args)
            .with_address(ali.to_account_hash())
            .with_authorization_keys(&[ali.to_account_hash()])
            .build();

        context.run(session);
        TestFixture {
            context,
            ali: ali.to_account_hash(),
            bob: bob.to_account_hash(),
            joe: joe.to_account_hash(),
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

    fn query_contract<T: CLTyped + FromBytes>(&self, name: &str) -> Option<T> {
        match self
            .context
            .query(self.ali, &[CONTRACT_KEY_NAME.to_string(), name.to_string()])
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

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }

    pub fn token_name(&self) -> String {
        self.query_contract(consts::NAME_RUNTIME_ARG_NAME).unwrap()
    }

    pub fn token_symbol(&self) -> String {
        self.query_contract(consts::SYMBOL_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn token_granularity(&self) -> U256 {
        self.query_contract(consts::GRANULARITY_RUNTIME_ARG_NAME)
            .unwrap()
    }

    pub fn balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }

    pub fn operators(&self, owner: Key) -> Option<String>{
        let key_bytes = owner.to_bytes().unwrap();
        let hash = blake2b256(&key_bytes);
        let operators_item_key = hex::encode(&hash);

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(
                key,
                Some(consts::OPERATORS_KEY_NAME.to_string()),
                operators_item_key,
            ).ok()?;
        Some(value.into_t::<String>().unwrap())
    }

    pub fn approve(&mut self, spender: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::APPROVE_ENTRY_POINT_NAME,
            runtime_args! {
                consts::SPENDER_RUNTIME_ARG_NAME => spender,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn transfer_from(&mut self, owner: Key, recipient: Key, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::TRANSFER_FROM_ENTRY_POINT_NAME,
            runtime_args! {
                consts::OWNER_RUNTIME_ARG_NAME => owner,
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount
            },
        );
    }

    pub fn burn(&mut self, amount: U256, sender: Sender) {
        self.call(
            sender,
            consts::BURN_ENTRY_POINT_NAME,
            runtime_args! {
                consts::AMOUNT_RUNTIME_ARG_NAME => amount,
                consts::DATA_RUNTIME_ARG_NAME => Bytes::new()
            },
        );
    }

    pub fn authorize_operator(&mut self, operator: Key, sender: Sender) {
        self.call(
            sender,
            consts::AUTHORIZE_OPERATOR_ENTRY_POINT_NAME,
            runtime_args! {
                consts::OPERATOR_RUNTIME_ARG_NAME => operator
            },
        );
    }

    pub fn revoke_operator(&mut self, operator: Key, sender: Sender) {
        self.call(
            sender,
            consts::REVOKE_OPERATOR_ENTRY_POINT_NAME,
            runtime_args! {
                consts::OPERATOR_RUNTIME_ARG_NAME => operator
            },
        );
    }

    pub fn operator_send(
        &mut self,
        sender: Key,
        recipient: Key,
        amount: U256,
        data: Bytes,
        operator_data: Bytes,
        operator: Sender
    ) {
        self.call(
            operator,
            consts::OPERATOR_SEND_ENTRY_POINT_NAME,
            runtime_args! {
                consts::SENDER_RUNTIME_ARG_NAME => sender,
                consts::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount,
                consts::DATA_RUNTIME_ARG_NAME => data,
                consts::OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
            },
        );
    }

    pub fn operator_burn(
        &mut self,
        account: Key,
        amount: U256,
        data: Bytes,
        operator_data: Bytes,
        operator: Sender
    ) {
        self.call(
            operator,
            consts::OPERATOR_BURN_ENTRY_POINT_NAME,
            runtime_args! {
                consts::ACCOUNT_RUNTIME_ARG_NAME => account,
                consts::AMOUNT_RUNTIME_ARG_NAME => amount,
                consts::DATA_RUNTIME_ARG_NAME => data,
                consts::OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
            },
        );
    }
}
