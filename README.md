# CLE Coin

A Substrate-based blockchain node for Clevelanders. This Blockchain is an example and experiment for my presentation at the [Crypto Cleveland meetup](https://www.meetup.com/crypto-cleveland/events/268646500). The chain is Proof of Work based with simple onchain governance. During the meetup we will launch the network, transfer coins, mine coins, and explore the nature of on-chain governance.

## Using the Network

TODO

link to UI

## Getting the Node

### Download Binaries

TODO

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

## Joining the Network

Once you have you node, you can join the network.

TODO

This network will probably not continue to live much beyond the meetup. Although that really is up to the participants.
