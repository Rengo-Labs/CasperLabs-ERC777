# casper-erc20

A library for developing ERC20 tokens for the Casper network.

To create an example ERC20 contract which uses this library, use the cargo-casper tool:

This contract was modified for working as a ERC777.
it implements new entry points for making transfer operations sure and those tokens can be reverted:

- send
- operator_send
- burn
- operator_burn
- default_operators
- authorize
- revoke
- is_operator_for