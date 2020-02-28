# CLE Coin

A Substrate-based blockchain node for Clevelanders. This Blockchain is an example and experiment for my presentation at the [Crypto Cleveland meetup](https://www.meetup.com/crypto-cleveland/events/268646500). The chain is Proof of Work based with simple onchain governance. During the meetup we will launch the network, transfer coins, mine coins, and explore the nature of on-chain governance.

## Using the Network

As an end-user of the network, the easiest way to begin is by launching the [hosted user-interface](https://polkadot.js.org/apps?rpc=wss://cleveland.bootnodes.net/node).

## Getting the Node

### Download Binaries

* [Linux](https://cleveland.bootnodes.net/cle-coin)
* Mac - TODO
* [Windows](https://cleveland.bootnodes.net/cle-coin.exe) (experimental)

### Build it Yourself
You can also build the node yourself. This is the most well-trodden path, but has some prerequisites and takes some disk space.

```bash
# Install Rust
curl https://sh.rustup.rs -sSf | sh

# Initialize rust toolchain
./scripts/init.sh

# Build release node for your platform
cargo build --release
```

## Running a Node

Once you have you node, you can join the live network. Because building to wasm is not yet deterministic, the most reliable way to ensure you sync the correct chain is to first download this [Chain Specification File](./mainnet-spec.json). Here are some [docs](https://substrate.dev/docs/en/development/deployment/chain-spec) about how Substrate uses chain spec files.

As a full node:
`./cle-coin --chain=mainnet-spec.json --name YOUR-NODE-NAME`

As a mining node:
`./cle-coin --chain=mainnet-spec.json --name YOUR-NODE-NAME --validator`
There are not yet block rewards issued to miners. See for [issue #2](https://github.com/JoshOrndorff/cle-coin/issues/2) for details.

### The UI

Once you have your own node running, you can connect the user interface to your own node rather than the fairly centralized bootnode. On the UI Setting tab, choose the node you wish to connect to.

## Longevity

There are no guarantees that this network will live much beyond the meetup. Although that really is up to the participants. Let's go Cleveland!
