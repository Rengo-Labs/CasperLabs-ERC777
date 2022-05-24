casper-client put-deploy -n http://16.162.124.124:7777 \
--chain-name casper-test --secret-key /Users/raulvillca/n/TestUser_key.pem \
--session-arg "name:string='ERC777 Modificado'" \
--session-arg "symbol:string='my_erc777'" \
--session-arg "total_supply:u256='100000'" \
--session-arg "granularity:u256='1'" \
--session-path /Users/raulvillca/n/erc20/target/wasm32-unknown-unknown/release/erc20_token.wasm \
--payment-amount 40000000000

casper-client put-deploy \
--session-name erc20_token_contract \
--session-entry-point send \
--session-arg "recipient:Key='account-hash-675e10b7e268c61db84dbc4ddd0dc6c92230b6898e8d2109a3fdc49de8fedab4'" \
--session-arg "amount:u256='150000000000'" \
--session-arg "data:string=''" \
--payment-amount 1000000000 \
--chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key /Users/raulvillca/n/TestUser_key.pem

# Transfer
casper-client put-deploy \
--session-name erc20_token_contract \
--session-entry-point transfer \
--session-arg "recipient:Key='account-hash-675e10b7e268c61db84dbc4ddd0dc6c92230b6898e8d2109a3fdc49de8fedab4'" \
--session-arg "amount:u256='15000'" \
--payment-amount 1000000000 \
--chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key /Users/raulvillca/n/TestUser_key.pem

casper-client put-deploy \
--session-name erc20_token_contract \
--session-entry-point send \
--session-arg "recipient:Key='account-hash-6bc21d981ab85c81b765879b74c70832551e1c2b83f149f6e8ac7fc54a74df40'" \
--session-arg "amount:u256='2500'" \
--session-arg "data:string=''" \
--payment-amount 1000000000 \
--chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key /Users/raulvillca/n/TestUser_key.pem

# query get-state-root-hash
casper-client get-state-root-hash --node-address http://16.162.124.124:7777

casper-client query-global-state \
--id 7243122153472239535 \
--node-address http://16.162.124.124:7777 \
--key 013b8bd1f2bfc7241d69e7cba488bfca52e29d2836e1a8e62035719aee0b81f5f1 \
--state-root-hash f101c972f93eae2783a00853b72287e38ff5671673e2653335bf48a732267ffa \
-q "erc20_token_contract"


casper-client query-global-state \
--id 7243122153472239535 \
--node-address http://16.162.124.124:7777 \
--key uref-ca26202aa94e79fc2ffb69943bac9b1cb6a007f42562f13177a80fd5e32da5bf-007 \
--block-hash dd19f633e7938e51abce87a9acdbb5c3243c1c1f1de741270443b1ad0b8d0ed5 \
-q "erc20_token_contract"

casper-client get-deploy \
--node-address http://16.162.124.124:7777 dd19f633e7938e51abce87a9acdbb5c3243c1c1f1de741270443b1ad0b8d0ed5

casper-client get-state-root-hash \
--id 1 \
--node-address http://16.162.124.124:7777/rpc

casper-client total_supply \
--id 7243122153472239535 \
--node-address http://16.162.124.124:7777 \
--state-root-hash dd19f633e7938e51abce87a9acdbb5c3243c1c1f1de741270443b1ad0b8d0ed5 \
--purse-uref uref-ca26202aa94e79fc2ffb69943bac9b1cb6a007f42562f13177a80fd5e32da5bf-007


casper-client query-global-state -n http://16.162.124.124:7777 \
--id 7243122153472239535 \
--key uref-ca26202aa94e79fc2ffb69943bac9b1cb6a007f42562f13177a80fd5e32da5bf-007 \
--state-root-hash dd19f633e7938e51abce87a9acdbb5c3243c1c1f1de741270443b1ad0b8d0ed5


casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer" \
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b" \
--session-arg "amount:u256='50'" \
--payment-amount "10000000000"

casper-client put-deploy \
--session-name erc20_token_contract \
--session-entry-point decimals \
--payment-amount 1000000000 \
--chain-name casper-test \
--node-address http://16.162.124.124:7777 \
--secret-key /Users/raulvillca/n/TestUser_key.pem

# Casper ERC-20

A library for developing ERC-20 tokens for the Casper Network.

The main functionality is provided via the ERC-20 struct, and is intended to be consumed by a smart contract written to be deployed on the Casper Network.

## Usage

To create an example ERC-20 contract which uses this library, use the `cargo-casper` tool:

```
cargo install cargo-casper
cargo casper --erc20 <PATH TO NEW PROJECT>
```

This command will generate a new project structure with an example token contract based on an [example project](example/erc20-token/src/main).

## Development

Make sure the `wasm32-unknown-unknown` Rust target is installed.

```
make prepare
```

## Build Smart Contracts
To build the example ERC-20 contract and supporting test contracts:

```
make build-contracts
```

## Test

```
make test
```

## JavaScript Client SDK

A [JavaScript client SDK](https://github.com/casper-network/casper-contracts-js-clients/tree/master/packages/erc20-client) can be used to interact with the ERC-20 contract. 


## Documentation

For more information, visit the below guides:
-  [ERC-20 Tutorial](TUTORIAL.md) - An illustrated guide on how to implement, deploy, and test an ERC-20 contract. 
-  [ERC-20 How-To Guide](https://casper.network/docs/workflow/erc-20-sample-guide) - An example-driven guide on how to setup, query, transfer, approve, and check the balance of an ERC-20 contract.


