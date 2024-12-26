# Substrate

[![GitHub license](https://img.shields.io/badge/license-GPL3%2FApache2-blue)](#LICENSE)
[![GitLab Status](https://gitlab.parity.io/parity/mirrors/polkadot-sdk/badges/master/pipeline.svg)](https://gitlab.parity.io/parity/mirrors/polkadot-sdk/-/pipelines)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](docs/contributor/CONTRIBUTING.md)
[![Stack Exchange](https://img.shields.io/badge/Substrate-Community%20&%20Support-24CC85?logo=stackexchange)](https://substrate.stackexchange.com/)

<p align="center">
  <img src="../substrate/docs/media/sub.gif">
</p>

Substrate is a next-generation framework for blockchain innovation 🚀.

## Getting Started

Head to [`docs.substrate.io`](https://docs.substrate.io) and follow the [installation](https://docs.substrate.io/install/) instructions. Then try out one of the [tutorials](https://docs.substrate.io/tutorials/). Refer to the [Docker instructions](./docker/README.md) to quickly run Substrate, Substrate Node Template, Subkey, or to build a chain spec.

## Community & Support

Join the highly active and supportive community on the [Substrate Stack Exchange](https://substrate.stackexchange.com/) to ask questions about use and problems you run into using this software. Please do report bugs and [issues here](https://github.com/paritytech/polkadot-sdk/issues) for anything you suspect requires action in the source.

## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/contributor/CONTRIBUTING.md`](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CONTRIBUTING.md). In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/CODE_OF_CONDUCT.md).

## Security

The security policy and procedures can be found in [`docs/contributor/SECURITY.md`](https://github.com/paritytech/polkadot-sdk/blob/master/docs/contributor/SECURITY.md).

## License

- Substrate Primitives (`sp-*`), Frame (`frame-*`) and the pallets (`pallets-*`), binaries (`/bin`) and all other utilities are licensed under [Apache 2.0](LICENSE-APACHE2).
- Substrate Client (`/client/*` / `sc-*`) is licensed under [GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

The reason for the split-licensing is to ensure that for the vast majority of teams using Substrate to create feature-chains, then all changes can be made entirely in Apache2-licensed code, allowing teams full freedom over what and how they release and giving licensing clarity to commercial teams.

In the interests of the community, we require any deeper improvements made to Substrate's core logic (e.g. Substrate's internal consensus, crypto or database code) to be contributed back so everyone can benefit.

# Substrate Node Template

## Installation Guide

### Prerequisites

Ensure you have the following installed on your system:
- [Rust](https://www.rust-lang.org/tools/install)
- Cargo (comes with Rust installation)
- [Git](https://git-scm.com/)

### Cloning the Repository

First, clone the repository to your local machine:

```sh
git clone https://github.com/nhitranbtc/substrate-node-template.git
cd substrate-node-template
```

### Building the Project

To build the project, run the following commands:

```sh
# Check the project
make check

# Build the project
make build

# Build the project in release mode
make build-release
```

### Running the Project

To run the project, use the following commands:

```sh
# Run the project in development mode
make run

# Run the project in release mode
make release
```
