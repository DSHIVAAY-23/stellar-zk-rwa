# The Crash: Why Raw ZK Fails on Stellar (And How We Fixed It)

## 1. The Issue: "The Math is Too Heavy"

**The Tech**: Zero-Knowledge Proofs (specifically Groth16) rely on a specific type of complex cryptography called **Elliptic Curve Pairings** (using the BN254 curve).

**The Cost**: To verify one single proof, a computer has to perform roughly **300 Million+ CPU instructions**.

**The Limit**: Stellar Soroban has a strict speed limit of **100 Million CPU instructions** per transaction to ensure the network stays fast and decentralized.

**The Result**: If you try to run the ZK verification code natively on-chain, the Soroban network stops your contract halfway through and forces a crash.

> **Error**: `HostError: Error(Budget, ExceededLimit)`
> *"Stop! You are using too much power. Transaction Failed."*

---

## 2. The Solution: "The Optimistic Gateway"

Since we can't run the heavy math on the chain (yet), we move the heavy verification **off-chain** but keep the trust secure using digital signatures. This is designated as the **Optimistic Gateway** pattern.

### Step 1: Off-Chain Verification
You run the heavy 300M+ instruction check on your own powerful server (or the user's laptop) using the **SP1 Prover**. This machine performs the complex elliptic curve pairings without worrying about blockchain gas limits.

### Step 2: The Signing
Once your server confirms the proof is valid, it uses a simple **Ed25519 Key** (which we call the "Gateway Authority") to sign a receipt.
> **Receipt**: *"I, the Gateway, checked the proof for User X, and it is valid."*

### Step 3: On-Chain Settlement
You send that signed receipt to the Soroban contract. The contract only has to verify the **Ed25519 signature**, which is incredibly cheap.

| Operation | CPU Cost | Status |
| :--- | :--- | :--- |
| **Rank ZK Verify** | ~300,000,000 | ❌ **CRASH** (Exceeds Limit) |
| **Gateway Verify** | ~450,000 | ✅ **SUCCESS** (0.45% of Limit) |

**Result**: We achieve privacy compliance on Stellar Mainnet *today*, with a path to decentralized verifiers in the future as limits increase or precompiles are added.
