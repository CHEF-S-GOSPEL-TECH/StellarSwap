#![no_std]
#![allow(unused_variables, dead_code, unused_imports)]

/// AMM Contract — Entry Point
///
/// This is the main Soroban smart contract for the DEX.
/// It wires together pool logic, math, and LP token management.
///
/// ## What this contract does:
/// - Accepts two tokens (e.g. XLM and a SEP-41 token) as a trading pair
/// - Maintains reserves of both tokens
/// - Allows users to:
///     1. `add_liquidity`    — deposit both tokens, receive LP tokens
///     2. `remove_liquidity` — burn LP tokens, receive back both tokens
///     3. `swap`             — trade one token for the other
/// - Uses a constant-product curve: reserve_a * reserve_b = k
mod math;
mod pool;
mod token;

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct AmmContract;

#[contractimpl]
impl AmmContract {
    /// Initialize the pool with two token addresses and the LP token address.
    /// Must be called once before any other function.
    pub fn initialize(env: Env, token_a: Address, token_b: Address, lp_token: Address) {
        pool::initialize(&env, token_a, token_b, lp_token);
    }

    /// Deposit `amount_a` of token_a and `amount_b` of token_b into the pool.
    /// Mints LP tokens to `to` representing their share of the pool.
    pub fn add_liquidity(env: Env, to: Address, amount_a: i128, amount_b: i128) -> i128 {
        pool::add_liquidity(&env, to, amount_a, amount_b)
    }

    /// Burn `lp_amount` LP tokens and return the proportional share of both reserves to `to`.
    pub fn remove_liquidity(env: Env, to: Address, lp_amount: i128) -> (i128, i128) {
        pool::remove_liquidity(&env, to, lp_amount)
    }

    /// Swap `amount_in` of `token_in` for as much `token_out` as the curve allows.
    /// Applies a 0.3% fee (kept in the pool, accruing to LPs).
    pub fn swap(
        env: Env,
        to: Address,
        token_in: Address,
        amount_in: i128,
        min_amount_out: i128, // slippage protection
    ) -> i128 {
        pool::swap(&env, to, token_in, amount_in, min_amount_out)
    }

    /// Returns current reserves of (token_a, token_b).
    pub fn get_reserves(env: Env) -> (i128, i128) {
        pool::get_reserves(&env)
    }

    /// Returns a price quote: how much token_out you'd get for `amount_in` of token_in.
    pub fn get_quote(env: Env, token_in: Address, amount_in: i128) -> i128 {
        pool::get_quote(&env, token_in, amount_in)
    }
}
