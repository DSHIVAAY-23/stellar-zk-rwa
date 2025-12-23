# The Crash: Why Raw ZK Fails on Stellar (And How We Fixed It)

## 1. Problem Summary

Raw ZK verification (pairing-based proofs) requires on the order of ~300M CPU instructions per proof. Soroban transaction CPU limits are much lower (~100M), so running that verification inside a contract will exceed the budget and abort the transaction.

## 2. Architecture (Nightclub Bouncer vs Background Check Agency)

- Nightclub (Soroban contract): Cheap, fast checks performed at the door.
- Background Check Agency (Off-chain Gateway + SP1 Prover): Performs heavy ZK verification off-chain and issues a signed "ticket" the bouncer can quickly validate.

Detailed 4-step flow implemented in this repo:

1. Secret Input (Local): the user supplies private data (PDFs/records) locally. This never goes on-chain.
2. Heavy ZK Check (Off-chain): the `SP1` prover computes/validates the ZK proof off-chain.
3. Gateway Stamp (Off-chain): the Gateway, holding an Ed25519 authority key, signs a small payload: "User [X] is verified until [expiry]".
4. Cheap Settlement (On-chain): the Soroban contract verifies the Ed25519 signature using `env.crypto().ed25519_verify` and settles (mints) if valid.

## 3. How this repo maps to the design

- Gateway / signer script: [sp1-prover/script/src/main.rs](sp1-prover/script/src/main.rs) — generates gateway keys and sketches the signing flow.
- On-chain verifier contract: [contracts/zk_verifier/src/lib.rs](contracts/zk_verifier/src/lib.rs) — stores the gateway pubkey and calls `env.crypto().ed25519_verify` on a reconstructed payload.
- Contract tests & gas measurement: [contracts/zk_verifier/tests/gas_calibration.rs](contracts/zk_verifier/tests/gas_calibration.rs) — measures the gatekeeping cost.

Key implementation notes found in the code:

- `initialize(env, gateway_key: BytesN<32>)` stores the 32-byte Ed25519 gateway public key in contract storage.
- `verify_and_mint(...)` reconstructs the payload by XDR-encoding `user`, `token_id`, and `expiry`, then calls `env.crypto().ed25519_verify(&gateway_key, &payload, &signature)`.

## 4. Security & Privacy considerations

- Private data never leaves the user's environment or the SP1 prover host unless the user chooses to upload it. Only the off-chain prover reads raw documents.
- The signature is a deterministic attestation of the prover's check; the contract trusts the gateway key stored in its instance storage.
- Replay/expiry: the contract checks `expiry` against `env.ledger().timestamp()` for simple expiration-based replay protection. For stronger freshness guarantee, include a nonce or claim-id and track consumed claims in storage.
- Key rotation: to rotate the Gateway key, add a privileged `set_gateway_key` entrypoint guarded by an admin address or multisig.

## 5. Reproduction / verification steps (what I ran)

To compile & run the contract tests I executed:

```bash
cargo test -p zk_verifier --manifest-path contracts/zk_verifier/Cargo.toml
```

Result: tests compiled and passed locally (including the `gas_calibration` test). This confirms the contract builds and the `ed25519_verify` code path is exercised in tests.

## 6. Recommendations for grant write-up

- Explain the problem (ZK verify cost) and present the Gateway pattern as a pragmatic, auditable intermediate solution.
- Include the above flow diagram and point reviewers to the code references:
	- Gateway signer script: [sp1-prover/script/src/main.rs](sp1-prover/script/src/main.rs)
	- On-chain verifier: [contracts/zk_verifier/src/lib.rs](contracts/zk_verifier/src/lib.rs)
	- Gas test: [contracts/zk_verifier/tests/gas_calibration.rs](contracts/zk_verifier/tests/gas_calibration.rs)
- Add a short section on key-management, rotation, and optional decentralization (e.g., threshold signatures or multiple independent gateways) to reduce single-authority risk.

## 7. Next steps I can take (choose any)

- Implement payload XDR construction in `sp1-prover/script` so signed messages match the contract's `to_xdr` packing.
- Add a `set_gateway_key` admin function and a small rotation test.
- Add an end-to-end test: generate key, create payload, sign it with the script, and call `verify_and_mint` in the test harness to ensure full interoperability.

---

If you want, I can now implement one of the next steps above (for example, make `sp1-prover/script` produce the exact XDR payload the contract expects and add an end-to-end test).
