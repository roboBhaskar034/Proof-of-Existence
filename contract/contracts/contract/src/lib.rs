#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, BytesN, Map};

#[contract]
pub struct ProofOfExistence;

#[contractimpl]
impl ProofOfExistence {

    // Store hash -> timestamp
    pub fn store_proof(env: Env, hash: BytesN<32>) -> u64 {
        let key = Symbol::short("proofs");

        let mut proofs: Map<BytesN<32>, u64> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        // Check if already exists
        if proofs.contains_key(hash.clone()) {
            panic!("Proof already exists");
        }

        let timestamp = env.ledger().timestamp();

        proofs.set(hash.clone(), timestamp);
        env.storage().instance().set(&key, &proofs);

        timestamp
    }

    // Verify proof
    pub fn verify_proof(env: Env, hash: BytesN<32>) -> (bool, u64) {
        let key = Symbol::short("proofs");

        let proofs: Map<BytesN<32>, u64> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        match proofs.get(hash) {
            Some(time) => (true, time),
            None => (false, 0),
        }
    }
}