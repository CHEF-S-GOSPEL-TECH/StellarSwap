#![no_std]
#![allow(unused_variables, dead_code)]

/// Factory Contract — Pair Registry Boundary
///
/// This contract is the future place for creating and tracking pair contracts.
/// It is intentionally scaffolded for now so contributors can implement the
/// factory layer without overloading the pair contract.
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env};

#[derive(Clone)]
#[contracttype]
enum DataKey {
    Admin,
}

#[contract]
pub struct FactoryContract;

#[contractimpl]
impl FactoryContract {
    /// Initialize the factory admin.
    pub fn initialize(env: Env, admin: Address) {
        let storage = env.storage().instance();

        assert!(
            !storage.has(&DataKey::Admin),
            "factory has already been initialized"
        );

        storage.set(&DataKey::Admin, &admin);
    }

    /// Returns the factory admin address.
    pub fn admin(env: Env) -> Address {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .expect("factory has not been initialized")
    }

    /// Future issue: deploy/register a pair for token_a/token_b.
    /// Tokens are sorted by address so (A,B) and (B,A) always resolve to the same pair.
    pub fn create_pair(env: Env, token_a: Address, token_b: Address) -> Address {
        assert!(token_a != token_b, "identical token addresses");
        // Sort so there is exactly one canonical pair per token combination.
        let (_token0, _token1) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };
        todo!("implement factory pair deployment")
    }

    /// Future issue: return the pair for token_a/token_b if it exists.
    /// Tokens are sorted before lookup so (A,B) and (B,A) return the same result.
    pub fn get_pair(env: Env, token_a: Address, token_b: Address) -> Option<Address> {
        let (_token0, _token1) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };
        todo!("implement factory pair lookup")
    }
}
