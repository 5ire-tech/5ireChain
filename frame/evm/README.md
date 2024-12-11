# EVM pallet

The EVM pallet allows unmodified EVM code to be executed in a Substrate-based blockchain.

## EVM engine

The EVM pallet uses [`SputnikVM`](https://github.com/rust-blockchain/evm) as the underlying EVM engine.

## Execution lifecycle

There are a separate set of accounts managed by the EVM pallet.
Substrate based accounts can call the EVM pallet to deposit or withdraw balance from the Substrate base-currency into a different balance managed and used by the EVM pallet.
Once a user has populated their balance, they can create and call smart contracts using this pallet.

Substrate accounts and EVM external accounts are mapped via customizable conversion functions.

## EVM pallet vs Ethereum network

The EVM pallet should be able to produce nearly identical results compared to the Ethereum mainnet, including gas cost and balance changes.

Observable differences include:

* The available length of block hashes may not be 256 depending on the configuration of the System pallet in the Substrate runtime.
* Difficulty and coinbase, which do not make sense in this pallet and is currently hard coded to zero.

We currently do not aim to make unobservable behaviors, such as state root, to be the same. We also don't aim to follow the exact same transaction / receipt format.
However, given one Ethereum transaction and one Substrate account's private key, one should be able to convert any Ethereum transaction into a transaction compatible with this pallet.

The gas configurations are configurable. Right now, a pre-defined Shanghai hard fork configuration option is provided.

License: Apache-2.0
