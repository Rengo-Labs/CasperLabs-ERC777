use once_cell::sync::Lazy;
use blake2::{
    digest::{Update, VariableOutput},
    VarBlake2b,
};
use casper_engine_test_support::{
    internal::{
        DeployItemBuilder, ExecuteRequestBuilder, InMemoryWasmTestBuilder,
        DEFAULT_RUN_GENESIS_REQUEST, ARG_AMOUNT, DEFAULT_PAYMENT,
        DEFAULT_GENESIS_CONFIG, DEFAULT_GENESIS_CONFIG_HASH
    },
    DEFAULT_ACCOUNT_ADDR, MINIMUM_ACCOUNT_CREATION_BALANCE, DEFAULT_ACCOUNT_INITIAL_BALANCE
};

use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
use casper_types::{
    account::AccountHash, system::mint,
    bytesrepr::{FromBytes, ToBytes, Bytes},
    runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512, SecretKey
};

const ERC1820_CONTRACT_WASM: &str = "erc1820_registry.wasm";
const ERC777_CONTRACT_WASM: &str = "erc777_token.wasm";

fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {
    let mut hasher = VarBlake2b::new(32).unwrap();
    hasher.update(item_key_string);
    hasher.finalize_boxed()
}

#[derive(Clone, Copy)]
pub struct Sender(pub AccountHash);

static ACCOUNT_1_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes(&[221u8; 32]).unwrap());
static ACCOUNT_1_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_1_SECRET_KEY));
static ACCOUNT_1_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_1_PUBLIC_KEY.to_account_hash());

static ACCOUNT_2_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes(&[220u8; 32]).unwrap());
static ACCOUNT_2_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_2_SECRET_KEY));
static ACCOUNT_2_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_2_PUBLIC_KEY.to_account_hash());

static ACCOUNT_3_SECRET_KEY: Lazy<SecretKey> =
    Lazy::new(|| SecretKey::secp256k1_from_bytes(&[222u8; 32]).unwrap());
static ACCOUNT_3_PUBLIC_KEY: Lazy<PublicKey> =
    Lazy::new(|| PublicKey::from(&*ACCOUNT_3_SECRET_KEY));
static ACCOUNT_3_ADDR: Lazy<AccountHash> = Lazy::new(|| ACCOUNT_3_PUBLIC_KEY.to_account_hash());

pub const TOKEN_NAME: &'static str = "Test ERC777";
pub const TOKEN_SYMBOL: &'static str = "TERC777";

pub struct MultipleFixture {
    pub owner: AccountHash,
    pub recipient: AccountHash,
    pub erc1820: ContractHash,
    pub erc777: ContractHash
}

impl MultipleFixture {

    fn new_account(builder: &mut InMemoryWasmTestBuilder, account: AccountHash) -> AccountHash {


        let id: Option<u64> = None;
        let transfer_1_args = runtime_args! {
            mint::ARG_TARGET => account,
            mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
            mint::ARG_ID => id,
        };

        let transfer_request =
            ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();

        builder.exec(transfer_request).expect_success().commit();

        account
    }

    pub fn init_test_builder() -> InMemoryWasmTestBuilder {
        let mut test_builder = InMemoryWasmTestBuilder::default();
        test_builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        test_builder
    }

    pub fn request_contract(
        test_builder: &mut InMemoryWasmTestBuilder,
        account_address: AccountHash,
        contract_name: &str,
        params: RuntimeArgs
    ) {
        let exec_request = {
            ExecuteRequestBuilder::standard(
                //*DEFAULT_ACCOUNT_ADDR,
                account_address,
                contract_name,
                params,
            )
                .build()
        };

        test_builder.exec(exec_request).expect_success().commit();
    }

    pub fn get_package_hash(test_builder: &mut InMemoryWasmTestBuilder, account_address: AccountHash, contract_name: &str) -> Key {
        let account = test_builder
            .query(None, Key::Account(account_address), &[])
            .expect("should query account")
            .as_account()
            .cloned()
            .expect("should be account");

        *(account
            .named_keys()
            .get(contract_name)
            .expect("should have test_call_package hash"))
    }

    pub fn erc1820Params() -> RuntimeArgs {
        runtime_args! {}
    }

    pub fn erc777Params() -> RuntimeArgs {
        runtime_args! {
            casper_erc20::constants::NAME_KEY_NAME => TOKEN_NAME.to_string(),
            casper_erc20::constants::SYMBOL_KEY_NAME => TOKEN_SYMBOL.to_string(),
            casper_erc20::constants::GRANULARITY_KEY_NAME => U256::one(),
            casper_erc20::constants::TOTAL_SUPPLY_RUNTIME_ARG_NAME => U256::from(10000)
        }
    }

