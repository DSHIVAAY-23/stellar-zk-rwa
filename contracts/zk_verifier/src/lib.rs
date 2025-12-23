#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, BytesN, Env, Symbol, xdr::ToXdr};

extern crate alloc; 

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[contract]
pub struct ZkVerifierContract;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Gateway, // Address of the trusted gateway
}

pub trait VerifiableRwaTrait {
    fn verify_and_mint(env: Env, proof: Bytes, user: Address) -> Symbol;
}

#[contractimpl]
impl ZkVerifierContract {
    pub fn initialize(env: Env, gateway_key: BytesN<32>) {
        env.storage().instance().set(&DataKey::Gateway, &gateway_key);
    }

    pub fn verify_and_mint(
        env: Env, 
        user: Address, 
        token_id: u64, 
        expiry: u64, 
        signature: BytesN<64>
    ) -> Symbol {
        // 1. Replay Protection: Check if signature/auth is expired
        if env.ledger().timestamp() > expiry {
             panic!("Authorization expired");
        }

        // 2. Access Control: Retrieve Gateway public key
        let gateway_key: BytesN<32> = env.storage().instance().get(&DataKey::Gateway).expect("Gateway not set");

        // 3. Reconstruct Payload
        // Should match how the off-chain signer constructs it: [user, token_id, expiry]
        // Note: For simplicity, we assume the signature covers the raw bytes of these fields concatenated
        // or a constructed Bytes array. Soroban verify uses the raw payload.
        // Let's pack them into a Bytes buffer.
        
        let mut payload = Bytes::new(&env);
        payload.append(&user.to_xdr(&env));
        payload.append(&token_id.to_xdr(&env));
        payload.append(&expiry.to_xdr(&env));
        
        // 4. Verify Signature
        // 'gateway' is an Address. To use 'verify', we need to check signature type.
        // But handy way: user.require_auth() checks a signature for a TX invocation. 
        // Here we are verifying a DETACHED signature on a payload.
        // soroban_sdk::crypto::Crypto::ed25519_verify(pub_key, msg, sig)
        
        // We assume Gateway is an Ed25519 signer. 
        // We'll need the raw 32-byte public key from the Address if possible, 
        // OR we just use `env.crypto().ed25519_verify`.
        // However, extracting the Ed25519 pubkey from a Soroban Address (Account/Contract) inside the contract 
        // can be tricky if it's not stored explicitly as bytes.
        // CHANGE: Let's store Gateway as BytesN<32> (Ed25519 verify key) directly in storage for simplicity 
        // and gas efficiency, or pass it in. 
        // Wait, 'gateway' is stored. Let's assume we store the 32-byte key in DataKey::Gateway.
        
        // Let's refactor initialize to take BytesN<32>.
        // See updated logic below *inside* the function.
        
        env.crypto().ed25519_verify(&gateway_key, &payload, &signature);

        // 5. "Minting" (Mock)
        // If verify doesn't panic, it's valid.
        Symbol::new(&env, "verified")
    }
}

// Helper to ensure trait is satisfied if we still use it, 
// but for this refactor we might dropped the trait or updated it. 
// Let's keep the struct but maybe the trait method signature doesn't match effectively anymore.
// We will drop the trait impl for this custom flow or update the trait in a real scenario.
// For now, standalone impl is fine.


mod test;
