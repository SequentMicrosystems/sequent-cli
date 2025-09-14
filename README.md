# Sequent CLI

# README.md generated with AI, so don't make too much use of it

A unified command-line interface for Sequent Microsystems Raspberry Pi HATs.

> **Note:** 🚧🛠️🔜This project is a work in progress. Development has just started, and features may be incomplete or subject to change.

> **Note:** Many features as just as a presentation, not everything is functional

## Overview

Sequent CLI is a modern, universal tool for interfacing with all Sequent Microsystems Raspberry Pi HATs through a consistent, intuitive command-line interface.
Written in Rust, this project consolidates functionality that was previously spread across multiple separate command-line tools and also adds more features.

### Supported HATs

- Multi-IO (most feature rich)
- Industrial Automation (just the structure of the cli)
- Eight Inputs (just the structure of the cli)
- More coming soon!

## Features

- **Unified Interface:** Control any supported HAT using the same command structure
- **Modern CLI Design:** Intuitive subcommands with helpful documentation
- **Discoverable:** Built-in help system to explore available commands
- **Alot more extensibility** ...

## Installation

### Prerequisites: Installing Rust on Raspberry Pi

```bash
# Update your system
sudo apt update
sudo apt upgrade -y

# Install dependencies
sudo apt install -y git build-essential

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow the prompts, selecting the default installation (option 1)
# Then source the environment to use Rust right away
source $HOME/.cargo/env
```

### Building and installing from Source

```bash
# Clone the repository
git clone https://github.com/sequentmicrosystems/sequent-cli.git
cd sequent-cli

# one-step install to ~/.cargo/bin
cargo install --path .

# or build & copy manually
cargo build --release
sudo install -Dm755 target/release/sequent-cli /usr/local/bin/sequent-cli
```


### Installing from crates.io

There will be a better way to download the prebuilt binary directly,
so it is alot faster than compiling

```bash
# Not working yet
cargo install cargo-binstall
cargo binstall sequent-cli # much faster than building from source
```

### Running the CLI for development purposes

```bash
cargo run

# Add -- so you can add flags freely
# Ex:
cargo run -- multiio 0 opto cfg-edge --rising
```

## Basic Usage

After installing systemwide, the cli can be used as simply as follows:

```bash
# Get general help
sequent-cli --help

# Get help for a specific HAT
sequent-cli multiio --help

# Control a relay on stack level 0 (default)
sequent-cli multiio relay set 1 on

# Control a relay on stack level 2
sequent-cli multiio 2 relay set 1 off

# Get information about a device
sequent-cli industrial info
```

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
