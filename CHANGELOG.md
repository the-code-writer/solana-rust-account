# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

**Note:** Version 0 of Semantic Versioning is handled differently from version 1 and above.
The minor version will be incremented upon a breaking change and the patch version will be incremented for features.

## [Unreleased]

### Features

### Fixes

### Breaking

## [0.1.0] - 2024-04-16

Initial release.

### Pproject file structure

```
smart-contract
├── Cargo.toml
├── src
│   ├── lib.rs
│   ├── models
│   │   ├── transaction.rs
│   │   └── user.rs
│   ├── state
│   │   ├── account_state.rs
│   │   ├── transaction_state.rs
│   │   └── user_state.rs
│   └── transactions
│       ├── create_transaction.rs
│       ├── read_transaction.rs
│       ├── read_all_transactions.rs
│       ├── update_transaction.rs
│       ├── patch_transaction.rs
│       └── delete_transaction.rs
├── tests
│   ├── transaction.rs
│   ├── user.rs
```

### Files

- Cargo.toml: This is the Rust project configuration file.
- src: This is the Rust source code directory.
- lib.rs: This is the main Rust library file.
- models: This directory contains the Rust struct definitions for the transaction and user objects.A
- state: This directory contains the Rust struct definitions for the on-chain state accounts.
- transactions: This directory contains the Rust functions for performing CRUD operations on the transaction object.


### Includes

- src/: Project files
- lang: `anchor-lang` crate providing a Rust eDSL for Solana.
- lang/attribute/access-control: Internal attribute macro for function modifiers.
- lang/attribute/account: Internal attribute macro for defining Anchor accounts.
- lang/attribute/error: Internal attribute macro for defining Anchor program errors.
- lang/attribute/program: Internal attribute macro for defining an Anchor program.
- lang/attribute/state: Internal attribute macro for defining an Anchor program state struct.
- lang/derive/accounts: Internal derive macro for defining deserialized account structs.
- lang/syn: Internal crate for parsing the Anchor eDSL, generating code, and an IDL.
- spl: `anchor-spl` crate providing CPI clients for Anchor programs.
- client: `anchor-client` crate providing Rust clients for Anchor programs.
- ts: `@project-serum/anchor` package for generating TypeScript clients.
- cli: Command line interface for managing Anchor programs.