    /// contracts installation
    pub fn setup() -> (InMemoryWasmTestBuilder, MultipleFixture) {

        let mut in_memory = MultipleFixture::init_test_builder();
        println!("{}", *ACCOUNT_1_ADDR);
        println!("{}", *ACCOUNT_2_ADDR);

        let owner = MultipleFixture::new_account(&mut in_memory, *ACCOUNT_1_ADDR);
        let recipient = MultipleFixture::new_account(&mut in_memory, *ACCOUNT_2_ADDR);
        let install_request_1 = ExecuteRequestBuilder::standard(
            owner,
            ERC1820_CONTRACT_WASM,
            MultipleFixture::erc1820Params()
        )
            .build();
        let install_request_2 = ExecuteRequestBuilder::standard(
            owner,
            ERC777_CONTRACT_WASM,
            MultipleFixture::erc777Params()
        )
            .build();

        in_memory.exec(install_request_1).expect_success().commit();
        in_memory.exec(install_request_2).expect_success().commit();

        // Contract Deployer
        let account = in_memory
            .get_account(owner)
            .expect("should have account");

        let erc1820_registry = account
            .named_keys()
            .get(casper_erc1820::constants::ERC1820_REGISTRY_CONTRACT_NAME)
            .and_then(|key| key.into_hash())
            .map(ContractHash::new)
            .expect("should have contract hash");

        let erc20_token = account
            .named_keys()
            .get(casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME)
            .and_then(|key| key.into_hash())
            .map(ContractHash::new)
            .expect("should have contract hash");

        let test_context = MultipleFixture {
            owner,
            recipient,
            erc1820: erc1820_registry,
            erc777: erc20_token
        };

        (in_memory, test_context)
    }

    //---------------------------- contract calls ------------------
    pub fn set_registry(
        &mut self,
        in_memory: &mut InMemoryWasmTestBuilder,
        contract_registry: String,
        sender: AccountHash
    ) {
        self.call(
            in_memory,
            sender,
            casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME,
            casper_erc20::constants::SET_REGISTRY_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc20::constants::ACCOUNT_RUNTIME_ARG_NAME => contract_registry
            }
        );
    }


    pub fn set_interface(
        &mut self,
        in_memory: &mut InMemoryWasmTestBuilder,
        account: Key,
        data: String,
        implementer: Key,
        sender: AccountHash
    ) {
        self.call(
            in_memory,
            sender,
            casper_erc1820::constants::ERC1820_REGISTRY_CONTRACT_NAME,
            casper_erc1820::constants::SET_INTERFACE_ENTRY_POINT,
            runtime_args! {
                casper_erc1820::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
                casper_erc1820::constants::I_HASH_RUNTIME_ARG_NAME => data,
                casper_erc1820::constants::IMPLEMENTER_RUNTIME_ARG_NAME => implementer
            }
        );
    }

    pub fn get_interface(
        &mut self,
        in_memory: &mut InMemoryWasmTestBuilder,
        account: Key,
        data: String,
        sender: AccountHash
    ) {
        self.call(
            in_memory,
            sender,
            casper_erc1820::constants::ERC1820_REGISTRY_CONTRACT_NAME,
            casper_erc1820::constants::GET_INTERFACE_ENTRY_POINT,
            runtime_args! {
                casper_erc1820::constants::ACCOUNT_RUNTIME_ARG_NAME => account,
                casper_erc1820::constants::I_HASH_RUNTIME_ARG_NAME => data
            }
        );
    }

    pub fn send(
        &mut self,
        in_memory: &mut InMemoryWasmTestBuilder,
        recipient: Key,
        amount: U256,
        data: String,
        sender: AccountHash
    ) {
        self.call(
            in_memory,
            sender,
            casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME,
            casper_erc20::constants::SEND_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc20::constants::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                casper_erc20::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc20::constants::DATA_RUNTIME_ARG_NAME => data
            }
        );
    }

    pub fn operator_send(
        &mut self,
        in_memory: &mut InMemoryWasmTestBuilder,
        account: Key,
        recipient: Key,
        amount: U256,
        data: String,
        operator_data: String,
        sender: AccountHash
    ) {
        self.call(
            in_memory,
            sender,
            casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME,
            casper_erc20::constants::OPERATOR_SEND_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc20::constants::SENDER_RUNTIME_ARG_NAME => account,
                casper_erc20::constants::RECIPIENT_RUNTIME_ARG_NAME => recipient,
                casper_erc20::constants::AMOUNT_RUNTIME_ARG_NAME => amount,
                casper_erc20::constants::DATA_RUNTIME_ARG_NAME => data,
                casper_erc20::constants::OPERATOR_DATA_RUNTIME_ARG_NAME => operator_data
            }
        );
    }

    pub fn authorize_operator(
        &mut self,
        in_memory: &mut InMemoryWasmTestBuilder,
        operator: Key,
        sender: AccountHash
    ) {
        self.call(
            in_memory,
            sender,
            casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME,
            casper_erc20::constants::AUTHORIZE_OPERATOR_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator
            }
        );
    }

    pub fn revoke_operator(
        &mut self,
        in_memory: &mut InMemoryWasmTestBuilder,
        operator: Key,
        sender: AccountHash
    ) {
        self.call(
            in_memory,
            sender,
            casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME,
            casper_erc20::constants::REVOKE_OPERATOR_ENTRY_POINT_NAME,
            runtime_args! {
                casper_erc20::constants::OPERATOR_RUNTIME_ARG_NAME => operator
            }
        );
    }

    //----------------------- Call Contract
    fn call(&mut self, builder: &mut InMemoryWasmTestBuilder, sender: AccountHash, contract_name: &str, entry_point: &str, args: RuntimeArgs) {
        let deploy = DeployItemBuilder::new()
            .with_address(sender)
            .with_stored_session_named_key(
                contract_name,
                entry_point,
                args
            )
            .with_empty_payment_bytes(runtime_args! { ARG_AMOUNT => *DEFAULT_PAYMENT, })
            .with_authorization_keys(&[sender])
            .with_deploy_hash([42; 32])
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(deploy).build();
        builder.exec(execute_request).commit().expect_success();
    }
