# Casper ERC-1820

A library for developing ERC-1820 registry to notify or revert transfers on the Casper Network.

The main functionality is registering another contract to perform operations on behalf of token owner.
On this way, it offers a safe way to perform operations on the Casper Network.

This contract contains an interface registry for managers and implementers.
What's more, each account can implement more than a single interface.

### Entry points

The actual entry points are:
- *set_interface_implementer* : This entry point is used for registering sender and receiver contracts
- *get_interface_implementer* : This entry point returns the implementer for the caller address. Otherwise, a default account is returned.
- *set_manager*
- *get_manager*

## SETTING UP THE PROJECT
To start to develop with this library, you need to follow these steps to avoid errors:

- First, to add target `wasm32-unknown-unknown`.

```
make prepare
```

- Second, to build the example ERC-1820 contract, import libs and supporting test contracts:

```
make build-contracts
```

- Third, to run test
```
make test
```

## DEPLOYMENT
First of all, you must build the **target** package to be able to deploy the contract.

After that, you can deploy putting this command on terminal.

```
casper-client put-deploy \
--node-address http://16.162.124.124:7777 \
--chain-name casper-test \
--secret-key ~/TestUser_key.pem \
--session-path ~/casp-777/target/wasm32-unknown-unknown/release/erc1820_registry.wasm \
--payment-amount 50000000000
```