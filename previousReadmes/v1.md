# Welcome
Welcome to this tutorial for learning how to make your own NEAR contract using rust. 

For this tutorial we will be using rustc 1.80.1 (3f5fd8dd4 2024-08-06)

This tutorial will asume you know some things and have installed some others.

## Installing necessary packages
### Rust
For installing rust, follow the instructions in this [link](https://www.rust-lang.org/tools/install)

This will install the most recent version of rust. Currently, NEAR is having some issues with the most recent version of rust. For this, we recommend you to downgrade to the version for this tutorial the following way

```
rustup install 1.80.1
rustup default 1.80.1
```

You can check it running `rustc --version`

### Other packages
You can follow the instructions in the [NEAR documentation](https://docs.near.org/smart-contracts/quickstart). Just in case the contents change, we will paste the most relevant commands

```
# Contracts will be compiled to wasm, so we need to add the wasm target
rustup target add wasm32-unknown-unknown

# Install NEAR CLI-RS to deploy and interact with the contract
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/near-cli-rs/releases/latest/download/near-cli-rs-installer.sh | sh

# Install cargo near to help building the contract
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/near/cargo-near/releases/latest/download/cargo-near-installer.sh | sh
```

## Initialize your contract
With the following command we will have a basic NEAR contract ready for deployment with basic tests as well. This command will generate a `hello-near` folder.
```
cargo near new hello-near
```

After this, we can check on `src/lib.rs` our contract with a set_greeting and get_greeting function and their corresponding tests.

## Test your contract
### Using rust
Besides the mentioned tests, we have one more test on `tests/test_basics.rs`. If we want to run our rust tests, we can do it by simply running:
```
cargo test
```

After running this command, the code will be compiled into a `target/` folder on the root of the project. Note this folder is included on the `.gitignore` file, so it is not necessary nor convinient to be shared in github, since we can generate it very easily.

There are 2 types of tests on rust:
- Infile tests: tests that are written in a file and are supposed to test the functions defined on that file;
- Integration tests: written typically on a `tests` folder on the root of the project and should tests integrally the whole contract.

## What else is relevant?
We also have the `Cargo.toml` file which is like the `package.json` file for node. We can include some configuration and our dependencies. We can search for dependencies in `https://crates.io/`.