/*
    pub fn balance_of(
        &self,
        builder: &mut InMemoryWasmTestBuilder,
        account: AccountHash
    ) -> U256 {
        //get balance_uref
        let balance_uref = *builder
            .query(
                None,
                Key::Account(account),
                &[casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME.to_string()],
            )
            .expect("should have validator slots")
            .as_contract()
            .expect("should be contractpackage")
            .clone()
            .take_named_keys()
            .get("balances")
            .unwrap()
            .clone()
            .as_uref()
            .unwrap();

        let dic_item_key = base64::encode(&account.to_bytes().unwrap());

        builder
            .query_dictionary_item(
                None,
                balance_uref,
                &dic_item_key
            )
            .ok()
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap()
        // println!("herherhereh is {}", value);
    }*/

    pub fn balance_of(&self, builder: &mut InMemoryWasmTestBuilder, account_hash: AccountHash) -> Option<U256> {
        let account = Key::Account(account_hash);

        let balance_uref = *builder
            .query(
                None,
                account,
                &[casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME.to_string()],
            )
            .expect("should have validator slots")
            .as_contract()
            .expect("should be contractpackage")
            .clone()
            .take_named_keys()
            .get("balances")
            .unwrap()
            .clone()
            .as_uref()
            .unwrap();

        let dic_item_key = base64::encode(&account.to_bytes().unwrap());

        let valor = builder.query_dictionary_item(
            None,
            balance_uref,
            dic_item_key.as_str()
        )
            .ok()
            .unwrap()
            .as_cl_value()
            .unwrap()
            .clone()
            .into_t()
            .unwrap_or(U256::zero());

        Some(valor)
    }
/*
    pub fn operators(&self, builder: &mut InMemoryWasmTestBuilder, account: AccountHash) -> Option<String>{
        let owner= Key::from(account);
        let key_bytes = owner.to_bytes().unwrap();
        let hash = blake2b256(&key_bytes);
        let operators_item_key = hex::encode(&hash);

        let key = Key::Hash(self.contract_hash(builder, account).value());
        let value = *builder
            .query_dictionary_item(
                key,
                Some(casper_erc20::constants::OPERATORS_KEY_NAME.to_string()),
                &operators_item_key,
            ).ok()?;
        Some(value.into_t::<String>().unwrap())
    }

    fn contract_hash(&self, builder: &mut InMemoryWasmTestBuilder, account: AccountHash) -> ContractHash {
        *builder
            .get_account(account)
            .unwrap()
            .named_keys()
            .get(casper_erc20::constants::ERC20_TOKEN_CONTRACT_NAME)
            .unwrap()
            .normalize()
            .into_hash()
            .unwrap()
            .into()
    }*/
}
