/// Token Helpers
///
/// Thin wrappers around the Soroban SEP-41 token interface.
/// Used by pool.rs to transfer tokens and interact with the LP token contract.
///
/// We don't implement a token here — we call into external token contracts
/// using Soroban's cross-contract call mechanism.
use soroban_sdk::{Address, Env};

/// Transfer `amount` of `token` from `from` → `to`.
/// Requires that `from` has authorized this contract to spend on their behalf.
pub fn transfer(env: &Env, token: &Address, from: &Address, to: &Address, amount: i128) {
    // TODO: use soroban_sdk::token::Client to call token.transfer(from, to, amount)
    todo!("implement transfer")
}

/// Mint `amount` LP tokens to `to`.
/// Only callable by the AMM contract (which must be the LP token's admin).
pub fn mint(env: &Env, lp_token: &Address, to: &Address, amount: i128) {
    // TODO: use soroban_sdk::token::Client to call lp_token.mint(to, amount)
    todo!("implement mint")
}

/// Burn `amount` LP tokens from `from`.
pub fn burn(env: &Env, lp_token: &Address, from: &Address, amount: i128) {
    // TODO: call lp_token.burn(from, amount)
    todo!("implement burn")
}

/// Returns the total supply of the LP token.
pub fn total_supply(env: &Env, lp_token: &Address) -> i128 {
    // TODO: call lp_token.total_supply()
    todo!("implement total_supply")
}
