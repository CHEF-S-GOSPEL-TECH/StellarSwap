#![no_std]
#![allow(unused_variables, dead_code, unused_imports)]

/// Pair Contract — Entry Point
///
/// This is the Soroban pair contract for one token pair.
/// It wires together reserve state, constant-product math, and LP token calls.
///
/// ## What this contract does:
/// - Accepts two SEP-41-compatible tokens as a trading pair
/// - Maintains reserves of both tokens
/// - Allows users to:
///     1. `add_liquidity`    — deposit both tokens, receive LP tokens
///     2. `remove_liquidity` — burn LP tokens, receive back both tokens
///     3. `swap`             — trade one token for the other
/// - Uses a constant-product curve: reserve_a * reserve_b = k
pub mod math;
mod pair;
mod token;

use soroban_sdk::{contract, contractimpl, Address, Env};

#[contract]
pub struct PairContract;

#[contractimpl]
impl PairContract {
    /// Initialize the pair with two token addresses and the LP token address.
    /// Must be called once before any other function.
    pub fn initialize(env: Env, token_a: Address, token_b: Address, lp_token: Address) {
        pair::initialize(&env, token_a, token_b, lp_token);
    }

    /// Deposit tokens into the pair and receive LP tokens.
    /// Caller must transfer token_a and token_b to this contract BEFORE calling.
    /// Deposited amounts are derived from balance deltas — not trusted from caller.
    /// Mints LP tokens to `to` representing their share of the pair.
    pub fn add_liquidity(env: Env, to: Address) -> i128 {
        pair::add_liquidity(&env, to)
    }

    /// Burn LP tokens and receive back proportional reserves.
    /// Caller must transfer LP tokens to this contract BEFORE calling.
    pub fn remove_liquidity(env: Env, to: Address) -> (i128, i128) {
        pair::remove_liquidity(&env, to)
    }

    /// Swap token_in for token_out.
    /// Caller must transfer token_in to this contract BEFORE calling.
    /// Applies a 0.3% fee (kept in the pair, accruing to LPs).
    pub fn swap(env: Env, to: Address, token_in: Address, min_amount_out: i128) -> i128 {
        pair::swap(&env, to, token_in, min_amount_out)
    }

    /// Returns current reserves of (token_a, token_b).
    pub fn get_reserves(env: Env) -> (i128, i128) {
        pair::get_reserves(&env)
    }

    /// Returns a price quote: how much token_out you'd get for `amount_in` of token_in.
    /// Returns None if the pair has no liquidity.
    pub fn get_quote(env: Env, token_in: Address, amount_in: i128) -> Option<i128> {
        pair::get_quote(&env, token_in, amount_in)
    }
}
