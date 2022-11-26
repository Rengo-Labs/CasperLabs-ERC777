# `Casper ERC-777`

A library for developing ERC-777 tokens for the Casper Network.

The main functionality is provided via the ERC-777 struct, and is intended to be consumed by a smart contract written to be deployed on the Casper Network.

This library is based on [ERC-777-OpenZeppelin](https://docs.openzeppelin.com/contracts/2.x/api/token/erc777#IERC777-authorizeOperator-address-)

What's more, this contract relies on [ERC-1820 Tutorial](erc1820/README.md) to register implementers and managers to notify transfers performing for any operator.

## Development

Make sure the `wasm32-unknown-unknown` Rust target is installed.

```bash
make prepare
```

## Build Smart Contracts
To build the example ERC-777 contract and supporting test contracts:

```bash
make build-contracts
```

## Test

```bash
make test
```

## JavaScript Client SDK

You can use the next clients to deploy and interact with these contracts:
- [ERC1820 Client](https://github.com/Rengo-Labs/CasperLabs-ERC777-client/tree/master/src/clients/erc1820)
- [ERC777 Client](https://github.com/Rengo-Labs/CasperLabs-ERC777-client/tree/master/src/clients/erc777)
- [ERC777 Recipient Client](https://github.com/Rengo-Labs/CasperLabs-ERC777-client/tree/master/src/clients/erc777_recipient)
- [ERC777 Sender Client](https://github.com/Rengo-Labs/CasperLabs-ERC777-client/tree/master/src/clients/erc777_sender)

## Documentation

For more information, visit the below guides:
- [Basic ERC-777 Tutorial](TUTORIAL.md) - An illustrated guide on how to implement, deploy, and test the basic ERC-777 contract with ERC-20(CEP-18) functionality.
- [ERC-777 How To Make Tests](HOW_TO.md) - A guide for making tests of Casper in rust.
- [ERC-1820 Tutorial](erc1820/README.md) - An illustrated guide on how to implement, deploy, and test an ERC-1820 contract.
- [Advanced ERC-777 Tutorial](erc777/README.md) - An illustrated guide on how to implement, deploy, and test all the features of ERC-777 contract with an ERC-1820.
- [ERC-777-RECIPIENT Tutorial](erc777-recipient/README.md) - An illustrated guide on how to implement, deploy, and test an ERC-777-RECIPIENT contract.
- [ERC-777-SENDER Tutorial](erc777-sender/README.md) - An illustrated guide on how to implement, deploy, and test an ERC-777-SENDER contract.
- [ERC-20 How-To Guide](https://casper.network/docs/workflow/erc-20-sample-guide) - An example-driven guide on how to setup, query, transfer, approve, and check the balance of an ERC-20 contract (this covers basic ERC20 with more detail than the tutorials for ERC-777).
