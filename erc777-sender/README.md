# casper-erc777 sender

A library for implementing ERC777 Sender for ERC777 contract sending tokens on Casper network.

This contract lets token's holders know about the movements or destruction of their tokens whenever 
they implement this interface. To do this, they must be registered on the ERC1820 registry contract.

Its execution occurs before the token contract's state is updated.
What's more, this contract may revert and prevent the operations from being executed.

### ERC777-SENDER'S ENTRY POINT

- **tokens_to_send** : This entry point is executed by the erc777 when it sends tokens
  to another account.
- **transfer** : This entry point is used for transfer tokens to other account on behalf of the token owner.
For performing this operation, either you need to register the caller account as an operator, or you must use the token owner.
- **burn** : This entry point is used for burn tokens on behalf of the token owner.
For performing this operation, either you need to register the caller account as an operator, or you must use the token owner.

## SETTING UP THE PROJECT
To start to develop with this library, you need to follow these steps to avoid errors:

- First, to add target `wasm32-unknown-unknown`.

```
make prepare
```

- Second, to build the example ERC-777-SENDER contract, import libs and supporting test contracts:

```
make build-contracts
```

- Third, to run test
```
make test
```


## DEPLOYMENT
First of all, you must build the **target** package to be able to deploy the contract:

For install this contract you need to deploy the contract using this parameter:
- **erc1820_contract** : This parameter is a type: contract_hash.
- **erc777_contract** : This parameter is a type: contract_hash.

In this example, to deploy an erc777-recipient contract on casper testnet, you need to run this command on terminal:

```
casper-client put-deploy \
--node-address http://16.162.124.124:7777 \
--chain-name casper-test \
--session-arg "erc1820_contract:key='contract-KEY_HASH'" \
--session-arg "erc777_contract:key='contract-KEY_HASH'" \
--secret-key ~/Test_key.pem \
--session-path ~/casp-777/target/wasm32-unknown-unknown/release/erc777_recipient.wasm \
--payment-amount 20000000000
```
