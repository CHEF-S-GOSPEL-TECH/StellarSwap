/// Pair State & Logic
///
/// Manages the pair's storage, reserve tracking, and orchestrates
/// calls to math.rs and token.rs.
///
/// ## Storage keys:
/// - `RESERVE_A`  → i128 (current reserve of token_a)
/// - `RESERVE_B`  → i128 (current reserve of token_b)
/// - `TOKEN_A`    → Address
/// - `TOKEN_B`    → Address
/// - `LP_TOKEN`   → Address (the LP token contract)
use soroban_sdk::{contracttype, Address, Env};

use crate::math;
use crate::token;

#[derive(Clone)]
#[contracttype]
enum DataKey {
    ReserveA,
    ReserveB,
    TokenA,
    TokenB,
    LpToken,
}

// ---------------------------------------------------------------------------
// Init
// ---------------------------------------------------------------------------

pub fn initialize(env: &Env, token_a: Address, token_b: Address, lp_token: Address) {
    let storage = env.storage().instance();

    assert!(
        !storage.has(&DataKey::TokenA),
        "pair has already been initialized"
    );
    assert!(token_a != token_b, "pair tokens must be different");

    storage.set(&DataKey::TokenA, &token_a);
    storage.set(&DataKey::TokenB, &token_b);
    storage.set(&DataKey::LpToken, &lp_token);
    storage.set(&DataKey::ReserveA, &0_i128);
    storage.set(&DataKey::ReserveB, &0_i128);
}

// ---------------------------------------------------------------------------
// Liquidity
// ---------------------------------------------------------------------------

/// Caller must transfer token_a and token_b to this contract before calling.
/// The deposited amounts are derived from balance deltas (v2 model — not trusted from caller).
pub fn add_liquidity(env: &Env, to: Address) -> i128 {
    // TODO:
    // 1. require auth from `to`
    // 2. read stored reserves (reserve_a, reserve_b)
    // 3. read actual balances of token_a and token_b held by this contract
    // 4. amount_a = balance_a - reserve_a  (delta = what was just sent in)
    // 5. amount_b = balance_b - reserve_b
    // 6. calculate LP tokens to mint via math::calc_lp_tokens_to_mint
    // 7. on first deposit, permanently lock math::MINIMUM_LIQUIDITY LP tokens (burn to zero address)
    // 8. mint LP tokens to `to` via token::mint
    // 9. update reserves to balance_a, balance_b
    // 10. return lp_minted
    todo!("implement add_liquidity")
}

/// Caller must transfer LP tokens to this contract before calling.
/// The LP amount is derived from the contract's LP token balance delta (v2 model).
pub fn remove_liquidity(env: &Env, to: Address) -> (i128, i128) {
    // TODO:
    // 1. require auth from `to`
    // 2. read lp_amount = LP token balance held by this contract (sent in by caller)
    // 3. read reserves and LP total supply
    // 4. amount_a = lp_amount * reserve_a / lp_supply
    // 5. amount_b = lp_amount * reserve_b / lp_supply
    // 6. burn the lp_amount held by this contract via token::burn
    // 7. transfer amount_a and amount_b to `to`
    // 8. update reserves
    // 9. return (amount_a, amount_b)
    todo!("implement remove_liquidity")
}

// ---------------------------------------------------------------------------
// Swap
// ---------------------------------------------------------------------------

/// Caller must transfer token_in to this contract before calling.
/// amount_in is derived from the balance delta (v2 model — not trusted from caller).
pub fn swap(env: &Env, to: Address, token_in: Address, min_amount_out: i128) -> i128 {
    to.require_auth();

    let storage = env.storage().instance();
    let token_a: Address = storage.get(&DataKey::TokenA).expect("not initialized");
    let token_b: Address = storage.get(&DataKey::TokenB).expect("not initialized");
    let (reserve_a, reserve_b) = get_reserves(env);

    let (reserve_in, reserve_out, token_out) = if token_in == token_a {
        (reserve_a, reserve_b, token_b.clone())
    } else if token_in == token_b {
        (reserve_b, reserve_a, token_a.clone())
    } else {
        panic!("token_in is not part of this pair")
    };

    let this = env.current_contract_address();
    let amount_in = token::balance(env, &token_in, &this) - reserve_in;
    let amount_out = math::get_amount_out(amount_in, reserve_in, reserve_out);

    assert!(
        amount_out >= min_amount_out,
        "slippage: amount_out below minimum"
    );

    token::transfer(env, &token_out, &this, &to, amount_out);

    let balance_a = token::balance(env, &token_a, &this);
    let balance_b = token::balance(env, &token_b, &this);

    let (amount_in_a, amount_in_b) = if token_in == token_a {
        (amount_in, 0_i128)
    } else {
        (0_i128, amount_in)
    };

    math::check_invariant(
        balance_a,
        balance_b,
        amount_in_a,
        amount_in_b,
        reserve_a,
        reserve_b,
    );

    storage.set(&DataKey::ReserveA, &balance_a);
    storage.set(&DataKey::ReserveB, &balance_b);

    amount_out
}

// ---------------------------------------------------------------------------
// Read-only helpers
// ---------------------------------------------------------------------------

pub fn get_reserves(env: &Env) -> (i128, i128) {
    let storage = env.storage().instance();

    (
        storage.get(&DataKey::ReserveA).unwrap_or(0_i128),
        storage.get(&DataKey::ReserveB).unwrap_or(0_i128),
    )
}

/// Returns how much token_out you'd receive for `amount_in` of `token_in`.
/// Returns None if the pair has no liquidity yet.
pub fn get_quote(env: &Env, token_in: Address, amount_in: i128) -> Option<i128> {
    let storage = env.storage().instance();
    let token_a: Address = storage
        .get(&DataKey::TokenA)
        .expect("pair has not been initialized");
    let token_b: Address = storage
        .get(&DataKey::TokenB)
        .expect("pair has not been initialized");
    let (reserve_a, reserve_b) = get_reserves(env);

    if token_in == token_a {
        if reserve_a == 0 || reserve_b == 0 {
            return None;
        }
        Some(math::get_amount_out(amount_in, reserve_a, reserve_b))
    } else if token_in == token_b {
        if reserve_a == 0 || reserve_b == 0 {
            return None;
        }
        Some(math::get_amount_out(amount_in, reserve_b, reserve_a))
    } else {
        panic!("token_in is not part of this pair")
    }
}
