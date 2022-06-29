# Testing Framework

The testing framework in this tutorial uses the [Casper engine test support](https://crates.io/crates/casper-engine-test-support) crate for testing the contract implementation against the Casper execution environment.

We will review the following three [GitHub testing folders](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/tests/erc777-tests/src/integration_tests.rs), which create a testing framework for the Casper [ERC-777](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/erc777) project:

-   **erc1820-tests** - Links the test framework together and is required by the Rust toolchain
-   **erc20-tests** - Sets up the testing context and creates helper functions used by unit tests
-   **erc777-tests** - Sets up the testing context and creates helper functions used by unit tests
-   **erc777-recipient-tests** - Contains the unit tests
-   **erc777-sender-tests** - Contains the unit tests

The following is an example of a complete test:

```rust

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
    }
```

To run the tests, issue the following command in the project folder, [erc777](https://github.com/Rengo-Labs/CasperLabs-ERC777/erc777):

```bash

    make test

```

The project contains a [Makefile](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/Makefile), which is a custom build script that compiles the contract before running tests in _release_ mode. Then, the script copies the `contract.wasm` file to the [tests/wasm](https://github.com/casper-ecosystem/erc20/tree/master/testing/tests/wasm) directory. In practice, you only need to run the `make test` command during development.

## Configuring the Test Package {#configuring-the-test-package}

In this project, we define a `tests` package using the [tests/Cargo.toml](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/tests/erc777-tests/Cargo.toml) file.

```bash

    [package]
    name = "tests"
    version = "0.1.0"
    ...

    [dependencies]
    casper-types = "1.3.2"
    casper-engine-test-support = "1.3.2"
    casper-execution-engine = "1.3.2"
    once_cell = "1.8.0"

    [lib]
    name = "tests"
    ...
```

## Testing Logic {#testing-logic}

In Github, you will find an [example](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example) containing a Casper ERC-777 [contract](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/implementations/erc777-token/src/main.rs) implementation with the corresponding [tests](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/tests/erc777-tests/src). The tests follow this sequence:

-   [Step 1](#setting-up-the-testing-context) - Specify the starting state of the blockchain.
-   [Step 2](#deploying-the-contract) - Deploy the compiled contract to the blockchain and query it.
-   [Step 3](#invoking-contract-methods) - Create additional deploys for each of the methods in the contract.

The [TestFixture](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/tests/erc777-tests/src/test_fixture.rs) accomplishes these steps by simulating a real-world deployment (storing the contract in the blockchain) and invoking methods in the contract.

### Setting up the testing context {#setting-up-the-testing-context}

The code in the [TestFixture](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/tests/erc777-tests/src/test_fixture.rs) initializes the blockchain's [global state](https://casper.network/docs/glossary/G/#global-state) with all the data and methods a smart contract needs.

Below is a subset of the required constants for this project. For the most up-to-date version of the code, visit [GitHub](https://github.com/Rengo-Labs/CasperLabs-ERC777).

```rust

    // File https://github.com/casper-ecosystem/erc20/blob/master/example/erc20-tests/src/test_fixture.rs

    use casper_engine_test_support::{Code, SessionBuilder, TestContext, TestContextBuilder};
    use casper_erc20::constants as consts;
    use casper_types::{
        account::AccountHash,
        bytesrepr::{FromBytes, ToBytes},
        runtime_args, AsymmetricType, CLTyped, ContractHash, Key, PublicKey, RuntimeArgs, U256, U512,
    };

    const CONTRACT_ERC20_TOKEN: &str = "erc777_token.wasm";
    const CONTRACT_KEY_NAME: &str = "erc777_token_contract";

    fn blake2b256(item_key_string: &[u8]) -> Box<[u8]> {...}

    #[derive(Clone, Copy)]
    pub struct Sender(pub AccountHash);
    ...

```

### Deploying the contract {#deploying-the-contract}

The next step is to define a struct that has its own virtual machine (VM) instance and implements the ERC-777 methods. This struct holds a `TestContext` of its own. The _contract_hash_ and the _session_code_ won’t change after the contract is deployed, so it is good to keep them handy.

This code snippet builds the context and includes the compiled contract _.wasm_ binary being tested. The `TestFixture` struct creates a new instance of the `CONTRACT_ERC20_TOKEN` with the following accounts:

-   `ali` - account used to deploy the contract
-   `bob`
-   `joe`

**Note**: 

These accounts have a positive initial balance.

When the contract is deployed, it receives a default **contract_hash**


The full and most recent code implementation is available on [GitHub](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/tests/erc777-tests/src/test_fixture.rs).

```rust

    pub struct TestFixture {
        context: TestContext,
        pub ali: AccountHash,
        pub bob: AccountHash,
        pub joe: AccountHash,
    }

    impl TestFixture {
        pub const TOKEN_NAME: &'static str = "Test ERC777";
        pub const TOKEN_SYMBOL: &'static str = "TERC";
        const TOKEN_TOTAL_SUPPLY_AS_U64: u64 = 1000;
        const TOKEN_GRANULARITY_AS_U64: u64 = 1;

        pub fn token_granularity() -> U256 {
            Self::TOKEN_GRANULARITY_AS_U64.into()
        }
        
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
                consts::GRANULARITY_RUNTIME_ARG_NAME => TestFixture::token_granularity(),
                consts::TOTAL_SUPPLY_RUNTIME_ARG_NAME => TestFixture::token_total_supply(),
                consts::HASH_ERC1820_RUNTIME_ARG_NAME => ContractHash::default()
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
```

### Querying the network {#querying-the-network}

The previous step has simulated a real deploy on the network. The next code snippet describes how to query the network to find the _contract hash_.

Contracts are deployed under the context of an account. Since we created the deploy under the context of `self.ali`, this is what we will query next. The `query_contract` function uses `query` to lookup named keys. It will be used to implement the `balance_of`, `total_supply` and `allowance` checks.

```rust

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
```

**Helper Functions**

The following helper functions are defined to query the named keys in the contract.

The `token_name` function returns the name of the token:

```rust

    pub fn token_name(&self) -> String {
        self.query_contract(consts::NAME_RUNTIME_ARG_NAME).unwrap()
    }
```

The `token_symbol` function returns the token symbol:

```rust

    pub fn token_symbol(&self) -> String {
        self.query_contract(consts::SYMBOL_RUNTIME_ARG_NAME)
            .unwrap()
    }
```

The `token_decimals` function returns the number of decimal places for the token:

```rust

    pub fn token_decimals(&self) -> u8 {
        self.query_contract(consts::DECIMALS_RUNTIME_ARG_NAME)
            .unwrap()
    }
```

### Invoking contract methods {#invoking-contract-methods}

The following code snippet describes a generic way to call a specific entry point in the contract.

```rust

    fn call(&mut self, sender: Sender, method: &str, args: RuntimeArgs) {
        let Sender(address) = sender;
        let code = Code::Hash(self.contract_hash().value(), method.to_string());
        let session = SessionBuilder::new(code, args)
            .with_address(address)
            .with_authorization_keys(&[address])
            .build();
        self.context.run(session);
    }
```

The next code sample shows how to invoke one of the methods in the contract. The rest of the methods are available on [GitHub](https://github.com/Rengo-Labs/CasperLabs-ERC777/tree/main/example/tests/erc777-tests/src/test_fixture.rs).

```rust

    pub fn balance_of(&self, account: Key) -> Option<U256> {
        let item_key = base64::encode(&account.to_bytes().unwrap());

        let key = Key::Hash(self.contract_hash().value());
        let value = self
            .context
            .query_dictionary_item(key, Some(consts::BALANCES_KEY_NAME.to_string()), item_key)
            .ok()?;

        Some(value.into_t::<U256>().unwrap())
    }
```

## Creating Unit Tests {#creating-unit-tests}

Now that we have a testing context, we can use it to create unit tests in a file called [integration_tests.rs](example/tests/erc777-tests/src/integration_tests.rs). The unit tests verify the contract code by invoking the functions defined in the [test_fixture.rs](example/tests/erc777-tests/src/test_fixture.rs) file.

The example below shows one of the example tests. Visit [GitHub](https://github.com/casper-ecosystem/erc20/blob/master/example/erc20-tests/src/integration_tests.rs) to find all the available tests.

```rust

    use casper_types::{Key, U256};

    use crate::test_fixture::{Sender, TestFixture};

    #[test]
    fn should_install() {
        let fixture = TestFixture::install_contract();
        assert_eq!(fixture.token_name(), TestFixture::TOKEN_NAME);
        assert_eq!(fixture.token_symbol(), TestFixture::TOKEN_SYMBOL);
        assert_eq!(fixture.token_decimals(), TestFixture::TOKEN_DECIMALS);
        assert_eq!(
            fixture.balance_of(Key::from(fixture.ali)),
            Some(TestFixture::token_total_supply())
        );
    }
```

## Running the Tests {#running-the-tests}

To run the tests, navigate to the parent [erc777 directory](https://github.com/Rengo-Labs/CasperLabs-ERC777) and run the command:

```bash

    make test

```

This example uses `bash`. If you are using a Rust IDE, you need to configure it to run the tests.
