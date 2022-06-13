# casper-erc777 sender

A library for implementing ERC777 Sender for ERC777 contract sending tokens on Casper network.

This contract lets token's holders know about the movements or destruction of their tokens whenever 
they implement this interface. To do this, they must be registered on the ERC1820 registry contract.

Its execution occurs before the token contract's state is updated.
What's more, this contract may revert and prevent the operations from being executed.

