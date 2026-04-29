/// Pool State & Logic
///
/// Manages the pool's storage, reserve tracking, and orchestrates
/// calls to math.rs and token.rs.
///
/// ## Storage keys:
/// - `RESERVE_A`  → i128 (current reserve of token_a)
/// - `RESERVE_B`  → i128 (current reserve of token_b)
/// - `TOKEN_A`    → Address
/// - `TOKEN_B`    → Address
/// - `LP_TOKEN`   → Address (the LP token contract)
use soroban_sdk::{Address, Env};

use crate::math;
use crate::token;

// ---------------------------------------------------------------------------
// Storage key symbols — keep them short (Soroban charges per byte)
// ---------------------------------------------------------------------------
const RESERVE_A: &str = "RA";
const RESERVE_B: &str = "RB";
const TOKEN_A: &str = "TA";
const TOKEN_B: &str = "TB";
const LP_TOKEN: &str = "LP";

// ---------------------------------------------------------------------------
// Init
// ---------------------------------------------------------------------------

pub fn initialize(env: &Env, token_a: Address, token_b: Address, lp_token: Address) {
    // TODO: assert not already initialized
    // TODO: store TOKEN_A, TOKEN_B, LP_TOKEN
    // TODO: set RESERVE_A = 0, RESERVE_B = 0
    todo!("implement initialize")
}

// ---------------------------------------------------------------------------
// Liquidity
// ---------------------------------------------------------------------------

pub fn add_liquidity(env: &Env, to: Address, amount_a: i128, amount_b: i128) -> i128 {
    // TODO:
    // 1. require auth from `to`
    // 2. read reserves
    // 3. transfer amount_a of token_a and amount_b of token_b from `to` → contract
    // 4. calculate LP tokens to mint via math::calc_lp_tokens_to_mint
    // 5. mint LP tokens to `to` via token::mint
    // 6. update reserves
    // 7. return lp_minted
    todo!("implement add_liquidity")
}

pub fn remove_liquidity(env: &Env, to: Address, lp_amount: i128) -> (i128, i128) {
    // TODO:
    // 1. require auth from `to`
    // 2. read reserves and LP total supply
    // 3. calculate proportional share: amount_a = lp_amount / lp_supply * reserve_a
    // 4. burn LP tokens from `to`
    // 5. transfer amount_a and amount_b back to `to`
    // 6. update reserves
    // 7. return (amount_a, amount_b)
    todo!("implement remove_liquidity")
}

// ---------------------------------------------------------------------------
// Swap
// ---------------------------------------------------------------------------

pub fn swap(
    env: &Env,
    to: Address,
    token_in: Address,
    amount_in: i128,
    min_amount_out: i128,
) -> i128 {
    // TODO:
    // 1. require auth from `to`
    // 2. determine which token is in/out and fetch corresponding reserves
    // 3. calculate amount_out via math::get_amount_out
    // 4. assert amount_out >= min_amount_out (slippage check)
    // 5. transfer token_in from `to` → contract
    // 6. transfer token_out from contract → `to`
    // 7. update reserves
    // 8. return amount_out
    todo!("implement swap")
}

// ---------------------------------------------------------------------------
// Read-only helpers
// ---------------------------------------------------------------------------

pub fn get_reserves(env: &Env) -> (i128, i128) {
    // TODO: read RESERVE_A and RESERVE_B from storage
    todo!("implement get_reserves")
}

pub fn get_quote(env: &Env, token_in: Address, amount_in: i128) -> i128 {
    // TODO: read reserves, call math::get_amount_out, return result
    todo!("implement get_quote")
}
