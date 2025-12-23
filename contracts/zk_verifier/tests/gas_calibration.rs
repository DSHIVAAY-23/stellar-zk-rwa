#[cfg(test)]

use soroban_sdk::{Env, Bytes, BytesN, Address, testutils::Address as _, xdr::ToXdr};
use zk_verifier::{ZkVerifierContractClient}; 

use ed25519_dalek::{Signer, SigningKey};
use rand::rngs::OsRng;

#[test]
fn measure_gateway_gas() {
    let env = Env::default();
    
    // 1. Setup Gateway Keys (Off-chain)
    let mut csprng = OsRng;
    let signing_key = SigningKey::generate(&mut csprng);
    let verifying_key_bytes: [u8; 32] = signing_key.verifying_key().to_bytes();
    
    // 2. Deploy Contract & Initialize
    let contract_id = env.register_contract(None, zk_verifier::ZkVerifierContract);
    let client = ZkVerifierContractClient::new(&env, &contract_id);
    
    // Initialize with Gateway Public Key
    client.initialize(&BytesN::from_array(&env, &verifying_key_bytes));

    // 3. Prepare Payload
    let user = Address::generate(&env);
    let token_id: u64 = 101;
    let expiry: u64 = env.ledger().timestamp() + 3600; // 1 hour from now

    // Reconstruct Payload *Exactly* as Contract does
    // Note: In tests, `env` uses 0 as timestamp by default unless set.
    
    let mut payload = Bytes::new(&env);
    payload.append(&user.clone().to_xdr(&env));
    payload.append(&token_id.to_xdr(&env));
    payload.append(&expiry.to_xdr(&env));
    
    // Convert Soroban Bytes to Rust Vec<u8> for signing
    let mut payload_vec = vec![0u8; payload.len() as usize];
    payload.copy_into_slice(&mut payload_vec);
    
    // 4. Sign Payload
    let signature = signing_key.sign(&payload_vec);
    let signature_bytes = BytesN::from_array(&env, &signature.to_bytes());

    // 5. Measure Verification Gas
    env.cost_estimate().budget().reset_unlimited();

    let res = client.verify_and_mint(&user, &token_id, &expiry, &signature_bytes);
    
    // 6. Report
    std::println!("==================================================");
    std::println!("OPTIMISTIC GATEWAY VERIFICATION GAS USAGE:");
    env.cost_estimate().budget().print();
    std::println!("==================================================");
}
