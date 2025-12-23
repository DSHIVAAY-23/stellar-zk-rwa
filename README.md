# Stellar ZK-RWA Bridge

> **Privacy-Preserving Real World Asset Compliance on Stellar using Soroban & SP1.**

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Status](https://img.shields.io/badge/status-Prototype-orange.svg)
![Stellar](https://img.shields.io/badge/ecosystem-Stellar_Soroban-purple.svg)

## 📖 Introduction

The **Stellar ZK-RWA Bridge** is a next-generation protocol designed to bring institutional-grade Real World Assets (RWAs) to the Stellar network without compromising user privacy.

By integrating **Soroban** smart contracts with **Succinct's SP1 (Zero-Knowledge Virtual Machine)**, we enable users to prove compliance (KYC, AML, Accreditation) off-chain and submit a simplified *Zero-Knowledge Proof* on-chain. This allows the Stellar ledger to verify eligibility and settle assets without ever seeing the user's sensitive private data.

## 🚧 The Problem

Tokenized Real World Assets represent a trillion-dollar opportunity, but they face a critical dilemma:
1.  **Compliance requires Identity**: Issuers must verify "Is this user accredited?" or "Is this user in a sanctioned country?".
2.  **Blockchain transparency leaks Privacy**: Traditional allow-lists require mapping on-chain addresses to real-world identities, creating a "doxxing" risk for high-net-worth individuals and institutions.

## 💡 The Solution: Optimistic Zero-Knowledge Compliance

We solve this using an **Optimistic Gateway** pattern to minimize on-chain costs while preserving privacy:

1.  **Off-Chain Compliance**: Users run a Rust-based compliance program (via SP1) to generate a ZK Proof found on their private data.
2.  **Gateway Certification**: A trusted off-chain Gateway verifies this proof. If valid, it **signs** a certificate (Ed25519 signature) for the user.
3.  **On-Chain Settlement**: The Soroban contract verifies the *Gateway's Signature*. This is extremely cheap (~450k gas) compared to verifying a ZK proof directly (>100M gas).

**Result:** High-performance, low-cost privacy compliance on Stellar Mainnet.

## 🏗 Architecture

Detailed flows can be found in [flow.md](./flow.md).

- **`contracts/zk_verifier`**: Soroban contract that verifies Gateway signatures and mints tokens.
- **`sp1-prover`**: Off-chain agent that acts as the Gateway (Proof Verification + Signing).

## 🚀 Quick Start

### Prerequisites
- Rust & Cargo
- [Soroban CLI](https://soroban.stellar.org/docs/getting-started/setup)
- `wasm32-unknown-unknown` target

### Build & Test
```bash
# Clone the repository
git clone https://github.com/DSHIVAAY-23/stellar-zk-rwa.git
cd stellar-zk-rwa

# Compile the contract
make build

# Run unit tests (includes Gas Benchmark)
make test
```

## map Roadmap

- [x] **Project Scaffolding**: Workspace setup, Soroban configuration.
- [x] **Gas Benchmarking**: Confirmed native ZK costs are too high (>100M).
- [x] **Architecture Refactor**: Switched to Optimistic Gateway pattern (<500k Gas).
- [ ] **SP1 Integration**: Implement actual ZK circuit for compliance.
- [ ] **End-to-End Demo**: CLI flow for keygen, signing, and mocking on-chain minting.
- [ ] **Testnet Deployment**: Launch on Stellar Futurenet.

## 📄 License
MIT
