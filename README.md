# Casper ERC-777

A library for developing ERC-777 tokens for the Casper Network.

The main functionality is provided via the ERC-777 struct, and is intended to be consumed by a smart contract written to be deployed on the Casper Network.

This library is based on [ERC-777-OpenZeppelin](https://docs.openzeppelin.com/contracts/2.x/api/token/erc777#IERC777-authorizeOperator-address-)

What's more, this contract relies on [ERC-1820 Tutorial](erc1820/README.md) to register implementers and managers to notify transfers performing for any operator.

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
- [ERC-777 Tutorial](TUTORIAL.md) - An illustrated guide on how to implement, deploy, and test an ERC-20 contract.
- [ERC-777 How To Make Tests](HOW_TO.md) - A guide for making tests of Casper in rust.
- [ERC-1820 Tutorial](erc1820/README.md) - An illustrated guide on how to implement, deploy, and test an ERC-1820 contract.
- [ERC-777 Tutorial](erc777/README.md) - An illustrated guide on how to implement, deploy, and test an ERC-777 contract.
- [ERC-777-RECIPIENT Tutorial](erc777-recipient/README.md) - An illustrated guide on how to implement, deploy, and test an ERC-777-RECIPIENT contract.
- [ERC-777-SENDER Tutorial](erc777-sender/README.md) - An illustrated guide on how to implement, deploy, and test an ERC-777-SENDER contract. 
- [ERC-20 How-To Guide](https://casper.network/docs/workflow/erc-20-sample-guide) - An example-driven guide on how to setup, query, transfer, approve, and check the balance of an ERC-20 contract.