#![no_std]
#![allow(unused_variables, dead_code, unused_imports)]

/// LP Token Contract
///
/// A standard SEP-41 fungible token that represents a liquidity provider's
/// share of the AMM pool reserves.
///
/// ## Rules:
/// - Only the AMM contract (set as `admin` during initialization) can mint or burn.
/// - Any holder can transfer their LP tokens freely.
/// - Burning LP tokens is how liquidity is removed from the pool.
///
/// ## Why a separate contract?
/// Keeping the LP token as its own contract means wallets, explorers, and
/// other DeFi protocols can recognize and display it as a standard token.
///
/// ## Implementation note:
/// Rather than writing a token from scratch, this contract wraps or extends
/// the Soroban example token. See:
/// https://github.com/stellar/soroban-examples/tree/main/token
use soroban_sdk::{contract, contractimpl, Address, Env, String};

#[contract]
pub struct LpToken;

#[contractimpl]
impl LpToken {
    /// Initialize the LP token.
    /// `admin` should be the AMM contract address.
    pub fn initialize(env: Env, admin: Address, name: String, symbol: String) {
        // TODO: store admin, name, symbol, set total_supply = 0
        todo!("implement initialize")
    }

    // --- SEP-41 interface ---

    pub fn name(env: Env) -> String {
        todo!()
    }

    pub fn symbol(env: Env) -> String {
        todo!()
    }

    pub fn decimals(_env: Env) -> u32 {
        7 // Stellar standard
    }

    pub fn total_supply(env: Env) -> i128 {
        todo!()
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        todo!()
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        // TODO: require auth from `from`, update balances
        todo!()
    }

    pub fn transfer_from(env: Env, spender: Address, from: Address, to: Address, amount: i128) {
        // TODO: require auth from `spender`, check allowance, update balances
        todo!()
    }

    pub fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        expiration_ledger: u32,
    ) {
        // TODO: require auth from `from`, store allowance
        todo!()
    }

    pub fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        todo!()
    }

    // --- Admin-only (AMM contract calls these) ---

    pub fn mint(env: Env, to: Address, amount: i128) {
        // TODO: require auth from admin, increase balance and total_supply
        todo!()
    }

    pub fn burn(env: Env, from: Address, amount: i128) {
        // TODO: require auth from `from`, decrease balance and total_supply
        todo!()
    }
}
