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

## 💡 The Solution: Zero-Knowledge Compliance

We solve this by decoupling **Verification** from **Settlement**:

1.  **Off-Chain Compliance**: Users run a Rust-based compliance program (via SP1) on their local machine or a private server. This program checks their private documents.
2.  **ZK Proof**: The program generates a cryptographic proof that asserts: *"I certify that I satisfy the compliance requirements for Asset X, and my public key is Y."*
3.  **On-Chain Settlement**: The Soroban contract verifies this proof. If valid, it mints/transfers the RWA tokens to the user's public key.

**Result:** The blockchain knows *that* you are compliant, but not *why* (or *who* you are).

## 🏗 Architecture

Detailed flows can be found in [flow.md](./flow.md).

- **`contracts/zk_verifier`**: A Soroban smart contract acting as the on-chain verifier.
- **`sp1-prover`**: (In Progress) Rust-based SP1 guest program for generating proofs.

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

# Run unit tests
make test
```

## 🗺 Roadmap

- [x] **Project Scaffolding**: Workspace setup, Soroban configuration.
- [ ] **SP1 Integration**: Implement off-chain proof generation for basic compliance check.
- [ ] **On-Chain Verifier**: Implement Groth16/Plonk verifier in Soroban.
- [ ] **End-to-End Demo**: CLI flow for generating a proof and submitting it to a local network.
- [ ] **Testnet Deployment**: Launch on Stellar Futurenet.

## 📄 License
MIT
