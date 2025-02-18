<div align="center">

# Tanssi Development Template 🚀

<img height="180px" alt="Polkadot SDK Logo" src="https://static.wixstatic.com/media/c5e759_e4afdf041c8e4c6daf34c533d795b4a1~mv2.png"/>
</div>

## Table of Contents 📚

- [Tanssi Development Template 🚀](#tanssi-development-template-)
  - [Table of Contents 📚](#table-of-contents-)
  - [1. Project Roadmap](#1-project-roadmap)
    - [1.1 Project Structure](#11-project-structure)
  - [2. Setup](#2-setup)
    - [2.1 Ubuntu/Debian](#21-ubuntudebian)
    - [2.2 MacOS](#22-macos)
    - [2.3 Rust Toolchain](#23-rust-toolchain)
  - [3. Create new Template](#3-create-new-template)
    - [3.1 Copy and Rename Container Chain `Node` and `Runtime`](#31-copy-and-rename-container-chain-node-and-runtime)
    - [3.2 Rename `Cargo.toml` of `Node` and `Runtime`](#32-rename-cargotoml-of-node-and-runtime)
    - [3.2 Rename `Logs` of `Node`](#32-rename-logs-of-node)
  - [4. Create a new Pallet](#4-create-a-new-pallet)
    - [4.1 Copy from Minimal Pallet Template](#41-copy-from-minimal-pallet-template)
    - [4.2 Implement Counter Pallet](#42-implement-counter-pallet)
      - [4.2.1 Add `Config`](#421-add-config)
      - [4.2.2 Add `Event`](#422-add-event)
      - [4.2.3 Add `Error`](#423-add-error)
      - [4.2.4 Add `Storage`](#424-add-storage)
      - [4.2.5 Add `Call`](#425-add-call)
    - [4.3 Write Tests for Counter Pallet](#43-write-tests-for-counter-pallet)
    - [4.3 Create Benchmarks for Counter Pallet](#43-create-benchmarks-for-counter-pallet)
  - [5. Add Pallet into Runtime](#5-add-pallet-into-runtime)
    - [5.0 Add Pallet in `Cargo.toml`](#50-add-pallet-in-cargotoml)
    - [5.1 Import Pallet](#51-import-pallet)
    - [5.2 Type Pallet](#52-type-pallet)
    - [5.3 Use `parameter_types!` Pallet](#53-use-parameter_types-pallet)
    - [5.4 Add Pallet in `construct_runtime!`](#54-add-pallet-in-construct_runtime)
    - [5.5 Build Runtime](#55-build-runtime)
  - [6. Incorporate Runtime in Node](#6-incorporate-runtime-in-node)
  - [7. Validate new Features in Running Node](#7-validate-new-features-in-running-node)
  - [8. Update pallet](#8-update-pallet)
  - [9. Integrate new Pallet in Runtime](#9-integrate-new-pallet-in-runtime)
  - [9. Upgrade Node Network (new runtime)](#9-upgrade-node-network-new-runtime)
  - [10. Validate new feature](#10-validate-new-feature)

## 1. Project Roadmap

- [ ] setup
- [ ] clone node, runtime and rename
- [ ] create a pallet
- [ ] test pallet
- [ ] benchmark pallet
- [ ] integrate pallet in runtime
- [ ] incorporate runtime in node
- [ ] initiate node
- [ ] validate new features in running node
- [ ] update pallet
- [ ] integrate pallet in runtime
- [ ] upgrade node network (new runtime)
- [ ] validate new feature

### 1.1 Project Structure

```./chains/container-chains/
├── nodes               # Node configuration and CLI
│   ├── ...
└── runtime-templates   # Runtime configuration
    ├── ...
├── ...                 # Other things
└── pallets/            # Custom blockchain logic modules
```

## 2. Setup

### 2.1 Ubuntu/Debian

```bash
sudo apt update
sudo apt install -y git clang curl libssl-dev protobuf-compiler make
```

### 2.2 MacOS

```bash
brew install cmake openssl protobuf
```

### 2.3 Rust Toolchain

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src
```

## 3. Create new Template

### 3.1 Copy and Rename Container Chain `Node` and `Runtime`

### 3.2 Rename `Cargo.toml` of `Node` and `Runtime`

### 3.2 Rename `Logs` of `Node`

## 4. Create a new Pallet

### 4.1 Copy from Minimal Pallet Template

### 4.2 Implement Counter Pallet

#### 4.2.1 Add `Config`

#### 4.2.2 Add `Event`

#### 4.2.3 Add `Error`

#### 4.2.4 Add `Storage`

#### 4.2.5 Add `Call`

### 4.3 Write Tests for Counter Pallet

### 4.3 Create Benchmarks for Counter Pallet

## 5. Add Pallet into Runtime

### 5.0 Add Pallet in `Cargo.toml`

### 5.1 Import Pallet

### 5.2 Type Pallet

### 5.3 Use `parameter_types!` Pallet

### 5.4 Add Pallet in `construct_runtime!`

### 5.5 Build Runtime

## 6. Incorporate Runtime in Node

## 7. Validate new Features in Running Node

## 8. Update pallet

## 9. Integrate new Pallet in Runtime

## 9. Upgrade Node Network (new runtime)

## 10. Validate new feature
