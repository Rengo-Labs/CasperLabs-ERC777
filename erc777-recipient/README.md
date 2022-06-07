# casper-erc777 recipient

A library for developing ERC777 tokens for the Casper network.

This contract allows any contract that extend of this, is capable of receiving tokens. 

This contract lets token's holders know about the tokens are sent to them whenever
they implement this interface. To do this, they must be registered on the ERC1820 registry contract.

Its execution occurs after the token contract's state is updated.
What's more, this contract may revert and prevent the operations from being executed.


